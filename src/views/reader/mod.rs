pub mod canvas;
pub mod pdf_canvas;
pub mod scrubber;
pub mod top_bar;

use dioxus::prelude::*;
use crate::db::models::{Book, UserSettings};
use crate::engine::EpubBook;
use crate::theme::ThemeMode;
pub use canvas::ReaderCanvas;
pub use pdf_canvas::PdfCanvas;
pub use scrubber::ReaderScrubber;
pub use top_bar::ReaderTopBar;

#[component]
pub fn ReaderView(
    book: Book,
    epub: Option<EpubBook>,
    current_chapter: usize,
    theme: ThemeMode,
    settings: UserSettings,
    on_change_chapter: EventHandler<usize>,
    on_back: EventHandler<()>,
    on_toggle_typography: EventHandler<()>,
    on_toggle_toc: EventHandler<()>,
    on_toggle_search: EventHandler<()>,
    on_toggle_ai: EventHandler<()>,
    on_change_theme: EventHandler<String>,
) -> Element {
    // If book is a PDF or EPUB failed to parse, render PDF Canvas
    if book.format.to_lowercase() == "pdf" || epub.is_none() {
        return rsx! {
            div {
                class: "reader-screen flex-1 flex flex-col items-center relative overflow-y-auto min-h-screen transition-colors duration-300",
                style: "background-color: {theme.bg_color()}; color: {theme.text_primary()};",
                PdfCanvas {
                    book: book,
                    theme: theme,
                    on_back: move |_| on_back.call(()),
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

    rsx! {
        div {
            class: "reader-screen flex-1 flex flex-col items-center relative overflow-y-auto min-h-screen selection:bg-amber-200 transition-colors duration-300 animate-card-morph",
            style: "background-color: {theme.bg_color()}; color: {theme.text_primary()};",

            // 1. Floating Apple Top Navigation Bar
            ReaderTopBar {
                theme: theme,
                on_back: move |_| on_back.call(()),
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
                on_change_chapter: move |ch| on_change_chapter.call(ch),
                on_toggle_ai: move |_| on_toggle_ai.call(()),
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
