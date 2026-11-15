use dioxus::prelude::*;
use crate::db::models::{Book, UserSettings};
use crate::engine::EpubBook;
use crate::theme::ThemeMode;

#[component]
pub fn ReaderView(
    book: Book,
    epub: EpubBook,
    current_chapter: usize,
    theme: ThemeMode,
    settings: UserSettings,
    on_change_chapter: EventHandler<usize>,
) -> Element {
    let total_chapters = epub.chapters.len();
    let current_ch = epub.chapters.get(current_chapter);

    let font_family_css = match settings.font_family.as_str() {
        "sans" => "ui-sans-serif, system-ui, -apple-system, sans-serif",
        "mono" => "ui-monospace, SFMono-Regular, Menlo, monospace",
        _ => "Georgia, Cambria, 'Times New Roman', serif",
    };

    rsx! {
        div {
            class: "reader-container flex-1 flex flex-col items-center relative overflow-y-auto selection:bg-teal-500/30",
            style: "background-color: {theme.bg_color()}; color: {theme.text_primary()};",

            // Reading Content Canvas
            main {
                class: "reading-canvas w-full max-w-3xl px-6 py-12 flex flex-col min-h-screen transition-all duration-300",
                style: "font-family: {font_family_css}; font-size: {settings.font_size}px; line-height: {settings.line_height};",

                if let Some(ch) = current_ch {
                    div {
                        class: "chapter-content space-y-6 select-text",
                        // Chapter Heading
                        h1 {
                            class: "text-2xl md:text-3xl font-bold tracking-tight mb-8 pb-4 border-b font-serif",
                            style: "border-color: {theme.border_color()};",
                            "{ch.title}"
                        }

                        // Sanitized Chapter Content
                        div {
                            class: "prose max-w-none",
                            dangerous_inner_html: "{ch.html_content}",
                        }
                    }
                } else {
                    div {
                        class: "p-12 text-center opacity-60",
                        "Chapter could not be loaded."
                    }
                }

                // Chapter Navigation Footer
                div {
                    class: "mt-16 pt-8 border-t flex items-center justify-between select-none",
                    style: "border-color: {theme.border_color()};",

                    button {
                        class: "px-5 py-2.5 rounded-xl border text-sm font-medium hover:bg-white/10 disabled:opacity-30 disabled:pointer-events-none transition-all",
                        style: "border-color: {theme.border_color()};",
                        disabled: current_chapter == 0,
                        onclick: move |_| {
                            if current_chapter > 0 {
                                on_change_chapter.call(current_chapter - 1);
                            }
                        },
                        "← Previous Chapter"
                    }

                    span {
                        class: "text-xs font-mono opacity-60",
                        "Chapter {current_chapter + 1} of {total_chapters}"
                    }

                    button {
                        class: "px-5 py-2.5 rounded-xl border text-sm font-medium hover:bg-white/10 disabled:opacity-30 disabled:pointer-events-none transition-all",
                        style: "border-color: {theme.border_color()};",
                        disabled: current_chapter + 1 >= total_chapters,
                        onclick: move |_| {
                            if current_chapter + 1 < total_chapters {
                                on_change_chapter.call(current_chapter + 1);
                            }
                        },
                        "Next Chapter →"
                    }
                }
            }
        }
    }
}
