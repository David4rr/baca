use dioxus::prelude::*;
use crate::engine::ask_gemini;
use crate::theme::ThemeMode;

#[component]
pub fn AiExplanationModal(
    theme: ThemeMode,
    initial_text: String,
    api_key: Option<String>,
    model: Option<String>,
    is_open: bool,
    on_close: EventHandler<()>,
    on_save_key: EventHandler<String>,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    let mut query = use_signal(|| initial_text.clone());
    let mut explanation = use_signal(|| None::<String>);
    let mut is_loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);
    let mut key_input = use_signal(|| api_key.clone().unwrap_or_default());
    let mut is_editing_key = use_signal(|| api_key.as_ref().map_or(true, |k| k.trim().is_empty()));

    let mut do_ask = {
        let model_str = model.clone().unwrap_or_else(|| "gemini-2.5-flash".to_string());
        move || {
            let term = query().trim().to_string();
            let key = key_input().trim().to_string();
            if term.is_empty() {
                return;
            }
            if key.is_empty() {
                is_editing_key.set(true);
                error_msg.set(Some("Silakan masukkan Gemini API Key Anda terlebih dahulu.".to_string()));
                return;
            }

            let m = model_str.clone();
            spawn(async move {
                is_loading.set(true);
                error_msg.set(None);
                explanation.set(None);

                match ask_gemini(&key, &m, &term).await {
                    Ok(text) => explanation.set(Some(text)),
                    Err(e) => error_msg.set(Some(e)),
                }
                is_loading.set(false);
            });
        }
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-fade-in",
            onclick: move |_| on_close.call(()),

            div {
                class: "w-full max-w-lg max-h-[85vh] rounded-3xl border shadow-2xl flex flex-col overflow-hidden",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "p-4 border-b flex items-center justify-between",
                    style: "border-color: {theme.border_color()};",
                    div {
                        class: "flex items-center gap-2",
                        span { class: "text-lg", "✨" }
                        h3 { class: "font-bold text-sm", "AI Explanation (Gemini)" }
                    }
                    div {
                        class: "flex items-center gap-2",
                        button {
                            class: "text-xs opacity-60 hover:opacity-100 font-medium px-2 py-1 rounded-lg border border-white/10",
                            onclick: move |_| {
                                let cur = *is_editing_key.read();
                                is_editing_key.set(!cur);
                            },
                            "⚙ API Key"
                        }
                        button {
                            class: "p-1.5 rounded-full hover:bg-white/10 opacity-60 hover:opacity-100 text-sm font-bold",
                            onclick: move |_| on_close.call(()),
                            "✕"
                        }
                    }
                }

                // API Key Config Panel (if toggled or missing)
                if *is_editing_key.read() {
                    div {
                        class: "p-4 bg-black/20 border-b flex flex-col gap-2",
                        style: "border-color: {theme.border_color()};",
                        p { class: "text-xs opacity-70", "Masukkan Gemini API Key gratis Anda dari Google AI Studio:" }
                        div {
                            class: "flex gap-2",
                            input {
                                r#type: "password",
                                class: "flex-1 px-3 py-1.5 text-xs rounded-xl border bg-black/30 outline-none",
                                style: "border-color: {theme.border_color()};",
                                placeholder: "AIzaSy...",
                                value: "{key_input}",
                                oninput: move |e| key_input.set(e.value()),
                            }
                            button {
                                class: "px-3 py-1.5 rounded-xl text-xs font-bold transition-all",
                                style: format!("background-color: {}; color: #000;", theme.accent_color()),
                                onclick: move |_| {
                                    on_save_key.call(key_input().trim().to_string());
                                    is_editing_key.set(false);
                                },
                                "Simpan"
                            }
                        }
                    }
                }

                // Query Input Box
                div {
                    class: "p-4 border-b flex gap-2",
                    style: "border-color: {theme.border_color()};",
                    input {
                        r#type: "text",
                        class: "flex-1 px-3 py-2 text-sm rounded-xl border bg-black/10 outline-none font-medium",
                        style: "border-color: {theme.border_color()};",
                        placeholder: "Ketik kata atau kalimat untuk dijelaskan...",
                        value: "{query}",
                        oninput: move |e| query.set(e.value()),
                    }
                    button {
                        class: "px-4 py-2 rounded-xl text-xs font-bold shrink-0 transition-all flex items-center gap-1.5",
                        style: format!("background-color: {}; color: #000;", theme.accent_color()),
                        disabled: *is_loading.read(),
                        onclick: move |_| do_ask(),
                        if *is_loading.read() { "⏳" } else { "Tanya AI" }
                    }
                }

                // Body / Result Content
                div {
                    class: "p-5 flex-1 overflow-y-auto min-h-[160px] text-sm leading-relaxed",
                    if *is_loading.read() {
                        div {
                            class: "flex flex-col items-center justify-center py-12 gap-3 opacity-60",
                            div { class: "w-6 h-6 border-2 border-white/20 border-t-white rounded-full animate-spin" }
                            span { class: "text-xs", "Gemini sedang menganalisis teks..." }
                        }
                    } else if let Some(err) = error_msg.read().as_ref() {
                        div {
                            class: "p-4 rounded-xl bg-rose-500/10 border border-rose-500/20 text-rose-300 text-xs",
                            "{err}"
                        }
                    } else if let Some(text) = explanation.read().as_ref() {
                        div {
                            class: "prose prose-invert max-w-none text-xs sm:text-sm leading-relaxed whitespace-pre-line",
                            "{text}"
                        }
                    } else {
                        div {
                            class: "flex flex-col items-center justify-center py-10 opacity-40 text-center gap-1",
                            p { class: "text-xs font-semibold", "Pilih teks di buku atau ketik kata di atas" }
                            p { class: "text-[11px]", "Gemini akan menjelaskan definisi, makna filosofis, dan konteksnya." }
                        }
                    }
                }
            }
        }
    }
}
