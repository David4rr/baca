use dioxus::prelude::*;
use crate::db::models::Book;
use crate::theme::ThemeMode;

#[component]
pub fn PdfCanvas(
    book: Book,
    theme: ThemeMode,
    on_back: EventHandler<()>,
) -> Element {
    let mut current_page = use_signal(|| 1usize);
    let total_pages = use_signal(|| 1usize);
    let mut zoom_scale = use_signal(|| 100u32);

    let file_url = if book.file_path.starts_with("http") || book.file_path.starts_with("file://") {
        book.file_path.clone()
    } else {
        format!("file://{}", book.file_path)
    };

    let title_escaped = book.title.replace('"', "\\\"");
    let pdf_script = super::pdf_script::get_pdf_script(&file_url);

    rsx! {
        div {
            class: "w-full max-w-4xl mx-auto px-4 sm:px-6 pt-4 pb-20 flex flex-col flex-1 min-h-screen animate-fade-in",

            // Top Header
            div {
                class: "flex items-center justify-between pb-3 select-none",
                button {
                    class: "w-10 h-10 rounded-full flex items-center justify-center border shadow-sm apple-tap transition-all",
                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    title: "Kembali ke Rak",
                    onclick: move |_| on_back.call(()),
                    svg { class: "w-5 h-5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.2", stroke_linecap: "round", stroke_linejoin: "round",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                }

                div {
                    class: "flex flex-col items-center max-w-[60%] text-center min-w-0",
                    h2 {
                        class: "font-serif text-base font-semibold truncate",
                        style: "color: {theme.text_primary()};",
                        "{title_escaped}"
                    }
                    span {
                        class: "text-xs font-sans opacity-60 truncate",
                        style: "color: {theme.text_secondary()};",
                        "{book.author}"
                    }
                }

                span {
                    class: "px-2.5 py-1 rounded-full text-[10px] font-sans font-bold uppercase tracking-wider",
                    style: "background-color: #ef444420; color: #ef4444;",
                    "PDF"
                }
            }

            // PDF Toolbar (Page Selector & Zoom)
            div {
                class: "flex items-center justify-between px-3 py-2 mb-3 rounded-2xl border text-xs font-sans select-none",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",

                // Page Navigation
                div { class: "flex items-center gap-2",
                    button {
                        class: "px-2.5 py-1 rounded-lg border apple-tap disabled:opacity-30",
                        style: "border-color: {theme.border_color()};",
                        disabled: *current_page.read() <= 1,
                        onclick: move |_| {
                            let p = *current_page.read();
                            if p > 1 {
                                current_page.set(p - 1);
                                let scale = (*zoom_scale.read() as f32) / 100.0 * 1.2;
                                let _ = document::eval(&format!("window.__baca_render_pdf_page({}, {});", p - 1, scale));
                            }
                        },
                        "◀"
                    }
                    span { "Hal. {current_page} / " }
                    span { id: "baca-pdf-total-pages", "{total_pages}" }
                    button {
                        class: "px-2.5 py-1 rounded-lg border apple-tap",
                        style: "border-color: {theme.border_color()};",
                        onclick: move |_| {
                            let p = *current_page.read();
                            current_page.set(p + 1);
                            let scale = (*zoom_scale.read() as f32) / 100.0 * 1.2;
                            let _ = document::eval(&format!("window.__baca_render_pdf_page({}, {});", p + 1, scale));
                        },
                        "▶"
                    }
                }

                // Zoom Controls
                div { class: "flex items-center gap-1.5",
                    button {
                        class: "w-7 h-7 rounded-lg border flex items-center justify-center font-bold apple-tap",
                        style: "border-color: {theme.border_color()};",
                        disabled: *zoom_scale.read() <= 60,
                        onclick: move |_| {
                            let z = *zoom_scale.read();
                            if z > 60 {
                                zoom_scale.set(z - 20);
                                let scale = ((z - 20) as f32) / 100.0 * 1.2;
                                let _ = document::eval(&format!("window.__baca_render_pdf_page({}, {});", *current_page.read(), scale));
                            }
                        },
                        "-"
                    }
                    span { class: "w-10 text-center font-semibold", "{zoom_scale}%" }
                    button {
                        class: "w-7 h-7 rounded-lg border flex items-center justify-center font-bold apple-tap",
                        style: "border-color: {theme.border_color()};",
                        disabled: *zoom_scale.read() >= 200,
                        onclick: move |_| {
                            let z = *zoom_scale.read();
                            if z < 200 {
                                zoom_scale.set(z + 20);
                                let scale = ((z + 20) as f32) / 100.0 * 1.2;
                                let _ = document::eval(&format!("window.__baca_render_pdf_page({}, {});", *current_page.read(), scale));
                            }
                        },
                        "+"
                    }
                }
            }

            // Canvas Display Area
            div {
                class: "flex-1 w-full rounded-2xl overflow-auto border shadow-lg relative min-h-[65vh] flex items-center justify-center p-4",
                style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",

                canvas {
                    id: "baca-pdf-canvas",
                    class: "max-w-full shadow-md rounded-lg transition-transform",
                }

                // Fallback state if PDF.js is unavailable
                div {
                    id: "baca-pdf-fallback",
                    class: "hidden absolute inset-0 flex-col items-center justify-center p-6 text-center space-y-4",
                    p { class: "text-sm font-sans opacity-70", "Pratinjau langsung memerlukan koneksi awal atau pembaca PDF eksternal." }
                    a {
                        href: "{file_url}",
                        target: "_blank",
                        class: "px-5 py-2.5 rounded-xl font-sans text-xs font-semibold shadow-sm apple-tap transition-all",
                        style: "background-color: {theme.accent_color()}; color: {theme.accent_contrast_text()};",
                        "Buka dengan Pembaca PDF Bawaan ↗"
                    }
                }
            }

            script { dangerous_inner_html: "{pdf_script}" }
        }
    }
}
