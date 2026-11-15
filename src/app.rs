use dioxus::prelude::*;
use crate::components::{
    AiExplanationModal, AppleToast, FontControls, FormatModal, InBookSearchModal,
    TocDrawer, UpdateModal,
};
use crate::db::models::{BackupData, Book, DailyReadingStats, UserSettings};
use crate::db::Database;
use crate::engine::{
    download_archive_book, get_archive_format_file, parse_epub, ArchiveBook,
};
use crate::engine::updater::{check_latest_release, ReleaseInfo};
use crate::state::{AppState, View};
use crate::theme::ThemeMode;
use crate::views::{ExploreView, ReaderView, SettingsView, ShelfView, StatsView};

#[component]
pub fn App() -> Element {
    let (db, mut state) = use_hook(|| {
        let db = std::sync::Arc::new(match Database::init() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Database init failed, falling back to in-memory: {}", e);
                Database::memory().expect("In-memory database must succeed")
            }
        });

        let initial_books = {
            let mut existing = db.get_all_books().unwrap_or_default();
            let app_dir = Database::get_app_dir();
            let starters = crate::engine::ensure_starter_library(&app_dir);
            if existing.is_empty() {
                for b in &starters {
                    let _ = db.insert_or_update_book(b);
                }
                starters
            } else {
                for b in &mut existing {
                    if b.cover_image.is_none() {
                        if let Some(starter) = starters.iter().find(|s| {
                            s.id == b.id || s.title.to_lowercase() == b.title.to_lowercase()
                        }) {
                            b.cover_image = starter.cover_image.clone();
                            let _ = db.insert_or_update_book(b);
                        }
                    }
                }
                existing
            }
        };
        let initial_settings = db.get_settings().unwrap_or_default();
        let state = AppState::new(initial_books, initial_settings);
        (db, state)
    });

    let mut stats_data = use_signal(Vec::<DailyReadingStats>::new);
    let mut available_update = use_signal(|| None::<ReleaseInfo>);
    let mut show_update_modal = use_signal(|| false);
    let mut has_checked_update = use_signal(|| false);

    // Auto-check for updates on application startup (subtle background task)
    use_effect(move || {
        if !*has_checked_update.read() {
            has_checked_update.set(true);
            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                let current_ver = env!("CARGO_PKG_VERSION");
                if let Ok(Some(release)) = check_latest_release(current_ver).await {
                    let tag = release.tag_name.clone();
                    available_update.set(Some(release));
                    show_update_modal.set(true);
                    *state.toast_message.write() = Some(format!("Pembaruan Baca {} tersedia!", tag));
                }
            });
        }
    });

    // Refresh stats when opening stats view
    let db_stats = db.clone();
    use_effect(move || {
        if *state.current_view.read() == View::Stats {
            if let Ok(s) = db_stats.get_7_day_stats() {
                stats_data.set(s);
            }
        }
    });

    let current_view = state.current_view.read().clone();
    let current_theme = *state.theme.read();
    let settings = state.settings.read().clone();

    // 1. Local Book Import (Native Desktop Dialog + Android SAF Trigger)
    let db_import = db.clone();
    let import_book = {
        let db = db_import.clone();
        move || {
            let db = db.clone();
            spawn(async move {
                #[cfg(not(target_os = "android"))]
                {
                    use rfd::AsyncFileDialog;
                    let file = AsyncFileDialog::new()
                        .add_filter("E-Book (EPUB & PDF)", &["epub", "pdf"])
                        .set_title("Pilih berkas buku (EPUB atau PDF)")
                        .pick_file()
                        .await;

                    if let Some(file_handle) = file {
                        let path = file_handle.path();
                        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                        let app_dir = Database::get_app_dir();
                        let books_dir = app_dir.join("books");
                        let _ = std::fs::create_dir_all(&books_dir);
                        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                        let dest_path = books_dir.join(format!("{}_{}", uuid::Uuid::new_v4(), file_name));

                        let _ = std::fs::copy(path, &dest_path);
                        let now = chrono::Utc::now().timestamp();
                        let book_id = uuid::Uuid::new_v4().to_string();

                        if ext == "pdf" {
                            let new_book = Book {
                                id: book_id,
                                title: file_name.trim_end_matches(".pdf").replace('_', " "),
                                author: "Buku Lokal".to_string(),
                                file_path: dest_path.to_string_lossy().to_string(),
                                format: "pdf".to_string(),
                                cover_image: None,
                                total_chapters: 1,
                                current_chapter: 0,
                                progress_percent: 0.0,
                                last_read_at: now,
                                created_at: now,
                            };
                            if db.insert_or_update_book(&new_book).is_ok() {
                                let mut books = state.books.write();
                                books.insert(0, new_book);
                                *state.toast_message.write() = Some("Buku PDF berhasil ditambahkan ke rak".to_string());
                            }
                        } else if let Ok(parsed) = parse_epub(&dest_path) {
                            let new_book = Book {
                                id: book_id,
                                title: if !parsed.title.is_empty() { parsed.title } else { file_name.to_string() },
                                author: if !parsed.author.is_empty() { parsed.author } else { "Buku Lokal".to_string() },
                                file_path: dest_path.to_string_lossy().to_string(),
                                format: "epub".to_string(),
                                cover_image: parsed.cover_image,
                                total_chapters: parsed.chapters.len(),
                                current_chapter: 0,
                                progress_percent: 0.0,
                                last_read_at: now,
                                created_at: now,
                            };
                            if db.insert_or_update_book(&new_book).is_ok() {
                                let mut books = state.books.write();
                                books.insert(0, new_book);
                                *state.toast_message.write() = Some("Buku EPUB berhasil ditambahkan ke rak".to_string());
                            }
                        }
                    }
                }

                #[cfg(target_os = "android")]
                {
                    // Trigger webview hidden file input for SAF document picker
                    let _ = document::eval("let el = document.getElementById('baca-file-picker'); if (el) el.click();");
                }
            });
        }
    };

    // 2. Select Book to Read
    let handle_select_book = move |book: Book| {
        let path = book.file_path.clone();
        let book_clone = book.clone();
        spawn(async move {
            if book_clone.format.to_lowercase() == "pdf" {
                *state.current_chapter.write() = book_clone.current_chapter;
                *state.current_view.write() = View::Reader {
                    book: book_clone,
                    epub: None,
                };
                return;
            }

            let parsed_result = if let Some(starter) = crate::engine::get_starter_book_epub(&path) {
                Ok(starter)
            } else if let Some(starter) = crate::engine::get_starter_book_epub(&book_clone.title) {
                Ok(starter)
            } else {
                parse_epub(&path)
            };

            match parsed_result {
                Ok(parsed) => {
                    *state.current_chapter.write() = book_clone.current_chapter;
                    *state.current_view.write() = View::Reader {
                        book: book_clone,
                        epub: Some(parsed),
                    };
                }
                Err(e) => {
                    eprintln!("Error opening book: {}", e);
                    *state.toast_message.write() = Some(format!("Gagal membuka buku: {}", e));
                }
            }
        });
    };

    // 3. Delete Book
    let db_delete = db.clone();
    let handle_delete = move |book_id: String| {
        let db = db_delete.clone();
        let id_clone = book_id.clone();
        spawn(async move {
            if db.delete_book(&id_clone).is_ok() {
                let mut books = state.books.write();
                books.retain(|b| b.id != id_clone);
                *state.toast_message.write() = Some("Buku dihapus dari rak".to_string());
            }
        });
    };

    // 4. Update Reading Chapter
    let update_chapter = {
        let db = db.clone();
        move |new_chapter: usize| {
            *state.current_chapter.write() = new_chapter;
            if let View::Reader { book, epub } = state.current_view.read().clone() {
                let total = epub.as_ref().map(|e| e.chapters.len()).unwrap_or(1);
                let progress = if total > 0 { (new_chapter as f32) / (total as f32) } else { 0.0 };
                let book_id = book.id.clone();
                let db = db.clone();
                spawn(async move {
                    let _ = db.update_progress(&book_id, new_chapter, progress);
                    let _ = db.record_reading_session(&book_id, 30);
                });
            }
        }
    };

    // 5. Change Settings
    let change_settings = {
        let db = db.clone();
        move |new_settings: UserSettings| {
            *state.theme.write() = ThemeMode::from_str(&new_settings.theme);
            *state.settings.write() = new_settings.clone();
            let db = db.clone();
            spawn(async move {
                let _ = db.save_settings(&new_settings);
            });
        }
    };

    // 6. Download Online Book with Format Chooser
    let db_download = db.clone();
    let handle_select_format = move |(archive_book, format_str): (ArchiveBook, String)| {
        let db = db_download.clone();
        let id = archive_book.identifier.clone();
        let book_clone = archive_book.clone();
        let fmt = format_str.to_lowercase();

        *state.toast_message.write() = Some(format!("Mengunduh {} ({})...", book_clone.title, fmt.to_uppercase()));

        spawn(async move {
            match get_archive_format_file(&id, &fmt).await {
                Ok(Some(filename)) => {
                    let app_dir = Database::get_app_dir();
                    let books_dir = app_dir.join("books");
                    let _ = std::fs::create_dir_all(&books_dir);
                    let clean_filename = id.replace(|c: char| !c.is_alphanumeric(), "_");
                    let ext = if fmt == "pdf" { "pdf" } else { "epub" };
                    let dest_path = books_dir.join(format!("{}.{}", clean_filename, ext));

                    match download_archive_book(&id, &filename, &dest_path).await {
                        Ok(_) => {
                            let now = chrono::Utc::now().timestamp();
                            let new_book = if fmt == "pdf" {
                                Book {
                                    id: uuid::Uuid::new_v4().to_string(),
                                    title: book_clone.title.clone(),
                                    author: book_clone.author_display(),
                                    file_path: dest_path.to_string_lossy().to_string(),
                                    format: "pdf".to_string(),
                                    cover_image: Some(book_clone.thumbnail_url()),
                                    total_chapters: 1,
                                    current_chapter: 0,
                                    progress_percent: 0.0,
                                    last_read_at: now,
                                    created_at: now,
                                }
                            } else if let Ok(parsed) = parse_epub(&dest_path) {
                                Book {
                                    id: uuid::Uuid::new_v4().to_string(),
                                    title: if !parsed.title.is_empty() { parsed.title } else { book_clone.title.clone() },
                                    author: if !parsed.author.is_empty() { parsed.author } else { book_clone.author_display() },
                                    file_path: dest_path.to_string_lossy().to_string(),
                                    format: "epub".to_string(),
                                    cover_image: parsed.cover_image.or_else(|| Some(book_clone.thumbnail_url())),
                                    total_chapters: parsed.chapters.len(),
                                    current_chapter: 0,
                                    progress_percent: 0.0,
                                    last_read_at: now,
                                    created_at: now,
                                }
                            } else {
                                Book {
                                    id: uuid::Uuid::new_v4().to_string(),
                                    title: book_clone.title.clone(),
                                    author: book_clone.author_display(),
                                    file_path: dest_path.to_string_lossy().to_string(),
                                    format: "epub".to_string(),
                                    cover_image: Some(book_clone.thumbnail_url()),
                                    total_chapters: 1,
                                    current_chapter: 0,
                                    progress_percent: 0.0,
                                    last_read_at: now,
                                    created_at: now,
                                }
                            };

                            let _ = db.insert_or_update_book(&new_book);
                            let mut books = state.books.write();
                            books.insert(0, new_book);
                            *state.toast_message.write() = Some("Buku berhasil ditambahkan ke rak!".to_string());
                        }
                        Err(e) => {
                            *state.toast_message.write() = Some(format!("Gagal mengunduh: {}", e));
                        }
                    }
                }
                Ok(None) => {
                    *state.toast_message.write() = Some(format!("Format {} tidak tersedia untuk buku ini", fmt.to_uppercase()));
                }
                Err(e) => {
                    *state.toast_message.write() = Some(format!("Error koneksi: {}", e));
                }
            }
        });
    };

    // 7. Device Hardware Back-Stack Handler
    let mut handle_back_press = move || {
        // If update modal open, close it
        if *show_update_modal.read() {
            show_update_modal.set(false);
            return;
        }
        // If format modal open, close it
        if state.format_modal_book.read().is_some() {
            *state.format_modal_book.write() = None;
            return;
        }
        // If modals open in reader, close them
        if *state.is_typography_open.read() {
            *state.is_typography_open.write() = false;
            return;
        }
        if *state.is_toc_open.read() {
            *state.is_toc_open.write() = false;
            return;
        }
        if *state.is_inbook_search_open.read() {
            *state.is_inbook_search_open.write() = false;
            return;
        }
        if *state.is_ai_open.read() {
            *state.is_ai_open.write() = false;
            return;
        }

        // If not on shelf, navigate to shelf
        let view = state.current_view.read().clone();
        if view != View::Shelf {
            *state.current_view.write() = View::Shelf;
            return;
        }

        // On Shelf: double-tap back within 2 seconds to exit
        let now = chrono::Utc::now().timestamp_millis();
        let last = *state.last_back_press.read();
        if now - last < 2000 {
            std::process::exit(0);
        } else {
            *state.last_back_press.write() = now;
            *state.toast_message.write() = Some("Tekan sekali lagi untuk keluar".to_string());
        }
    };

    // 8. Backups
    let db_export = db.clone();
    let handle_export_backup = move |_| {
        if let Ok(data) = db_export.export_backup() {
            if let Ok(json_str) = serde_json::to_string_pretty(&data) {
                let backup_path = Database::get_app_dir().join("baca_backup.json");
                let _ = std::fs::write(&backup_path, json_str);
                *state.toast_message.write() = Some("Cadangan berhasil disimpan ke baca_backup.json".to_string());
            }
        }
    };

    let db_restore = db.clone();
    let handle_restore_backup = move |json_text: String| {
        let db = db_restore.clone();
        if let Ok(data) = serde_json::from_str::<BackupData>(&json_text) {
            let _ = db.restore_backup(data);
            if let Ok(books) = db.get_all_books() {
                *state.books.write() = books;
            }
            if let Ok(s) = db.get_settings() {
                *state.theme.write() = ThemeMode::from_str(&s.theme);
                *state.settings.write() = s;
            }
            *state.toast_message.write() = Some("Cadangan berhasil dipulihkan".to_string());
        } else {
            *state.toast_message.write() = Some("Format JSON cadangan tidak valid".to_string());
        }
    };

    let import_for_shelf = import_book.clone();
    let mut chapter_for_reader = update_chapter.clone();
    let mut chapter_for_toc = update_chapter.clone();
    let mut chapter_for_search = update_chapter.clone();

    let mut settings_for_font = change_settings.clone();
    let mut settings_for_ai = change_settings.clone();
    let mut settings_for_reader = change_settings.clone();

    let db_file_change = db.clone();
    let handle_file_change = move |evt: FormEvent| {
        if let Some(file_engine) = evt.files() {
            let files = file_engine.files();
            for file_name in files {
                let f_clone = file_name.clone();
                let fe_clone = file_engine.clone();
                let db_clone = db_file_change.clone();
                spawn(async move {
                    if let Some(bytes) = fe_clone.read_file(&f_clone).await {
                        let app_dir = Database::get_app_dir();
                        let books_dir = app_dir.join("books");
                        let _ = std::fs::create_dir_all(&books_dir);
                        let clean_id = uuid::Uuid::new_v4().to_string();
                        let is_pdf = f_clone.to_lowercase().ends_with(".pdf");
                        let ext = if is_pdf { "pdf" } else { "epub" };
                        let dest = books_dir.join(format!("{}.{}", clean_id, ext));
                        let _ = std::fs::write(&dest, &bytes);

                        let now = chrono::Utc::now().timestamp();
                        if is_pdf {
                            let new_book = Book {
                                id: clean_id,
                                title: f_clone.replace(".pdf", ""),
                                author: "Buku Lokal".to_string(),
                                file_path: dest.to_string_lossy().to_string(),
                                format: "pdf".to_string(),
                                cover_image: None,
                                total_chapters: 1,
                                current_chapter: 0,
                                progress_percent: 0.0,
                                last_read_at: now,
                                created_at: now,
                            };
                            let _ = db_clone.insert_or_update_book(&new_book);
                            let mut books = state.books.write();
                            books.insert(0, new_book);
                            *state.toast_message.write() = Some("Buku PDF berhasil ditambahkan".to_string());
                        } else if let Ok(parsed) = parse_epub(&dest) {
                            let new_book = Book {
                                id: clean_id,
                                title: if !parsed.title.is_empty() { parsed.title } else { f_clone.clone() },
                                author: if !parsed.author.is_empty() { parsed.author } else { "Buku Lokal".to_string() },
                                file_path: dest.to_string_lossy().to_string(),
                                format: "epub".to_string(),
                                cover_image: parsed.cover_image,
                                total_chapters: parsed.chapters.len(),
                                current_chapter: 0,
                                progress_percent: 0.0,
                                last_read_at: now,
                                created_at: now,
                            };
                            let _ = db_clone.insert_or_update_book(&new_book);
                            let mut books = state.books.write();
                            books.insert(0, new_book);
                            *state.toast_message.write() = Some("Buku EPUB berhasil ditambahkan".to_string());
                        }
                    }
                });
            }
        }
    };

    let back_script = r#"
        (function() {
            if (window.__baca_back_inited) return;
            window.__baca_back_inited = true;
            try {
                window.history.pushState({ page: 1 }, "");
                window.addEventListener("popstate", function(e) {
                    window.history.pushState({ page: 1 }, "");
                    var btn = document.getElementById("baca-back-trigger");
                    if (btn) btn.click();
                });
            } catch(err) {}
        })();
    "#;

    rsx! {
        style { dangerous_inner_html: include_str!("../assets/styles/bundle.css") }

        // Hidden input for Android SAF file picker
        input {
            r#type: "file",
            id: "baca-file-picker",
            accept: ".epub,.pdf",
            style: "display: none !important;",
            onchange: handle_file_change,
        }

        // Hidden button triggered by device hardware back (Android popstate)
        button {
            id: "baca-back-trigger",
            style: "display: none !important;",
            onclick: move |_| handle_back_press(),
        }

        script { dangerous_inner_html: "{back_script}" }

        div {
            class: "app-root flex flex-col min-h-screen transition-colors duration-300 font-sans selection:bg-amber-200",
            style: "background-color: {current_theme.bg_color()}; color: {current_theme.text_primary()};",

            match &current_view {
                View::Shelf => rsx! {
                    ShelfView {
                        books: state.books.read().clone(),
                        theme: current_theme,
                        search_query: state.search_query.read().clone(),
                        downloaded_ids: state.books.read().iter().map(|b| b.id.clone()).collect(),
                        has_update: available_update.read().is_some(),
                        on_search: move |q| *state.search_query.write() = q,
                        on_select_book: handle_select_book,
                        on_delete_book: handle_delete,
                        on_import: move |_| import_for_shelf(),
                        on_import_book: move |b: Book| {
                            let mut books = state.books.write();
                            books.insert(0, b);
                        },
                        on_explore: move |_| *state.current_view.write() = View::Explore,
                        on_stats: move |_| *state.current_view.write() = View::Stats,
                        on_settings: move |_| *state.current_view.write() = View::Settings,
                    }
                },
                View::Explore => rsx! {
                    ExploreView {
                        theme: current_theme,
                        downloaded_ids: state.books.read().iter().map(|b| b.id.clone()).collect(),
                        on_import_book: move |b: Book| {
                            let mut books = state.books.write();
                            books.insert(0, b);
                        },
                        on_back: move |_| *state.current_view.write() = View::Shelf,
                    }
                },
                View::Stats => rsx! {
                    StatsView {
                        stats: stats_data.read().clone(),
                        books: state.books.read().clone(),
                        theme: current_theme,
                        on_back: move |_| *state.current_view.write() = View::Shelf,
                    }
                },
                View::Settings => rsx! {
                    SettingsView {
                        settings: settings.clone(),
                        theme: current_theme,
                        on_change_settings: change_settings.clone(),
                        on_export_backup: handle_export_backup,
                        on_restore_backup: handle_restore_backup,
                        on_show_update_modal: move |rel| {
                            available_update.set(Some(rel));
                            show_update_modal.set(true);
                        },
                        on_back: move |_| *state.current_view.write() = View::Shelf,
                    }
                },
                View::Reader { book, epub } => rsx! {
                    ReaderView {
                        book: book.clone(),
                        epub: epub.clone(),
                        current_chapter: *state.current_chapter.read(),
                        theme: current_theme,
                        settings: settings.clone(),
                        on_change_chapter: move |ch| chapter_for_reader(ch),
                        on_back: move |_| *state.current_view.write() = View::Shelf,
                        on_toggle_typography: move |_| {
                            let cur = *state.is_typography_open.read();
                            *state.is_typography_open.write() = !cur;
                        },
                        on_toggle_toc: move |_| {
                            let cur = *state.is_toc_open.read();
                            *state.is_toc_open.write() = !cur;
                        },
                        on_toggle_search: move |_| *state.is_inbook_search_open.write() = true,
                        on_toggle_ai: move |_| *state.is_ai_open.write() = true,
                        on_change_theme: {
                            let settings = settings.clone();
                            move |theme_str: String| {
                                let mut s = settings.clone();
                                s.theme = theme_str;
                                settings_for_reader(s);
                            }
                        },
                    }

                    match epub.as_ref() {
                        Some(epub_data) => rsx! {
                            TocDrawer {
                                toc: epub_data.toc.clone(),
                                current_index: *state.current_chapter.read(),
                                theme: current_theme,
                                is_open: *state.is_toc_open.read(),
                                on_close: move |_| *state.is_toc_open.write() = false,
                                on_select: move |ch| chapter_for_toc(ch),
                            }

                            FontControls {
                                settings: settings.clone(),
                                theme: current_theme,
                                is_open: *state.is_typography_open.read(),
                                on_close: move |_| *state.is_typography_open.write() = false,
                                on_change_settings: move |s| settings_for_font(s),
                            }

                            InBookSearchModal {
                                chapters: epub_data.chapters.clone(),
                                theme: current_theme,
                                is_open: *state.is_inbook_search_open.read(),
                                on_close: move |_| *state.is_inbook_search_open.write() = false,
                                on_select_match: move |(ch_idx, _term)| {
                                    chapter_for_search(ch_idx);
                                },
                            }

                            AiExplanationModal {
                                theme: current_theme,
                                initial_text: state.ai_term.read().clone(),
                                api_key: settings.gemini_api_key.clone(),
                                model: settings.gemini_model.clone(),
                                is_open: *state.is_ai_open.read(),
                                on_close: move |_| *state.is_ai_open.write() = false,
                                on_save_key: {
                                    let settings = settings.clone();
                                    move |k: String| {
                                        let mut s = settings.clone();
                                        s.gemini_api_key = Some(k);
                                        settings_for_ai(s);
                                    }
                                },
                            }
                        },
                        None => rsx! {}
                    }
                },
            }

            // Global Format Chooser Bottom Sheet
            FormatModal {
                book: state.format_modal_book.read().clone(),
                theme: current_theme,
                on_select_format: handle_select_format,
                on_close: move |_| *state.format_modal_book.write() = None,
            }

            // Global Auto-Update Apple Spring Modal
            if *show_update_modal.read() {
                if let Some(rel) = available_update.read().clone() {
                    UpdateModal {
                        release: rel,
                        theme: current_theme,
                        on_close: move |_| show_update_modal.set(false),
                    }
                }
            }

            // Global Floating Apple Toast Banner
            AppleToast {
                message: state.toast_message.read().clone(),
                theme: current_theme,
                on_dismiss: move |_| *state.toast_message.write() = None,
            }
        }
    }
}
