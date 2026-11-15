use dioxus::prelude::*;
use crate::components::{
    AiExplanationModal, BackupModal, FontControls, InBookSearchModal, StatsModal, TocDrawer,
};
use crate::db::models::{BackupData, Book, DailyReadingStats, UserSettings};
use crate::db::Database;
use crate::engine::parse_epub;
use crate::state::{AppState, View};
use crate::theme::ThemeMode;
use crate::views::{ExploreView, ReaderView, ShelfView};

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
            let existing = db.get_all_books().unwrap_or_default();
            if existing.is_empty() {
                let app_dir = Database::get_app_dir();
                let starters = crate::engine::ensure_starter_library(&app_dir);
                for b in &starters {
                    let _ = db.insert_or_update_book(b);
                }
                starters
            } else {
                existing
            }
        };
        let initial_settings = db.get_settings().unwrap_or_default();
        let state = AppState::new(initial_books, initial_settings);
        (db, state)
    });

    let mut stats_data = use_signal(Vec::<DailyReadingStats>::new);

    let current_view = state.current_view.read().clone();
    let current_theme = *state.theme.read();
    let settings = state.settings.read().clone();

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
                        .add_filter("E-Book (EPUB)", &["epub"])
                        .set_title("Pilih berkas EPUB")
                        .pick_file()
                        .await;

                    if let Some(file_handle) = file {
                        let path = file_handle.path();
                        if let Ok(parsed) = parse_epub(path) {
                            let now = chrono::Utc::now().timestamp();
                            let book_id = uuid::Uuid::new_v4().to_string();
                            let new_book = Book {
                                id: book_id,
                                title: parsed.title,
                                author: parsed.author,
                                file_path: path.to_string_lossy().to_string(),
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
                            }
                        }
                    }
                }

                #[cfg(target_os = "android")]
                {
                    let scan_dir = std::path::PathBuf::from("/sdcard/Download");
                    if let Ok(entries) = std::fs::read_dir(scan_dir) {
                        for entry in entries.flatten() {
                            let p = entry.path();
                            if p.extension().and_then(|s| s.to_str()) == Some("epub") {
                                if let Ok(parsed) = parse_epub(&p) {
                                    let now = chrono::Utc::now().timestamp();
                                    let book_id = uuid::Uuid::new_v4().to_string();
                                    let new_book = Book {
                                        id: book_id,
                                        title: parsed.title,
                                        author: parsed.author,
                                        file_path: p.to_string_lossy().to_string(),
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
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    };

    let db_delete = db.clone();
    let handle_delete = move |book_id: String| {
        let db = db_delete.clone();
        let id_clone = book_id.clone();
        spawn(async move {
            if db.delete_book(&id_clone).is_ok() {
                let mut books = state.books.write();
                books.retain(|b| b.id != id_clone);
            }
        });
    };

    let handle_select_book = move |book: Book| {
        let path = book.file_path.clone();
        let book_clone = book.clone();
        spawn(async move {
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
                        epub: parsed,
                    };
                }
                Err(e) => {
                    eprintln!("Error opening book: {}", e);
                }
            }
        });
    };

    let update_chapter = {
        let db = db.clone();
        move |new_chapter: usize| {
            *state.current_chapter.write() = new_chapter;
            if let View::Reader { book, epub } = state.current_view.read().clone() {
                let total = epub.chapters.len();
                let progress = if total > 0 { (new_chapter as f32) / (total as f32) } else { 0.0 };
                let book_id = book.id.clone();
                let db = db.clone();
                spawn(async move {
                    let _ = db.update_progress(&book_id, new_chapter, progress);
                    // Record reading session increment (e.g. 30 seconds reading)
                    let _ = db.record_reading_session(&book_id, 30);
                });
            }
        }
    };

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

    let mut settings_for_font = change_settings.clone();
    let mut settings_for_ai = change_settings.clone();
    let mut settings_for_backup = change_settings.clone();
    let mut settings_for_reader = change_settings.clone();

    let db_export = db.clone();
    let handle_export_backup = move |_| {
        if let Ok(data) = db_export.export_backup() {
            if let Ok(json_str) = serde_json::to_string_pretty(&data) {
                let backup_path = Database::get_app_dir().join("baca_backup.json");
                let _ = std::fs::write(&backup_path, json_str);
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
        }
    };

    let db_stats = db.clone();
    let handle_open_stats = move |_| {
        if let Ok(s) = db_stats.get_7_day_stats() {
            stats_data.set(s);
        }
        *state.is_stats_open.write() = true;
    };

    let import_for_shelf = import_book.clone();

    let mut chapter_for_reader = update_chapter.clone();
    let mut chapter_for_toc = update_chapter.clone();
    let mut chapter_for_search = update_chapter.clone();

    rsx! {
        style { dangerous_inner_html: include_str!("../assets/styles/bundle.css") }
        div {
            class: "app-root flex flex-col min-h-screen transition-colors duration-300 font-sans selection:bg-amber-200",
            style: "background-color: {current_theme.bg_color()}; color: {current_theme.text_primary()};",

            match &current_view {
                View::Shelf => rsx! {
                    ShelfView {
                        books: state.books.read().clone(),
                        theme: current_theme,
                        search_query: state.search_query.read().clone(),
                        on_search: move |q| *state.search_query.write() = q,
                        on_select_book: handle_select_book,
                        on_delete_book: handle_delete,
                        on_import: move |_| import_for_shelf(),
                        on_explore: move |_| *state.current_view.write() = View::Explore,
                        on_stats: handle_open_stats,
                        on_settings: move |_| *state.is_backup_open.write() = true,
                    }
                },
                View::Explore => rsx! {
                    ExploreView {
                        theme: current_theme,
                        on_import_book: move |b: Book| {
                            let mut books = state.books.write();
                            books.insert(0, b);
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

                    TocDrawer {
                        toc: epub.toc.clone(),
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
                        chapters: epub.chapters.clone(),
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
                View::Settings => rsx! {
                    div { class: "p-8", "Settings" }
                }
            }

            // Global Statistics Modal
            StatsModal {
                theme: current_theme,
                stats: stats_data.read().clone(),
                total_books: state.books.read().len(),
                total_bookmarks: 0,
                is_open: *state.is_stats_open.read(),
                on_close: move |_| *state.is_stats_open.write() = false,
            }

            // Global Settings & Backup Modal
            BackupModal {
                theme: current_theme,
                settings: settings.clone(),
                is_open: *state.is_backup_open.read(),
                on_close: move |_| *state.is_backup_open.write() = false,
                on_export_json: handle_export_backup,
                on_restore_json: handle_restore_backup,
                on_save_settings: move |s| settings_for_backup(s),
            }
        }
    }
}
