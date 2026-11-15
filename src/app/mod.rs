pub mod bootstrap;
pub mod download_handler;
pub mod handlers;
pub mod importer;
pub mod navigation;
pub mod router;

use dioxus::prelude::*;
use crate::db::models::DailyReadingStats;
use crate::db::Database;
use crate::engine::updater::{check_latest_release, ReleaseInfo};
use crate::state::AppState;
use bootstrap::bootstrap_library;
use importer::import_local_file;
use navigation::{handle_device_back, BACK_SCRIPT};
use router::AppRouter;

#[component]
pub fn App() -> Element {
    let (db, mut state) = use_hook(|| {
        let db = std::sync::Arc::new(match Database::init() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Database init failed: {}, using memory fallback", e);
                Database::memory().expect("In-memory database must succeed")
            }
        });

        let initial_books = bootstrap_library(&db);
        let initial_settings = db.get_settings().unwrap_or_default();
        let state = AppState::new(initial_books, initial_settings);
        (db, state)
    });

    let mut stats_data = use_signal(Vec::<DailyReadingStats>::new);
    let mut available_update = use_signal(|| None::<ReleaseInfo>);
    let mut show_update_modal = use_signal(|| false);
    let mut has_checked_update = use_signal(|| false);

    // Refresh stats when opening stats view
    let db_stats = db.clone();
    use_effect(move || {
        if *state.current_view.read() == crate::state::View::Stats {
            if let Ok(s) = db_stats.get_7_day_stats() { stats_data.set(s); }
        }
    });

    // Auto-check for updates on application startup
    use_effect(move || {
        if !*has_checked_update.read() {
            has_checked_update.set(true);
            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                if let Ok(Some(release)) = check_latest_release(env!("CARGO_PKG_VERSION")).await {
                    let tag = release.tag_name.clone();
                    available_update.set(Some(release));
                    show_update_modal.set(true);
                    *state.toast_message.write() = Some(format!("Pembaruan Baca {} tersedia!", tag));
                }
            });
        }
    });

    let current_theme = *state.theme.read();
    let settings = state.settings.read().clone();

    // Import Book intent
    let db_import = db.clone();
    let handle_import = move |_| {
        let db = db_import.clone();
        spawn(async move {
            #[cfg(not(target_os = "android"))]
            {
                let file = rfd::AsyncFileDialog::new()
                    .add_filter("E-Book (EPUB & PDF)", &["epub", "pdf"])
                    .set_title("Pilih berkas buku (EPUB atau PDF)")
                    .pick_file().await;

                if let Some(h) = file {
                    let path = h.path();
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    match import_local_file(&db, &name, None, Some(path)) {
                        Ok(book) => {
                            state.books.write().insert(0, book);
                            *state.toast_message.write() = Some("Buku berhasil ditambahkan".to_string());
                        }
                        Err(e) => *state.toast_message.write() = Some(e),
                    }
                }
            }
            #[cfg(target_os = "android")]
            {
                let _ = db;
                let _ = document::eval("let el = document.getElementById('baca-file-picker'); if (el) el.click();");
            }
        });
    };

    // Android SAF change listener
    let db_saf = db.clone();
    let handle_file_change = move |evt: FormEvent| {
        if let Some(file_engine) = evt.files() {
            for f in file_engine.files() {
                let (f_clone, fe_clone, db_clone) = (f.clone(), file_engine.clone(), db_saf.clone());
                spawn(async move {
                    if let Some(bytes) = fe_clone.read_file(&f_clone).await {
                        if let Ok(b) = import_local_file(&db_clone, &f_clone, Some(&bytes), None) {
                            state.books.write().insert(0, b);
                            *state.toast_message.write() = Some("Buku berhasil ditambahkan".to_string());
                        }
                    }
                });
            }
        }
    };

    let (db_del, state_del) = (db.clone(), state);
    let (db_ch, state_ch) = (db.clone(), state);
    let (db_set, state_set) = (db.clone(), state);
    let (db_dl, state_dl) = (db.clone(), state);
    let (db_exp, state_exp) = (db.clone(), state);
    let (db_res, state_res) = (db.clone(), state);

    rsx! {
        style { dangerous_inner_html: include_str!("../../assets/styles/bundle.css") }

        input { r#type: "file", id: "baca-file-picker", accept: ".epub,.pdf", style: "display: none !important;", onchange: handle_file_change }
        button { id: "baca-back-trigger", style: "display: none !important;", onclick: move |_| handle_device_back(state, show_update_modal) }
        script { dangerous_inner_html: "{BACK_SCRIPT}" }

        div {
            class: "app-root flex flex-col min-h-screen transition-colors duration-300 font-sans selection:bg-amber-200 relative overflow-x-hidden",
            style: "background-color: {current_theme.bg_color()}; color: {current_theme.text_primary()};",

            // Apple Progressive Blur Status Bar Shield (for non-reader views)
            if !matches!(&*state.current_view.read(), crate::state::View::Reader { .. }) {
                div { class: "apple-status-bar-shield", style: "background-color: {current_theme.bg_color()}30;" }
                div { class: "apple-progressive-blur", style: "background-color: {current_theme.bg_color()}60;" }
            }

            AppRouter {
                state: state,
                stats_data: stats_data,
                available_update: available_update,
                show_update_modal: show_update_modal,
                current_theme: current_theme,
                settings: settings,
                on_select_book: move |b| handlers::open_book_reader(state, b),
                on_delete_book: move |id| handlers::delete_book_from_shelf(db_del.clone(), state_del, id),
                on_import: handle_import,
                on_change_chapter: move |ch| handlers::update_reading_progress(db_ch.clone(), state_ch, ch),
                on_change_settings: move |s| handlers::save_user_settings(db_set.clone(), state_set, s),
                on_select_format: move |(b, fmt)| handlers::download_online_book(db_dl.clone(), state_dl, b, fmt),
                on_export_backup: move |_| handlers::export_library_backup(db_exp.clone(), state_exp),
                on_restore_backup: move |json| handlers::restore_library_backup(db_res.clone(), state_res, json),
            }
        }
    }
}
