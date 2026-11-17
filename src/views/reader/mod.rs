pub mod canvas;
pub mod highlight;
pub mod pdf_canvas;
pub mod pdf_script;
pub mod scrubber;
pub mod top_bar;

use dioxus::prelude::*;
use std::sync::Arc;
use std::time::Duration;

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
    epub: Option<Arc<EpubBook>>,
    current_chapter: usize,
    theme: ThemeMode,
    settings: UserSettings,
    is_bookmarked: bool,
    highlight_keyword: Option<String>,
    on_change_chapter: EventHandler<usize>,
    on_progress: EventHandler<(usize, f32)>,
    on_pdf_progress: EventHandler<f32>,
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
    let mut is_fullscreen = use_signal(|| false);
    let mut session_start = use_signal(|| chrono::Utc::now().timestamp());
    let mut fraction = use_signal(|| 0.0f32);

    let on_back_click = {
        let on_rec = on_record_session;
        let on_b = on_back;
        move |_| {
            let elapsed = (chrono::Utc::now().timestamp() - *session_start.peek()).max(0) as u32;
            if elapsed >= 5 {
                on_rec.call(elapsed);
            }
            on_b.call(());
        }
    };

    let toggle_fullscreen = move |_| {
        let next = !*is_fullscreen.peek();
        is_fullscreen.set(next);
        spawn(async move {
            let _ = document::eval(
                "(() => {
                    try {
                        if (!document.fullscreenElement) {
                            if (document.documentElement.requestFullscreen) {
                                document.documentElement.requestFullscreen().catch(() => {});
                            }
                        } else {
                            if (document.exitFullscreen) {
                                document.exitFullscreen().catch(() => {});
                            }
                        }
                    } catch (_) {}
                })();",
            );
        });
    };

    // Persist session time periodically so backgrounding/killing the app keeps
    // most of the reading time instead of losing the whole session.
    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_secs(20)).await;
            let now = chrono::Utc::now().timestamp();
            let elapsed = (now - *session_start.peek()).max(0) as u32;
            if elapsed >= 20 {
                on_record_session.call(elapsed);
                session_start.set(now);
            }
        }
    });

    if book.format.eq_ignore_ascii_case("pdf") {
        return rsx! {
            div {
                class: "reader-screen flex-1 flex flex-col items-center relative h-[100svh] overflow-hidden transition-colors duration-300",
                style: "background-color: {theme.bg_color()}; color: {theme.text_primary()};",
                PdfCanvas {
                    initial_progress: book.progress_percent,
                    book: book,
                    theme: theme,
                    on_back: on_back_click,
                    on_progress: move |p| on_pdf_progress.call(p),
                }
            }
        };
    }

    let epub_book = match epub {
        Some(e) => e,
        None => {
            return rsx! {
                div {
                    class: "h-[100svh] flex flex-col items-center justify-center gap-3 p-8 text-center",
                    style: "background-color: {theme.bg_color()}; color: {theme.text_primary()};",
                    p { class: "font-serif text-lg", "Buku ini tidak dapat dibuka." }
                    button {
                        class: "px-4 py-2 rounded-full border text-sm apple-tap",
                        style: "border-color: {theme.border_color()};",
                        onclick: move |_| on_back.call(()),
                        "Kembali ke rak"
                    }
                }
            };
        }
    };
    let total_chapters = epub_book.chapters.len();
    let all_chapters = epub_book.chapters.clone();
    let current_ch = epub_book.chapters.get(current_chapter).cloned();
    let chapter_title = current_ch.as_ref().map(|c| c.title.clone());
    let local_fraction = *fraction.read();

    let progress_percent = if total_chapters > 0 {
        (((current_chapter as f32 + local_fraction) / total_chapters as f32) * 100.0)
            .round()
            .clamp(0.0, 100.0) as u32
    } else {
        0
    };

    let est_minutes_left = if total_chapters > 0 {
        ((total_chapters.saturating_sub(current_chapter)) * 4).max(1)
    } else {
        0
    };

    let font_family_css = match settings.font_family.as_str() {
        "sans" => "'Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif".to_string(),
        "mono" => "'JetBrains Mono', ui-monospace, SFMono-Regular, monospace".to_string(),
        _ => "'Newsreader', 'Playfair Display', Georgia, serif".to_string(),
    };

    let cur_mode = *reading_mode.read();
    let fullscreen_val = *is_fullscreen.read();

    rsx! {
        div {
            class: "reader-screen flex-1 flex flex-col items-center relative h-[100svh] overflow-hidden selection:bg-amber-200 transition-colors duration-300",
            style: "background-color: {theme.bg_color()}; color: {theme.text_primary()};",

            ReaderTopBar {
                theme: theme,
                reading_mode: cur_mode,
                is_bookmarked: is_bookmarked,
                is_fullscreen: fullscreen_val,
                on_toggle_mode: move |_| {
                    reading_mode.set(if *reading_mode.peek() == ReadingMode::Scroll {
                        ReadingMode::Paged
                    } else {
                        ReadingMode::Scroll
                    });
                },
                on_toggle_bookmark: move |_| on_toggle_bookmark.call(()),
                on_back: on_back_click,
                on_toggle_search: move |_| on_toggle_search.call(()),
                on_toggle_ai: move |_| on_toggle_ai.call(()),
                on_toggle_typography: move |_| on_toggle_typography.call(()),
                on_toggle_toc: move |_| on_toggle_toc.call(()),
                on_toggle_fullscreen: toggle_fullscreen,
            }

            ReaderCanvas {
                current_ch: current_ch,
                all_chapters: all_chapters,
                current_chapter: current_chapter,
                total_chapters: total_chapters,
                font_family_css: font_family_css,
                font_size: settings.font_size,
                line_height: settings.line_height,
                margin_horizontal: settings.margin_horizontal,
                theme: theme,
                reading_mode: cur_mode,
                is_fullscreen: fullscreen_val,
                highlight_keyword: highlight_keyword,
                on_change_chapter: {
                    let on_rec = on_record_session;
                    let on_ch = on_change_chapter;
                    move |ch| {
                        let elapsed = (chrono::Utc::now().timestamp() - *session_start.peek()).max(0) as u32;
                        if elapsed >= 5 {
                            on_rec.call(elapsed);
                            session_start.set(chrono::Utc::now().timestamp());
                        }
                        on_ch.call(ch);
                    }
                },
                on_metrics: move |(ch, f): (usize, f32)| {
                    fraction.set(f);
                    on_progress.call((ch, f));
                },
                on_clear_highlight: move |_| on_clear_highlight.call(()),
                on_toggle_fullscreen: toggle_fullscreen,
            }

            ReaderScrubber {
                progress_percent: progress_percent,
                current_chapter: current_chapter,
                total_chapters: total_chapters,
                est_minutes_left: est_minutes_left,
                theme: theme,
                is_fullscreen: fullscreen_val,
                chapter_title: chapter_title,
                on_change_theme: move |t| on_change_theme.call(t),
                on_seek: move |ch| {
                    if cur_mode == ReadingMode::Scroll {
                        let script = format!(
                            "(() => {{
                                const el = document.getElementById('baca-chapter-{}');
                                if (el) el.scrollIntoView({{ behavior: 'smooth', block: 'start' }});
                            }})();",
                            ch
                        );
                        let _ = document::eval(&script);
                    }
                    on_change_chapter.call(ch);
                },
            }
        }
    }
}
