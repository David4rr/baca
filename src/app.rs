use dioxus::prelude::*;
use crate::components::{FontControls, Navbar, TocDrawer};
use crate::db::models::{Book, UserSettings};
use crate::db::Database;
use crate::engine::parse_epub;
use crate::state::{AppState, View};
use crate::theme::ThemeMode;
use crate::views::{ReaderView, ShelfView};
use std::sync::Arc;

#[component]
pub fn App() -> Element {
    // Database initialization
    let db = use_hook(|| Arc::new(Database::init().expect("Failed to initialize database")));
    
    // Initial fetch from DB
    let initial_books = db.get_all_books().unwrap_or_default();
    let initial_settings = db.get_settings().unwrap_or_default();

    let mut state = use_hook(|| AppState::new(initial_books, initial_settings));

    let current_theme = *state.theme.read();
    let current_view = state.current_view.read().clone();
    let settings = state.settings.read().clone();

    // Import helper
    let import_book = {
        let db = db.clone();
        move || {
            let db = db.clone();
            spawn(async move {
                let file = rfd::AsyncFileDialog::new()
                    .add_filter("E-Books", &["epub"])
                    .set_title("Select EPUB Book")
                    .pick_file()
                    .await;

                if let Some(handle) = file {
                    let path = handle.path();
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
            if let Ok(parsed) = parse_epub(&path) {
                *state.current_chapter.write() = book_clone.current_chapter;
                *state.current_view.write() = View::Reader {
                    book: book_clone,
                    epub: parsed,
                };
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
                });
            }
        }
    };

    let db_settings = db.clone();
    let handle_change_settings = move |new_settings: UserSettings| {
        *state.theme.write() = ThemeMode::from_str(&new_settings.theme);
        *state.settings.write() = new_settings.clone();
        let db = db_settings.clone();
        spawn(async move {
            let _ = db.save_settings(&new_settings);
        });
    };

    let import_for_nav = import_book.clone();
    let import_for_shelf = import_book.clone();

    let mut chapter_for_reader = update_chapter.clone();
    let mut chapter_for_toc = update_chapter.clone();

    rsx! {
        div {
            class: "app-root flex flex-col min-h-screen transition-colors duration-300 font-sans",
            style: "background-color: {current_theme.bg_color()}; color: {current_theme.text_primary()};",

            Navbar {
                title: match &current_view {
                    View::Reader { book, .. } => book.title.clone(),
                    _ => "Baca.".to_string(),
                },
                theme: current_theme,
                is_reader: matches!(current_view, View::Reader { .. }),
                on_back: move |_| *state.current_view.write() = View::Shelf,
                on_import: move |_| import_for_nav(),
                on_toggle_toc: move |_| {
                    let cur = *state.is_toc_open.read();
                    *state.is_toc_open.write() = !cur;
                },
                on_toggle_typography: move |_| {
                    let cur = *state.is_typography_open.read();
                    *state.is_typography_open.write() = !cur;
                },
            }

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
                        on_change_settings: handle_change_settings,
                    }
                },
                View::Settings => rsx! {
                    div { class: "p-8", "Settings" }
                }
            }
        }
    }
}
