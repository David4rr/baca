use dioxus::prelude::*;
use crate::engine::Chapter;
use crate::theme::ThemeMode;

#[component]
pub fn ReaderCanvas(
    current_ch: Option<Chapter>,
    current_chapter: usize,
    total_chapters: usize,
    font_family_css: String,
    font_size: u32,
    line_height: f32,
    theme: ThemeMode,
    on_change_chapter: EventHandler<usize>,
    on_toggle_ai: EventHandler<()>,
) -> Element {
    rsx! {
        main {
            class: "reading-canvas w-full max-w-2xl px-6 sm:px-8 pt-4 pb-48 flex flex-col transition-all duration-300 animate-fade-in",
            style: "font-family: {font_family_css}; font-size: {font_size}px; line-height: {line_height};",

            if let Some(ch) = current_ch {
                div {
                    class: "chapter-content space-y-6 select-text",

                    // Chapter Eyebrow ("CHAPTER 5")
                    div {
                        class: "text-center mb-2 select-none",
                        span {
                            class: "font-sans text-[11px] uppercase tracking-[0.25em] font-semibold opacity-60",
                            style: "color: {theme.text_secondary()};",
                            "CHAPTER {current_chapter + 1}"
                        }
                    }

                    // Chapter Headline ("The Wealth Equation")
                    h1 {
                        class: "font-serif text-[30px] sm:text-[36px] font-normal tracking-tight text-center mb-8 leading-[1.2]",
                        style: "color: {theme.text_primary()};",
                        "{ch.title}"
                    }

                    // Chapter Body Text (Sanitized HTML)
                    div {
                        class: "prose max-w-none leading-relaxed text-justify",
                        dangerous_inner_html: "{ch.html_content}",
                    }

                    // Floating Highlight Action Pill ("✨ Tanya AI")
                    div {
                        class: "flex justify-end pt-4 pb-2 select-none",
                        button {
                            class: "rounded-full px-4 py-1.5 text-xs font-sans font-medium flex items-center gap-1.5 shadow-sm border apple-tap transition-all",
                            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_secondary()};",
                            onclick: move |_| on_toggle_ai.call(()),
                            svg {
                                class: "w-3 h-3 text-amber-500",
                                width: "12",
                                height: "12",
                                style: "width: 12px; height: 12px; min-width: 12px; min-height: 12px;",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.5",
                                path { d: "m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z" }
                            }
                            span { "Tanya AI" }
                        }
                    }
                }
            } else {
                div {
                    class: "p-12 text-center opacity-60 font-sans text-sm",
                    "Bab buku tidak dapat dimuat."
                }
            }

            // Next / Previous Chapter Navigators
            div {
                class: "mt-16 pt-8 border-t flex items-center justify-between select-none",
                style: "border-color: {theme.border_color()};",

                button {
                    class: "px-4 py-2 rounded-xl border text-xs font-sans font-medium apple-tap transition-all disabled:opacity-20 disabled:pointer-events-none",
                    style: "border-color: {theme.border_color()}; background-color: {theme.surface_color()}; color: {theme.text_primary()};",
                    disabled: current_chapter == 0,
                    onclick: move |_| {
                        if current_chapter > 0 {
                            on_change_chapter.call(current_chapter - 1);
                        }
                    },
                    "← Bab Sebelumnya"
                }

                button {
                    class: "px-4 py-2 rounded-xl border text-xs font-sans font-medium apple-tap transition-all disabled:opacity-20 disabled:pointer-events-none",
                    style: "border-color: {theme.border_color()}; background-color: {theme.surface_color()}; color: {theme.text_primary()};",
                    disabled: current_chapter + 1 >= total_chapters,
                    onclick: move |_| {
                        if current_chapter + 1 < total_chapters {
                            on_change_chapter.call(current_chapter + 1);
                        }
                    },
                    "Bab Selanjutnya →"
                }
            }
        }
    }
}
