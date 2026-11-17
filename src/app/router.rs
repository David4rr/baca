use dioxus::prelude::*;
use crate::components::*;
use crate::db::models::{Book, DailyReadingStats, UserSettings};
use crate::engine::updater::ReleaseInfo;
use crate::state::{AppState, View};
use crate::theme::ThemeMode;
use crate::views::{BookDetailView, ExploreView, ReaderView, SettingsView, ShelfView, StatsView};

#[component]
pub fn AppRouter(
    mut state: AppState,
    stats_data: Signal<Vec<DailyReadingStats>>,
    available_update: Signal<Option<ReleaseInfo>>,
    mut show_update_modal: Signal<bool>,
    current_theme: ThemeMode,
    settings: UserSettings,
    on_select_book: EventHandler<Book>,
    on_delete_book: EventHandler<String>,
    on_import: EventHandler<()>,
    on_change_chapter: EventHandler<usize>,
    on_progress: EventHandler<(usize, f32)>,
    on_pdf_progress: EventHandler<f32>,
    on_save_book: EventHandler<Book>,
    on_change_settings: EventHandler<UserSettings>,
    on_export_backup: EventHandler<()>,
    on_restore_backup: EventHandler<String>,
    on_toggle_bookmark: EventHandler<()>,
    on_delete_bookmark: EventHandler<String>,
    on_record_session: EventHandler<u32>,
    on_clear_highlight: EventHandler<()>,
) -> Element {
    let current_view = state.current_view.read().clone();

    rsx! {
        match &current_view {
            View::Shelf => rsx! {
                ShelfView {
                    books: state.books.read().clone(),
                    theme: current_theme,
                    search_query: state.search_query.read().clone(),
                    has_update: available_update.read().is_some(),
                    on_search: move |q| *state.search_query.write() = q,
                    on_select_book: move |b| on_select_book.call(b),
                    on_delete_book: move |id| {
                        if let Some(b) = state.books.read().iter().find(|b| b.id == id).cloned() {
                            *state.delete_confirm_book.write() = Some(b);
                        }
                    },
                    on_import: move |_| on_import.call(()),
                    on_import_book: move |b: Book| on_save_book.call(b),
                    on_select_detail: move |b| *state.current_view.write() = View::BookDetail { book: b },
                    on_explore: move |_| *state.current_view.write() = View::Explore,
                    on_stats: move |_| *state.current_view.write() = View::Stats,
                    on_settings: move |_| *state.current_view.write() = View::Settings,
                }
            },
            View::Explore => rsx! {
                ExploreView {
                    theme: current_theme,
                    library_books: state.books.read().clone(),
                    on_import_book: move |b: Book| on_save_book.call(b),
                    on_select_detail: move |b| *state.current_view.write() = View::BookDetail { book: b },
                    on_back: move |_| *state.current_view.write() = View::Shelf,
                }
            },
            View::BookDetail { book } => rsx! {
                BookDetailView {
                    book: book.clone(),
                    library_books: state.books.read().clone(),
                    theme: current_theme,
                    on_import_book: move |b: Book| on_save_book.call(b),
                    on_open_book: move |b| on_select_book.call(b),
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
                    on_change_settings: move |s| on_change_settings.call(s),
                    on_export_backup: move |_| on_export_backup.call(()),
                    on_restore_backup: move |json| on_restore_backup.call(json),
                    on_show_update_modal: move |rel: ReleaseInfo| {
                        available_update.set(Some(rel));
                        show_update_modal.set(true);
                    },
                    on_back: move |_| *state.current_view.write() = View::Shelf,
                }
            },
            View::Reader { book, epub } => {
                let epub_for_ai = epub.clone();
                rsx! {
                    ReaderView {
                        book: book.clone(),
                        epub: epub.clone(),
                        current_chapter: *state.current_chapter.read(),
                        theme: current_theme,
                        settings: settings.clone(),
                        is_bookmarked: state.bookmarks.read().iter().any(|b| {
                            b.book_id == book.id && b.chapter_index == *state.current_chapter.read()
                        }),
                        highlight_keyword: state.inbook_keyword.read().clone(),
                        on_change_chapter: move |ch| on_change_chapter.call(ch),
                        on_progress: move |(ch, frac)| on_progress.call((ch, frac)),
                        on_pdf_progress: move |p| on_pdf_progress.call(p),
                        on_back: move |_| *state.current_view.write() = View::Shelf,
                        on_toggle_typography: move |_| { let cur = *state.is_typography_open.read(); *state.is_typography_open.write() = !cur; },
                        on_toggle_toc: move |_| { let cur = *state.is_toc_open.read(); *state.is_toc_open.write() = !cur; },
                        on_toggle_search: move |_| *state.is_inbook_search_open.write() = true,
                        on_toggle_ai: move |_| {
                            let ch_idx = *state.current_chapter.read();
                            let title = epub_for_ai.as_ref().and_then(|e| e.chapters.get(ch_idx)).map(|c| c.title.clone()).unwrap_or_default();
                            *state.ai_term.write() = title;
                            *state.is_ai_open.write() = true;
                        },
                        on_toggle_bookmark: move |_| on_toggle_bookmark.call(()),
                        on_clear_highlight: move |_| on_clear_highlight.call(()),
                        on_record_session: move |sec| on_record_session.call(sec),
                        on_change_theme: {
                            let mut s = settings.clone();
                            move |theme_str: String| { s.theme = theme_str; on_change_settings.call(s.clone()); }
                        },
                    }

                    if let Some(epub_data) = epub.as_ref() {
                        TocDrawer {
                            toc: epub_data.toc.clone(),
                            bookmarks: state.bookmarks.read().clone(),
                            current_index: *state.current_chapter.read(),
                            theme: current_theme,
                            is_open: *state.is_toc_open.read(),
                            on_close: move |_| *state.is_toc_open.write() = false,
                            on_select: move |ch| on_change_chapter.call(ch),
                            on_delete_bookmark: move |bm_id| on_delete_bookmark.call(bm_id),
                        }
                        FontControls {
                            settings: settings.clone(),
                            theme: current_theme,
                            is_open: *state.is_typography_open.read(),
                            on_close: move |_| *state.is_typography_open.write() = false,
                            on_change_settings: move |s| on_change_settings.call(s),
                        }
                        InBookSearchModal {
                            epub: epub_data.clone(),
                            theme: current_theme,
                            is_open: *state.is_inbook_search_open.read(),
                            on_close: move |_| *state.is_inbook_search_open.write() = false,
                            on_select_match: move |(ch_idx, term)| { on_change_chapter.call(ch_idx); *state.inbook_keyword.write() = Some(term); },
                        }
                        AiExplanationModal {
                            theme: current_theme,
                            initial_text: state.ai_term.read().clone(),
                            api_key: settings.gemini_api_key.clone(),
                            model: settings.gemini_model.clone(),
                            is_open: *state.is_ai_open.read(),
                            on_close: move |_| *state.is_ai_open.write() = false,
                            on_save_key: {
                                let mut s = settings.clone();
                                move |k: String| { s.gemini_api_key = Some(k); on_change_settings.call(s.clone()); }
                            },
                        }
                    }
                }
            },
        }


        if let Some(book_to_delete) = state.delete_confirm_book.read().clone() {
            ConfirmDeleteModal {
                title: "Hapus Buku".to_string(),
                message: format!("Hapus \"{}\" dari rak bacaan Anda secara permanen?", book_to_delete.title),
                theme: current_theme,
                on_confirm: {
                    let id = book_to_delete.id.clone();
                    let on_del = on_delete_book;
                    move |_| {
                        *state.delete_confirm_book.write() = None;
                        on_del.call(id.clone());
                    }
                },
                on_cancel: move |_| *state.delete_confirm_book.write() = None,
            }
        }

        if *show_update_modal.read() {
            if let Some(rel) = available_update.read().clone() {
                UpdateModal {
                    release: rel,
                    theme: current_theme,
                    on_close: move |_| show_update_modal.set(false),
                }
            }
        }

        AppleToast {
            message: state.toast_message.read().clone(),
            theme: current_theme,
            on_dismiss: move |_| *state.toast_message.write() = None,
        }
    }
}
