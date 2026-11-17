use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine as _;
use dioxus::prelude::*;

use crate::db::models::Book;
use crate::theme::ThemeMode;

#[component]
pub fn PdfCanvas(
    book: Book,
    theme: ThemeMode,
    initial_progress: f32,
    on_back: EventHandler<()>,
    on_progress: EventHandler<f32>,
) -> Element {
    let mut current_page = use_signal(|| 1usize);
    let mut total_pages = use_signal(|| 0usize);
    let mut zoom = use_signal(|| 100u32);
    let mut error = use_signal(|| None::<String>);

    use_hook(move || {
        let path = book.file_path.clone();
        spawn(async move {
            let read = tokio::task::spawn_blocking(move || std::fs::read(&path)).await;
            match read {
                Ok(Ok(bytes)) if bytes.len() <= 128 * 1024 * 1024 => {
                    let encoded = BASE64.encode(bytes);
                    let script = super::pdf_script::get_boot_script(&encoded, initial_progress);
                    let _eval = document::eval(&script);
                }
                Ok(Ok(_)) => error.set(Some("Berkas PDF terlalu besar untuk dibuka".to_string())),
                Ok(Err(e)) => error.set(Some(format!("Gagal membaca berkas PDF: {e}"))),
                Err(e) => error.set(Some(format!("Gagal membaca berkas PDF: {e}"))),
            }
        });
    });

    use_effect(move || {
        let page = *current_page.read();
        let scale = *zoom.read() as f64 / 100.0;
        if *total_pages.read() == 0 {
            return;
        }
        spawn(async move {
            let _ = document::eval(&format!(
                "window.__baca_pdf_render && window.__baca_pdf_render({page}, {scale});"
            ));
        });
    });

    use_effect(move || {
        let page = *current_page.read();
        let total = *total_pages.read();
        if total > 0 {
            on_progress.call((page as f32 / total as f32).clamp(0.0, 1.0));
        }
    });

    let total = *total_pages.read();
    let page = *current_page.read();
    let can_prev = page > 1;
    let can_next = total > 0 && page < total;

    rsx! {
        div {
            class: "w-full h-[100svh] flex flex-col overflow-hidden",
            style: "background-color: {theme.bg_color()}; color: {theme.text_primary()};",

            input {
                id: "baca-pdf-total",
                r#type: "text",
                style: "display: none;",
                oninput: move |e| {
                    if let Ok(n) = e.value().parse::<usize>() {
                        total_pages.set(n);
                    }
                },
            }
            input {
                id: "baca-pdf-current",
                r#type: "text",
                style: "display: none;",
                oninput: move |e| {
                    if let Ok(n) = e.value().parse::<usize>() {
                        current_page.set(n);
                    }
                },
            }
            input {
                id: "baca-pdf-error",
                r#type: "text",
                style: "display: none;",
                oninput: move |e| {
                    if !e.value().is_empty() {
                        error.set(Some(e.value()));
                    }
                },
            }

            div {
                class: "flex items-center justify-between gap-3 px-4 pb-3 border-b apple-nav-glass",
                style: "padding-top: max(calc(env(safe-area-inset-top, 0px) + 0.65rem), 2.25rem); background-color: {theme.bg_color()}F0; border-color: {theme.border_color()}60;",
                button {
                    class: "w-10 h-10 rounded-full flex items-center justify-center shadow-sm border apple-tap",
                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    title: "Kembali",
                    aria_label: "Kembali ke rak",
                    onclick: move |_| on_back.call(()),
                    svg { class: "w-4 h-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                }
                div { class: "flex-1 min-w-0 text-center",
                    p { class: "font-serif text-sm font-semibold truncate", "{book.title}" }
                    p { class: "font-sans text-[11px] opacity-60", if total > 0 { "Hal. {page} / {total}" } else { "Memuat PDF..." } }
                }
                div { class: "flex items-center gap-1",
                    button {
                        class: "w-9 h-9 rounded-full border flex items-center justify-center apple-tap",
                        style: "border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        title: "Perkecil",
                        aria_label: "Perkecil",
                        onclick: move |_| {
                        let next = (*zoom.peek()).saturating_sub(10).max(60);
                        zoom.set(next);
                    },
                        "-"
                    }
                    button {
                        class: "w-9 h-9 rounded-full border flex items-center justify-center apple-tap",
                        style: "border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        title: "Perbesar",
                        aria_label: "Perbesar",
                        onclick: move |_| {
                        let next = (*zoom.peek() + 10).min(300);
                        zoom.set(next);
                    },
                        "+"
                    }
                }
            }

            div {
                class: "flex-1 overflow-y-auto overscroll-contain px-3 py-4 flex flex-col items-center",
                if let Some(msg) = error.read().as_ref() {
                    div {
                        class: "m-auto max-w-sm text-center flex flex-col gap-3 p-6 rounded-2xl border",
                        style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",
                        p { class: "font-sans text-sm", "{msg}" }
                        button {
                            class: "px-4 py-2 rounded-full text-xs font-sans font-medium border apple-tap",
                            style: "border-color: {theme.border_color()}; color: {theme.text_primary()};",
                            onclick: move |_| on_back.call(()),
                            "Kembali ke rak"
                        }
                    }
                } else {
                    canvas {
                        id: "baca-pdf-canvas",
                        class: "max-w-full rounded-lg shadow-lg bg-white",
                    }
                }
            }

            div {
                class: "flex items-center justify-between gap-4 px-6 pb-4 pt-3 border-t",
                style: "padding-bottom: max(calc(env(safe-area-inset-bottom, 0px) + 1rem), 1rem); background-color: {theme.bg_color()}F0; border-color: {theme.border_color()}60;",
                button {
                    class: "px-4 py-2.5 rounded-full text-xs font-sans font-medium border apple-tap disabled:opacity-40",
                    style: "border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    disabled: !can_prev,
                    aria_label: "Halaman sebelumnya",
                    onclick: move |_| {
                        let next = (*current_page.peek()).saturating_sub(1).max(1);
                        current_page.set(next);
                    },
                    "Sebelumnya"
                }
                span { class: "font-sans text-xs opacity-60", if total > 0 { "Hal. {page} / {total}" } else { "—" } }
                button {
                    class: "px-4 py-2.5 rounded-full text-xs font-sans font-medium border apple-tap disabled:opacity-40",
                    style: "border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    disabled: !can_next,
                    aria_label: "Halaman berikutnya",
                    onclick: move |_| {
                        let total = *total_pages.peek();
                        if total > 0 {
                            let next = (*current_page.peek() + 1).min(total);
                            current_page.set(next);
                        }
                    },
                    "Berikutnya"
                }
            }
        }
    }
}
