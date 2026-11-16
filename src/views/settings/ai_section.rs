use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn AiSection(
    api_key: Option<String>,
    model: Option<String>,
    theme: ThemeMode,
    on_save: EventHandler<(String, String)>,
) -> Element {
    let mut current_key = use_signal(|| api_key.unwrap_or_default());
    let mut current_model = use_signal(|| model.unwrap_or_else(|| "gemini-2.5-flash".to_string()));
    let mut show_saved = use_signal(|| false);

    let is_configured = !current_key.read().trim().is_empty();

    rsx! {
        div {
            class: "p-6 rounded-3xl border shadow-sm flex flex-col gap-4",
            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",

            div {
                class: "flex items-center justify-between",
                div {
                    class: "flex flex-col",
                    h3 {
                        class: "font-serif text-lg font-semibold",
                        style: "color: {theme.text_primary()};",
                        "Integrasi AI (Google Gemini)"
                    }
                    span {
                        class: "text-xs font-sans opacity-60 mt-0.5",
                        style: "color: {theme.text_secondary()};",
                        "Jelaskan istilah sulit dan konteks sejarah saat membaca"
                    }
                }
                span {
                    class: "px-2.5 py-1 rounded-full text-[11px] font-sans font-medium",
                    style: if is_configured {
                        "background-color: #22c55e20; color: #16a34a;"
                    } else {
                        "background-color: #ef444420; color: #dc2626;"
                    },
                    if is_configured { "Terhubung" } else { "Belum Diatur" }
                }
            }

            // Input API Key
            div {
                class: "flex flex-col gap-1.5",
                label {
                    class: "text-xs font-sans font-medium opacity-70",
                    style: "color: {theme.text_secondary()};",
                    "Google Gemini API Key"
                }
                input {
                    class: "w-full px-4 py-3 rounded-2xl border text-sm outline-none transition-all",
                    style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    r#type: "password",
                    placeholder: "AIzaSy...",
                    value: "{current_key}",
                    oninput: move |e| current_key.set(e.value()),
                }
            }

            // Model Selection & Save
            div {
                class: "flex items-center justify-between gap-3 pt-1",
                div {
                    class: "flex items-center gap-2",
                    button {
                        class: "px-3 py-1.5 rounded-xl border text-xs font-sans font-medium apple-tap transition-all",
                        style: if *current_model.read() == "gemini-2.5-flash" {
                            format!("background-color: {}; color: {}; border-color: transparent;", theme.accent_color(), theme.accent_contrast_text())
                        } else {
                            format!("background-color: {}; color: {}; border-color: {};", theme.bg_color(), theme.text_secondary(), theme.border_color())
                        },
                        onclick: move |_| current_model.set("gemini-2.5-flash".to_string()),
                        "Gemini 2.5 Flash"
                    }
                    button {
                        class: "px-3 py-1.5 rounded-xl border text-xs font-sans font-medium apple-tap transition-all",
                        style: if *current_model.read() == "gemini-1.5-pro" {
                            format!("background-color: {}; color: {}; border-color: transparent;", theme.accent_color(), theme.accent_contrast_text())
                        } else {
                            format!("background-color: {}; color: {}; border-color: {};", theme.bg_color(), theme.text_secondary(), theme.border_color())
                        },
                        onclick: move |_| current_model.set("gemini-1.5-pro".to_string()),
                        "Gemini 1.5 Pro"
                    }
                }

                button {
                    class: "px-5 py-2 rounded-xl font-sans text-xs font-semibold apple-tap shadow-sm transition-all",
                    style: "background-color: {theme.accent_color()}; color: {theme.accent_contrast_text()};",
                    onclick: move |_| {
                        let k = current_key.read().trim().to_string();
                        let m = current_model.read().clone();
                        on_save.call((k, m));
                        show_saved.set(true);
                    },
                    if *show_saved.read() { "Tersimpan" } else { "Simpan Kunci" }
                }
            }
        }
    }
}
