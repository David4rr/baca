pub mod canvas;
pub mod highlight;
pub mod pdf_canvas;
pub mod pdf_script;
pub mod scrubber;
pub mod top_bar;

use dioxus::prelude::*;
use crate::db::models::{Book, UserSettings};
use crate::engine::EpubBook;
use crate::theme::ThemeMode;
pub use canvas::ReaderCanvas;
pub use pdf_canvas::PdfCanvas;
pub use scrubber::ReaderScrubber;
pub use top_bar::{ReaderTopBar, ReadingMode};

#[component]
pub fn ReaderView(
    book: Book,
    epub: Option<EpubBook>,
    current_chapter: usize,
    theme: ThemeMode,
    settings: UserSettings,
    is_bookmarked: bool,
    highlight_keyword: Option<String>,
    on_change_chapter: EventHandler<usize>,
    on_back: EventHandler<()>,
    on_toggle_typography: EventHandler<()>,
    on_toggle_toc: EventHandler<()>,
    on_toggle_search: EventHandler<()>,
    on_toggle_ai: EventHandler<()>,
    on_toggle_bookmark: EventHandler<()>,
    on_clear_highlight: EventHandler<()>,
    on_record_session: EventHandler<u32>,
    on_change_theme: EventHandler<String>,
) -> Element {
    let mut reading_mode = use_signal(|| ReadingMode::Scroll);
    let mut session_start = use_signal(|| chrono::Utc::now().timestamp());

    // Flush session time on back
    let on_back_click = {
        let on_rec = on_record_session.clone();
        let on_b = on_back.clone();
        move |_| {
            let elapsed = (chrono::Utc::now().timestamp() - *session_start.read()).max(0) as u32;
            if elapsed >= 5 { on_rec.call(elapsed); }
            on_b.call(());
        }
    };

    // If book is a PDF or EPUB failed to parse, render PDF Canvas
    if book.format.to_lowercase() == "pdf" || epub.is_none() {
        return rsx! {
            div {
                class: "reader-screen flex-1 flex flex-col items-center relative overflow-y-auto min-h-screen transition-colors duration-300",
                style: "background-color: {theme.bg_color()}; color: {theme.text_primary()};",
                PdfCanvas {
                    book: book,
                    theme: theme,
                    on_back: on_back_click,
                }
            }
        };
    }

    let epub_book = epub.unwrap();
    let total_chapters = epub_book.chapters.len();
    let current_ch = epub_book.chapters.get(current_chapter).cloned();

    let progress_percent = if total_chapters > 0 {
        (((current_chapter as f32) / (total_chapters as f32)) * 100.0).round() as u32
    } else {
        0
    };

    let est_minutes_left = if total_chapters > 0 {
        ((total_chapters - current_chapter) * 4).max(1)
    } else {
        0
    };

    let font_family_css = match settings.font_family.as_str() {
        "sans" => "'Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif".to_string(),
        "mono" => "'JetBrains Mono', ui-monospace, SFMono-Regular, monospace".to_string(),
        _ => "'Newsreader', 'Playfair Display', Georgia, serif".to_string(),
    };

    let cur_mode = *reading_mode.read();

    rsx! {
        div {
            class: "reader-screen flex-1 flex flex-col items-center relative overflow-y-auto min-h-screen selection:bg-amber-200 transition-colors duration-300 animate-card-morph",
            style: "background-color: {theme.bg_color()}; color: {theme.text_primary()};",

            // 1. Floating Apple Top Navigation Bar
            ReaderTopBar {
                theme: theme,
                reading_mode: cur_mode,
                is_bookmarked: is_bookmarked,
                on_toggle_mode: move |_| {
                    if *reading_mode.read() == ReadingMode::Scroll {
                        reading_mode.set(ReadingMode::Paged);
                    } else {
                        reading_mode.set(ReadingMode::Scroll);
                    }
                },
                on_toggle_bookmark: move |_| on_toggle_bookmark.call(()),
                on_back: on_back_click,
                on_toggle_search: move |_| on_toggle_search.call(()),
                on_toggle_ai: move |_| on_toggle_ai.call(()),
                on_toggle_typography: move |_| on_toggle_typography.call(()),
                on_toggle_toc: move |_| on_toggle_toc.call(()),
            }

            // 2. Main Editorial Reading Canvas
            ReaderCanvas {
                current_ch: current_ch,
                current_chapter: current_chapter,
                total_chapters: total_chapters,
                font_family_css: font_family_css,
                font_size: settings.font_size,
                line_height: settings.line_height,
                theme: theme,
                reading_mode: cur_mode,
                highlight_keyword: highlight_keyword,
                on_change_chapter: {
                    let on_rec = on_record_session.clone();
                    let on_ch = on_change_chapter.clone();
                    move |ch| {
                        let elapsed = (chrono::Utc::now().timestamp() - *session_start.read()).max(0) as u32;
                        if elapsed >= 5 {
                            on_rec.call(elapsed);
                            session_start.set(chrono::Utc::now().timestamp());
                        }
                        on_ch.call(ch);
                    }
                },
                on_toggle_ai: move |_| on_toggle_ai.call(()),
                on_clear_highlight: move |_| on_clear_highlight.call(()),
            }

            // 3. Bottom Scrubber & Segmented Theme Dock
            ReaderScrubber {
                progress_percent: progress_percent,
                current_chapter: current_chapter,
                total_chapters: total_chapters,
                est_minutes_left: est_minutes_left,
                theme: theme,
                on_change_theme: move |t| on_change_theme.call(t),
            }
        }
    }
}
