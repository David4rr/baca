use dioxus::prelude::*;
use crate::engine::Chapter;
use crate::theme::ThemeMode;
use super::top_bar::ReadingMode;

#[component]
pub fn ReaderCanvas(
    current_ch: Option<Chapter>,
    current_chapter: usize,
    total_chapters: usize,
    font_family_css: String,
    font_size: u32,
    line_height: f32,
    theme: ThemeMode,
    reading_mode: ReadingMode,
    on_change_chapter: EventHandler<usize>,
    on_toggle_ai: EventHandler<()>,
) -> Element {
    let mut touch_start_x = use_signal(|| 0.0f64);
    let mut touch_start_y = use_signal(|| 0.0f64);
    let mut touch_last_x = use_signal(|| 0.0f64);
    let mut touch_last_y = use_signal(|| 0.0f64);

    let is_paged = reading_mode == ReadingMode::Paged;

    rsx! {
        main {
            class: "reading-canvas w-full max-w-2xl px-6 sm:px-8 pt-4 pb-48 flex flex-col relative transition-all duration-300 animate-fade-in",
            style: "font-family: {font_family_css}; font-size: {font_size}px; line-height: {line_height};",
            ontouchstart: move |e: TouchEvent| {
                if let Some(t) = e.touches().first() {
                    let (x, y) = (t.client_coordinates().x, t.client_coordinates().y);
                    touch_start_x.set(x);
                    touch_start_y.set(y);
                    touch_last_x.set(x);
                    touch_last_y.set(y);
                }
            },
            ontouchmove: move |e: TouchEvent| {
                if let Some(t) = e.touches().first() {
                    touch_last_x.set(t.client_coordinates().x);
                    touch_last_y.set(t.client_coordinates().y);
                }
            },
            ontouchend: move |_| {
                if is_paged {
                    let dx = *touch_last_x.read() - *touch_start_x.read();
                    let dy = *touch_last_y.read() - *touch_start_y.read();
                    if dy.abs() < 120.0 {
                        if dx < -45.0 && current_chapter + 1 < total_chapters {
                            on_change_chapter.call(current_chapter + 1);
                        } else if dx > 45.0 && current_chapter > 0 {
                            on_change_chapter.call(current_chapter - 1);
                        }
                    }
                }
            },

            // Floating Navigation Arrows (Paged / Swipe Mode)
            if is_paged {
                // Floating Left Arrow
                if current_chapter > 0 {
                    button {
                        class: "fixed left-2 sm:left-6 top-1/2 -translate-y-1/2 z-20 w-10 sm:w-12 h-10 sm:h-12 rounded-full border shadow-lg backdrop-blur-md flex items-center justify-center apple-tap transition-all hover:scale-110",
                        style: "background-color: {theme.surface_color()}CC; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        title: "Halaman / Bab Sebelumnya (Geser Kanan)",
                        onclick: move |_| on_change_chapter.call(current_chapter - 1),
                        svg { class: "w-5 h-5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round",
                            polyline { points: "15 18 9 12 15 6" }
                        }
                    }
                }

                // Floating Right Arrow
                if current_chapter + 1 < total_chapters {
                    button {
                        class: "fixed right-2 sm:right-6 top-1/2 -translate-y-1/2 z-20 w-10 sm:w-12 h-10 sm:h-12 rounded-full border shadow-lg backdrop-blur-md flex items-center justify-center apple-tap transition-all hover:scale-110",
                        style: "background-color: {theme.surface_color()}CC; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        title: "Halaman / Bab Selanjutnya (Geser Kiri)",
                        onclick: move |_| on_change_chapter.call(current_chapter + 1),
                        svg { class: "w-5 h-5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round",
                            polyline { points: "9 18 15 12 9 6" }
                        }
                    }
                }
            }

            if let Some(ch) = current_ch {
                div {
                    class: "chapter-content space-y-6 select-text",

                    // Chapter Eyebrow ("CHAPTER 5")
                    div {
                        class: "text-center mb-2 select-none",
                        span {
                            class: "font-sans text-[11px] uppercase tracking-[0.25em] font-semibold opacity-60",
                            style: "color: {theme.text_secondary()};",
                            "BAB {current_chapter + 1} DARI {total_chapters}"
                        }
                    }

                    // Chapter Headline
                    h1 {
                        class: "font-serif text-[28px] sm:text-[34px] font-normal tracking-tight text-center mb-8 leading-[1.2]",
                        style: "color: {theme.text_primary()};",
                        "{ch.title}"
                    }

                    // Chapter Body Text (Sanitized HTML with Embedded Base64 Images)
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
                            svg { class: "w-3 h-3 text-amber-500", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5",
                                path { d: "m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z" }
                            }
                            span { "Tanya AI" }
                        }
                    }
                }
            } else {
                div { class: "p-12 text-center opacity-60 font-sans text-sm", "Bab buku tidak dapat dimuat." }
            }

            // Navigation Controls
            if is_paged {
                div {
                    class: "mt-12 text-center select-none opacity-50 font-sans text-[11px] tracking-wide",
                    "← Geser layar ke kiri/kanan atau ketuk tombol panah untuk membalik halaman →"
                }
            } else {
                div {
                    class: "mt-16 pt-8 border-t flex items-center justify-between select-none",
                    style: "border-color: {theme.border_color()};",

                    button {
                        class: "px-4 py-2 rounded-xl border text-xs font-sans font-medium apple-tap transition-all disabled:opacity-20 disabled:pointer-events-none",
                        style: "border-color: {theme.border_color()}; background-color: {theme.surface_color()}; color: {theme.text_primary()};",
                        disabled: current_chapter == 0,
                        onclick: move |_| { if current_chapter > 0 { on_change_chapter.call(current_chapter - 1); } },
                        "← Bab Sebelumnya"
                    }

                    button {
                        class: "px-4 py-2 rounded-xl border text-xs font-sans font-medium apple-tap transition-all disabled:opacity-20 disabled:pointer-events-none",
                        style: "border-color: {theme.border_color()}; background-color: {theme.surface_color()}; color: {theme.text_primary()};",
                        disabled: current_chapter + 1 >= total_chapters,
                        onclick: move |_| { if current_chapter + 1 < total_chapters { on_change_chapter.call(current_chapter + 1); } },
                        "Bab Selanjutnya →"
                    }
                }
            }
        }
    }
}
