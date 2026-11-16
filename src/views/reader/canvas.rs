use dioxus::prelude::*;
use crate::engine::Chapter;
use crate::theme::ThemeMode;
use super::highlight::highlight_html;
use super::top_bar::ReadingMode;

const SCROLL_SCRIPT: &str = r#"
    (function() {
        if (window.__baca_obs) window.__baca_obs.disconnect();
        var s = document.getElementById("baca-scroll-sentinel");
        var t = document.getElementById("baca-auto-next-trigger");
        if (s && t) {
            var o = new IntersectionObserver(function(e) {
                if (e[0] && e[0].isIntersecting) t.click();
            }, { threshold: 0.1 });
            o.observe(s);
            window.__baca_obs = o;
        }
    })();
"#;

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

    let rendered_html = current_ch.as_ref().map(|ch| {
        if let Some(kw) = &highlight_keyword {
            let clean = kw.trim();
            if clean.len() >= 2 { highlight_html(&ch.html_content, clean) } else { ch.html_content.clone() }
        } else {
            ch.html_content.clone()
        }
    }).unwrap_or_default();

    rsx! {
        main {
            class: "reading-canvas w-full max-w-2xl px-6 sm:px-8 pt-4 pb-48 flex flex-col relative transition-all duration-300 animate-fade-in",
            style: "font-family: {font_family_css}; font-size: {font_size}px; line-height: {line_height};",
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
                    if dy.abs() < 120.0 {
                        if dx < -45.0 { go_next(); } else if dx > 45.0 { go_prev(); }
                    }
                }
            },

            // Floating Navigation Arrows (Paged Mode)
            if is_paged {
                if *current_page.read() > 0 || current_chapter > 0 {
                    button {
                        class: "fixed left-2 sm:left-6 top-1/2 -translate-y-1/2 z-20 w-10 sm:w-12 h-10 sm:h-12 rounded-full border shadow-lg backdrop-blur-md flex items-center justify-center apple-tap transition-all hover:scale-110",
                        style: "background-color: {theme.surface_color()}CC; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        title: "Halaman Sebelumnya", onclick: move |_| go_prev(),
                        svg { class: "w-5 h-5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5", polyline { points: "15 18 9 12 15 6" } }
                    }
                }
                if *current_page.read() + 1 < est_pages || current_chapter + 1 < total_chapters {
                    button {
                        class: "fixed right-2 sm:right-6 top-1/2 -translate-y-1/2 z-20 w-10 sm:w-12 h-10 sm:h-12 rounded-full border shadow-lg backdrop-blur-md flex items-center justify-center apple-tap transition-all hover:scale-110",
                        style: "background-color: {theme.surface_color()}CC; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        title: "Halaman Selanjutnya", onclick: move |_| go_next(),
                        svg { class: "w-5 h-5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5", polyline { points: "9 18 15 12 9 6" } }
                    }
                }
            }

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
                        div { class: "paged-container relative w-full overflow-hidden rounded-2xl", style: "height: calc(100vh - 14rem); max-height: calc(100vh - 14rem);",
                            div {
                                class: "prose max-w-none leading-relaxed text-justify transition-transform duration-300 ease-out",
                                style: "column-width: 100%; column-gap: 3rem; column-fill: auto; height: 100%; transform: translateX(calc(-{current_page} * (100% + 3rem)));",
                                dangerous_inner_html: "{rendered_html}",
                            }
                        }
                    } else {
                        div { class: "prose max-w-none leading-relaxed text-justify", dangerous_inner_html: "{rendered_html}" }
                    }

                    if let Some(kw) = &highlight_keyword {
                        div { class: "pt-4 pb-2 select-none",
                            button {
                                class: "rounded-full px-3 py-1 text-xs font-sans font-medium flex items-center gap-1.5 border apple-tap shadow-xs",
                                style: "background-color: #fef3c7; border-color: #f59e0b; color: #78350f;",
                                onclick: move |_| on_clear_highlight.call(()),
                                span { "Menyorot: \"{kw}\"" }
                                span { class: "font-bold", "✕" }
                            }
                        }
                    }
                }
            } else {
                div { class: "p-12 text-center opacity-60 font-sans text-sm", "Bab buku tidak dapat dimuat." }
            }

            if is_paged {
                div { class: "mt-4 text-center select-none font-sans text-xs opacity-60 tracking-wide",
                    "Halaman {current_page + 1} dari {est_pages} • Geser ke kiri/kanan untuk membalik"
                }
            } else if current_chapter + 1 < total_chapters {
                div {
                    id: "baca-scroll-sentinel",
                    class: "py-16 flex flex-col items-center justify-center opacity-40 select-none animate-pulse",
                    style: "color: {theme.text_secondary()};",
                    span { class: "font-sans text-xs tracking-wider", "Memuat Bab Selanjutnya..." }
                    button {
                        id: "baca-auto-next-trigger",
                        class: "hidden",
                        onclick: {
                            let on_ch = on_change_chapter.clone();
                            move |_| {
                                if current_chapter + 1 < total_chapters {
                                    on_ch.call(current_chapter + 1);
                                }
                            }
                        }
                    }
                    script { dangerous_inner_html: "{SCROLL_SCRIPT}" }
                }
            }
        }
    }
}
