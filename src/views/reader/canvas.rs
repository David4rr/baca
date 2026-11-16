use dioxus::prelude::*;
use crate::engine::Chapter;
use crate::theme::ThemeMode;
use super::highlight::highlight_html;
use super::top_bar::ReadingMode;

#[component]
pub fn ReaderCanvas(
    current_ch: Option<Chapter>,
    next_ch: Option<Chapter>,
    current_chapter: usize,
    total_chapters: usize,
    font_family_css: String,
    font_size: u32,
    line_height: f32,
    theme: ThemeMode,
    reading_mode: ReadingMode,
    highlight_keyword: Option<String>,
    on_change_chapter: EventHandler<usize>,
    on_clear_highlight: EventHandler<()>,
) -> Element {
    let mut touch_start_x = use_signal(|| 0.0f64);
    let mut touch_start_y = use_signal(|| 0.0f64);
    let mut touch_last_x = use_signal(|| 0.0f64);
    let mut touch_last_y = use_signal(|| 0.0f64);
    let mut current_page = use_signal(|| 0usize);

    let mut last_ch = use_signal(|| current_chapter);
    if *last_ch.read() != current_chapter {
        *last_ch.write() = current_chapter;
        current_page.set(0);
    }

    let is_paged = reading_mode == ReadingMode::Paged;
    let est_pages = current_ch.as_ref().map(|c| {
        let wc = c.html_content.split_whitespace().count();
        ((wc as f32 / 240.0).ceil() as usize).max(1)
    }).unwrap_or(1);

    let mut go_next = {
        let on_change = on_change_chapter.clone();
        move || {
            let p = *current_page.read();
            if p + 1 < est_pages { current_page.set(p + 1); }
            else if current_chapter + 1 < total_chapters { current_page.set(0); on_change.call(current_chapter + 1); }
        }
    };

    let mut go_prev = {
        let on_change = on_change_chapter.clone();
        move || {
            let p = *current_page.read();
            if p > 0 { current_page.set(p - 1); }
            else if current_chapter > 0 { current_page.set(0); on_change.call(current_chapter - 1); }
        }
    };

    let render_chapter = |ch: &Chapter| {
        if let Some(kw) = &highlight_keyword {
            let clean = kw.trim();
            if clean.len() >= 2 { highlight_html(&ch.html_content, clean) } else { ch.html_content.clone() }
        } else {
            ch.html_content.clone()
        }
    };

    let cur_rendered = current_ch.as_ref().map(&render_chapter).unwrap_or_default();
    let next_rendered = next_ch.as_ref().map(&render_chapter);

    rsx! {
        main {
            class: "reading-canvas w-full max-w-2xl px-6 sm:px-8 pb-48 flex flex-col relative transition-all duration-300 animate-fade-in",
            style: "font-family: {font_family_css}; font-size: {font_size}px; line-height: {line_height}; padding-top: max(calc(env(safe-area-inset-top, 0px) + 4.5rem), 5.5rem);",
            ontouchstart: move |e: TouchEvent| {
                if let Some(t) = e.touches().first() {
                    let (x, y) = (t.client_coordinates().x, t.client_coordinates().y);
                    touch_start_x.set(x); touch_start_y.set(y);
                    touch_last_x.set(x); touch_last_y.set(y);
                }
            },
            ontouchmove: move |e: TouchEvent| {
                if let Some(t) = e.touches().first() {
                    touch_last_x.set(t.client_coordinates().x); touch_last_y.set(t.client_coordinates().y);
                }
            },
            ontouchend: move |_| {
                if is_paged {
                    let dx = *touch_last_x.read() - *touch_start_x.read();
                    let dy = *touch_last_y.read() - *touch_start_y.read();
                    if dy.abs() < 100.0 {
                        if dx < -40.0 { go_next(); } else if dx > 40.0 { go_prev(); }
                    }
                }
            },

            if let Some(ch) = current_ch {
                div {
                    class: "chapter-content space-y-6 select-text",
                    div { class: "text-center mb-2 select-none",
                        span { class: "font-sans text-[11px] uppercase tracking-[0.25em] font-semibold opacity-60", style: "color: {theme.text_secondary()};",
                            "BAB {current_chapter + 1} DARI {total_chapters}"
                        }
                    }
                    h1 { class: "font-serif text-[28px] sm:text-[34px] font-normal tracking-tight text-center mb-8 leading-[1.2]", style: "color: {theme.text_primary()};",
                        "{ch.title}"
                    }

                    if is_paged {
                        div {
                            class: "paged-container relative w-full overflow-hidden rounded-2xl cursor-pointer",
                            style: "height: calc(100vh - 14rem); max-height: calc(100vh - 14rem);",
                            onclick: move |e: MouseEvent| {
                                let x = e.client_coordinates().x;
                                let w = 600.0; // approximate center split
                                if x > w * 0.65 { go_next(); } else if x < w * 0.35 { go_prev(); }
                            },
                            div {
                                class: "prose max-w-none leading-relaxed text-justify transition-transform duration-300 ease-out",
                                style: "column-width: 100%; column-gap: 3rem; column-fill: auto; height: 100%; transform: translateX(calc(-{current_page} * (100% + 3rem)));",
                                dangerous_inner_html: "{cur_rendered}",
                            }
                        }
                    } else {
                        div { class: "prose max-w-none leading-relaxed text-justify", dangerous_inner_html: "{cur_rendered}" }

                        // Seamless continuous scroll into next chapter (Zero loading delays)
                        if let (Some(next), Some(next_html)) = (next_ch, next_rendered) {
                            div { class: "pt-24 pb-8 text-center select-none",
                                span { class: "font-sans text-[11px] uppercase tracking-[0.25em] font-semibold opacity-50",
                                    "BAB {current_chapter + 2} DARI {total_chapters}"
                                }
                                h2 { class: "font-serif text-[26px] sm:text-[30px] font-normal tracking-tight text-center mt-2 leading-[1.2]",
                                    "{next.title}"
                                }
                            }
                            div { class: "prose max-w-none leading-relaxed text-justify", dangerous_inner_html: "{next_html}" }
                        }
                    }

                    if let Some(kw) = &highlight_keyword {
                        div { class: "pt-4 pb-2 select-none",
                            button {
                                class: "rounded-full px-3 py-1 text-xs font-sans font-medium flex items-center gap-1.5 border apple-tap shadow-xs",
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
            } else {
                div { class: "p-12 text-center opacity-60 font-sans text-sm", "Bab buku tidak dapat dimuat." }
            }

            if is_paged {
                div { class: "mt-4 text-center select-none font-sans text-xs opacity-60 tracking-wide",
                    "Halaman {current_page + 1} dari {est_pages} • Bab {current_chapter + 1}"
                }
            } else if current_chapter + 2 < total_chapters {
                div { class: "pt-16 pb-8 flex flex-col items-center justify-center select-none",
                    button {
                        class: "px-5 py-2.5 rounded-full text-xs font-sans font-medium border shadow-sm apple-tap transition-all hover:scale-105",
                        style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        onclick: {
                            let on_ch = on_change_chapter.clone();
                            move |_| on_ch.call(current_chapter + 2)
                        },
                        "Lanjut ke Bab {current_chapter + 3}"
                    }
                }
            } else {
                div { class: "py-16 text-center select-none font-sans text-xs opacity-40 tracking-wider",
                    "Akhir dari Buku Ini"
                }
            }
        }
    }
}
