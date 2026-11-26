use std::sync::Arc;
use dioxus::prelude::*;

use crate::engine::EpubBook;
use crate::theme::ThemeMode;

#[component]
pub fn ScrollReaderContent(
    epub: Arc<EpubBook>,
    rendered_chapters: Vec<String>,
    current_chapter: usize,
    margin_horizontal: u32,
    theme: ThemeMode,
    is_fullscreen: bool,
) -> Element {
    let total_chapters = epub.chapters.len();
    let top_pad = if is_fullscreen {
        "max(calc(env(safe-area-inset-top, 0px) + 2rem), 2.5rem)"
    } else {
        "max(calc(env(safe-area-inset-top, 0px) + 5rem), 5.5rem)"
    };
    let bottom_pad = if is_fullscreen {
        "max(calc(env(safe-area-inset-bottom, 0px) + 2rem), 2.5rem)"
    } else {
        "max(calc(env(safe-area-inset-bottom, 0px) + 11rem), 12rem)"
    };

    rsx! {
        div {
            id: "baca-scroll-container",
            "data-target-chapter": "{current_chapter}",
            class: "w-full h-full overflow-y-auto overscroll-y-contain select-text",
            style: "touch-action: pan-y; -webkit-overflow-scrolling: touch;",
            div {
                class: "max-w-2xl mx-auto space-y-12 transition-all duration-300",
                style: "padding-top: {top_pad}; padding-bottom: {bottom_pad}; padding-left: {margin_horizontal}px; padding-right: {margin_horizontal}px;",

                for (idx, ch) in epub.chapters.iter().enumerate() {
                    div {
                        key: "{ch.index}_{idx}",
                        id: "baca-chapter-{idx}",
                        class: if idx == current_chapter {
                            "baca-chapter-section"
                        } else {
                            "baca-chapter-section baca-chapter-deferred"
                        },
                        "data-chapter-index": "{idx}",

                        div {
                            class: "text-center mb-8 pt-6 select-none",
                            h1 {
                                class: "font-serif text-[26px] sm:text-[32px] font-normal tracking-tight text-center mt-2 leading-[1.2]",
                                style: "color: {theme.text_primary()};",
                                "{ch.title}"
                            }
                        }

                        div {
                            class: "chapter-content select-text leading-relaxed text-justify",
                            dangerous_inner_html: "{rendered_chapters.get(idx).unwrap_or(&ch.html_content)}",
                        }

                        if idx + 1 < total_chapters {
                            div {
                                class: "my-12 flex items-center justify-center gap-3 opacity-30 select-none",
                                div { class: "h-px w-12 bg-current" }
                                div { class: "w-1.5 h-1.5 rounded-full bg-current" }
                                div { class: "h-px w-12 bg-current" }
                            }
                        }
                    }
                }

                div {
                    class: "py-16 text-center text-xs font-sans opacity-40 tracking-wider uppercase select-none",
                    "Akhir dari Buku Ini"
                }
            }
        }
    }
}
