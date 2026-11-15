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
    if !is_open { return rsx! {}; }

    let mut query = use_signal(|| initial_text.clone());
    let mut explanation = use_signal(|| None::<String>);
    let mut is_loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);
    let mut key_input = use_signal(|| api_key.clone().unwrap_or_default());
    let mut is_editing_key = use_signal(|| api_key.as_ref().map_or(true, |k| k.trim().is_empty()));

    let mut do_ask = {
        let model_str = model.clone().unwrap_or_else(|| "gemini-2.5-flash".to_string());
        move || {
            let (term, key) = (query().trim().to_string(), key_input().trim().to_string());
            if term.is_empty() { return; }
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
            class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-sm animate-fade-in",
            onclick: move |_| on_close.call(()),

            div {
                class: "w-full max-w-lg max-h-[85vh] rounded-[28px] border shadow-2xl flex flex-col overflow-hidden animate-slide-up",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "p-5 border-b flex items-center justify-between",
                    style: "border-color: {theme.border_color()};",
                    div { class: "flex items-center gap-2.5",
                        div { class: "w-8 h-8 rounded-full flex items-center justify-center text-amber-500", style: "background-color: {theme.surface_subtle()};",
                            svg { class: "w-4 h-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z" }
                            }
                        }
                        div {
                            h3 { class: "font-serif text-lg font-medium", "Asisten AI Baca." }
                            p { class: "font-sans text-[11px] opacity-60", "Didukung Google Gemini" }
                        }
                    }
                    button { class: "w-8 h-8 rounded-full flex items-center justify-center apple-tap opacity-60 hover:opacity-100", style: "background-color: {theme.surface_subtle()};", onclick: move |_| on_close.call(()), "✕" }
                }

                // Body
                div {
                    class: "p-5 flex-1 overflow-y-auto space-y-4",

                    if *is_editing_key.read() {
                        div {
                            class: "p-4 rounded-2xl border text-xs font-sans space-y-2.5",
                            style: "background-color: {theme.surface_subtle()}; border-color: {theme.border_color()};",
                            div { class: "flex items-center justify-between",
                                span { class: "font-semibold", "Google Gemini API Key" }
                                a { class: "text-[11px] underline opacity-70 hover:opacity-100", href: "https://aistudio.google.com/app/apikey", target: "_blank", "Dapatkan Kunci Gratis ↗" }
                            }
                            div { class: "flex gap-2",
                                input { r#type: "password", class: "flex-1 px-3 py-2 rounded-xl border text-xs outline-none bg-white dark:bg-stone-900", style: "border-color: {theme.border_color()};", placeholder: "AIzaSy...", value: "{key_input}", oninput: move |e| key_input.set(e.value()) }
                                button { class: "px-4 py-2 rounded-xl text-xs font-medium apple-tap shadow-sm", style: "background-color: {theme.accent_color()}; color: {theme.accent_contrast_text()};",
                                    onclick: move |_| { on_save_key.call(key_input().trim().to_string()); is_editing_key.set(false); }, "Simpan"
                                }
                            }
                        }
                    }

                    div { class: "space-y-1.5",
                        div { class: "flex items-center justify-between",
                            label { class: "font-sans text-[11px] font-semibold uppercase tracking-wider opacity-60", "Kata atau Kutipan" }
                            if !*is_editing_key.read() {
                                button { class: "font-sans text-[11px] opacity-50 hover:opacity-100 underline", onclick: move |_| is_editing_key.set(true), "Ganti API Key" }
                            }
                        }
                        textarea { class: "w-full p-3.5 rounded-2xl border text-sm font-sans outline-none resize-none transition-all", style: "background-color: {theme.surface_subtle()}; border-color: {theme.border_color()}; color: {theme.text_primary()};", rows: "3", placeholder: "Masukkan kata atau kalimat yang ingin dipahami...", value: "{query}", oninput: move |e| query.set(e.value()) }
                    }

                    button {
                        class: "w-full py-3 rounded-2xl text-xs font-sans font-semibold flex items-center justify-center gap-2 shadow-sm apple-tap disabled:opacity-40",
                        style: "background-color: {theme.accent_color()}; color: {theme.accent_contrast_text()};",
                        disabled: *is_loading.read() || query().trim().is_empty(),
                        onclick: move |_| do_ask(),
                        if *is_loading.read() {
                            div { class: "w-4 h-4 border-2 border-t-transparent border-white rounded-full animate-spin" }
                            span { "Menganalisis..." }
                        } else {
                            span { "Jelaskan dengan AI ✨" }
                        }
                    }

                    if let Some(err) = error_msg.read().as_ref() {
                        div { class: "p-3.5 rounded-2xl border text-xs text-rose-500 font-sans leading-relaxed animate-fade-in bg-rose-500/10 border-rose-500/20", "{err}" }
                    }

                    if let Some(res) = explanation.read().as_ref() {
                        div { class: "p-5 rounded-2xl border text-sm font-serif leading-relaxed animate-slide-up whitespace-pre-wrap select-text", style: "background-color: {theme.surface_subtle()}; border-color: {theme.border_color()}; color: {theme.text_primary()};", "{res}" }
                    }
                }
            }
        }
    }
}
