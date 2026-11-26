use std::sync::Arc;
use dioxus::prelude::*;
use serde::Deserialize;

use crate::engine::EpubBook;
use crate::theme::ThemeMode;

use super::highlight::highlight_html;
use super::paged_view::PagedReaderContent;
use super::scripts::{PAGED_GESTURE_SCRIPT, PROBE_SCRIPT, SCROLL_ENGINE_SCRIPT};
use super::scroll_view::ScrollReaderContent;
use super::top_bar::ReadingMode;

#[derive(Deserialize)]
struct PageProbe {
    pages: usize,
    #[serde(default)]
    _width: f64,
}

fn prepare_chapter_html(raw: &str, keyword: Option<&str>) -> String {
    match keyword.map(str::trim).filter(|kw| kw.len() >= 2) {
        Some(kw) => highlight_html(raw, kw),
        None => raw.to_string(),
    }
}

#[component]
pub fn ReaderCanvas(
    epub: Arc<EpubBook>,
    current_chapter: usize,
    font_family_css: String,
    font_size: u32,
    line_height: f32,
    margin_horizontal: u32,
    theme: ThemeMode,
    reading_mode: ReadingMode,
    is_fullscreen: bool,
    highlight_keyword: Option<String>,
    on_change_chapter: EventHandler<usize>,
    on_ui_fraction: EventHandler<f32>,
    on_commit_progress: EventHandler<(usize, f32)>,
    on_clear_highlight: EventHandler<()>,
    on_toggle_fullscreen: EventHandler<()>,
) -> Element {
    let mut pages = use_signal(|| 1usize);
    let mut page = use_signal(|| 0usize);
    let mut landing_last = use_signal(|| false);
    let mut landing_for = use_signal(|| None::<usize>);
    let mut resize_epoch = use_signal(|| 0u32);
    let mut measured = use_signal(|| false);

    let total_chapters = epub.chapters.len();
    let is_paged = reading_mode == ReadingMode::Paged;
    let page_index = *page.read();
    let page_count = (*pages.read()).max(1);

    let epub_single = epub.clone();
    let kw_single = highlight_keyword.clone();
    let rendered_single = use_memo(use_reactive!(|(is_paged, current_chapter, kw_single)| {
        if !is_paged {
            return String::new();
        }
        epub_single
            .chapters
            .get(current_chapter)
            .map(|ch| prepare_chapter_html(&ch.html_content, kw_single.as_deref()))
            .unwrap_or_default()
    }));

    let epub_all = epub.clone();
    let kw_all = highlight_keyword.clone();
    let rendered_all = use_memo(use_reactive!(|(is_paged, kw_all)| {
        if is_paged {
            return Vec::new();
        }
        epub_all
            .chapters
            .iter()
            .map(|ch| prepare_chapter_html(&ch.html_content, kw_all.as_deref()))
            .collect::<Vec<_>>()
    }));

    // Reset pagination state when chapter or mode changes in Paged mode.
    use_effect(use_reactive!(|(is_paged, current_chapter)| {
        if !is_paged {
            return;
        }
        page.set(0);
        pages.set(1);
        measured.set(false);
        let landing = *landing_for.peek() == Some(current_chapter);
        landing_last.set(landing);
        if landing {
            landing_for.set(None);
        }
    }));

    // Measure page count whenever layout-affecting props or viewport width change.
    let font_probe = font_family_css.clone();
    let kw_probe = highlight_keyword.clone();
    use_effect(use_reactive!(|(
        is_paged,
        current_chapter,
        font_size,
        line_height,
        font_probe,
        margin_horizontal,
        kw_probe,
        is_fullscreen
    )| {
        let _ = (current_chapter, font_size, line_height, font_probe, margin_horizontal, kw_probe, is_fullscreen);
        let _ = *resize_epoch.read();
        if !is_paged {
            return;
        }
        spawn(async move {
            let probe = document::eval(PROBE_SCRIPT);
            if let Ok(p) = probe.join::<PageProbe>().await {
                let clamped = p.pages.max(1);
                if *pages.peek() != clamped {
                    pages.set(clamped);
                }
                let cur = *page.peek();
                if cur >= clamped {
                    page.set(clamped - 1);
                }
                measured.set(true);
            }
        });
    }));

    // Bind JS gesture or scroll engine whenever reading mode toggles.
    use_effect(use_reactive!(|(is_paged,)| {
        spawn(async move {
            if is_paged {
                let _ = document::eval(PAGED_GESTURE_SCRIPT);
            } else {
                let _ = document::eval(SCROLL_ENGINE_SCRIPT);
            }
        });
    }));

    // Jump to last page when navigating backwards into previous chapter in Paged mode.
    use_effect(use_reactive!(|(is_paged,)| {
        if !is_paged || !*landing_last.peek() || *pages.read() <= 1 {
            return;
        }
        landing_last.set(false);
        let last = *pages.peek() - 1;
        page.set(last);
    }));

    // Commit progress when page changes in Paged mode.
    use_effect(use_reactive!(|(is_paged, current_chapter)| {
        if !is_paged || !*measured.read() {
            return;
        }
        let p = *page.read();
        let total = (*pages.read()).max(1);
        let fraction = ((p + 1) as f32 / total as f32).clamp(0.0, 1.0);
        on_commit_progress.call((current_chapter, fraction));
    }));

    let mut go_next = move || {
        let p = *page.peek();
        let total = (*pages.peek()).max(1);
        if p + 1 < total {
            page.set(p + 1);
        } else if current_chapter + 1 < total_chapters {
            page.set(0);
            landing_last.set(false);
            on_change_chapter.call(current_chapter + 1);
        }
    };

    let mut go_prev = move || {
        let p = *page.peek();
        if p > 0 {
            page.set(p - 1);
        } else if current_chapter > 0 {
            landing_for.set(Some(current_chapter - 1));
            landing_last.set(true);
            on_change_chapter.call(current_chapter - 1);
        }
    };

    rsx! {
        main {
            class: "reading-canvas relative w-full h-[100svh] flex flex-col overflow-hidden",
            style: "font-family: {font_family_css}; font-size: {font_size}px; line-height: {line_height};",

            if is_paged {
                PagedReaderContent {
                    rendered_html: rendered_single(),
                    page_index: page_index,
                    page_count: page_count,
                    margin_horizontal: margin_horizontal,
                    is_fullscreen: is_fullscreen,
                }
            } else {
                ScrollReaderContent {
                    epub: epub.clone(),
                    rendered_chapters: rendered_all(),
                    current_chapter: current_chapter,
                    margin_horizontal: margin_horizontal,
                    theme: theme,
                    is_fullscreen: is_fullscreen,
                }
            }

            // Hidden action triggers
            button { id: "baca-paged-next-btn", class: "hidden", onclick: move |_| go_next() }
            button { id: "baca-paged-prev-btn", class: "hidden", onclick: move |_| go_prev() }
            button { id: "baca-fullscreen-toggle-btn", class: "hidden", onclick: move |_| on_toggle_fullscreen.call(()) }

            // Lightweight UI-only scroll progress sync (no DB write / no global state mutation)
            input {
                id: "baca-scroll-sync",
                r#type: "text",
                style: "display: none;",
                oninput: move |e| {
                    if !is_paged {
                        if let Ok(f) = e.value().parse::<f32>() {
                            on_ui_fraction.call(f);
                        }
                    }
                },
            }
            // Persisted only when scroll settles (`scrollend`)
            input {
                id: "baca-scroll-commit",
                r#type: "text",
                style: "display: none;",
                oninput: move |e| {
                    if !is_paged {
                        if let Ok(f) = e.value().parse::<f32>() {
                            on_commit_progress.call((current_chapter, f));
                        }
                    }
                },
            }
            input {
                id: "baca-chapter-sync",
                r#type: "text",
                style: "display: none;",
                oninput: move |e| {
                    if !is_paged {
                        if let Ok(ch) = e.value().parse::<usize>() {
                            if ch != current_chapter && ch < total_chapters {
                                on_change_chapter.call(ch);
                            }
                        }
                    }
                },
            }
            input {
                id: "baca-resize-sync",
                r#type: "text",
                style: "display: none;",
                oninput: move |_| {
                    let next = *resize_epoch.peek() + 1;
                    resize_epoch.set(next);
                },
            }

            if total_chapters == 0 {
                div { class: "p-12 text-center opacity-60 font-sans text-sm", "Bab buku tidak dapat dimuat." }
            }

            if let Some(kw) = &highlight_keyword {
                div { class: "fixed left-1/2 -translate-x-1/2 bottom-40 z-20 select-none",
                    button {
                        class: "rounded-full px-3 py-1.5 text-xs font-sans font-medium flex items-center gap-1.5 border apple-tap shadow-md",
                        style: "background-color: #fef3c7; border-color: #f59e0b; color: #78350f;",
                        onclick: move |_| on_clear_highlight.call(()),
                        span { "Menyorot: \"{kw}\"" }
                        svg { class: "w-3 h-3", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5",
                            line { x1: "18", y1: "6", x2: "6", y2: "18" }
                            line { x1: "6", y1: "6", x2: "18", y2: "18" }
                        }
                    }
                }
            }
        }
    }
}
