use dioxus::prelude::*;
use crate::db::models::UserSettings;
use crate::theme::ThemeMode;

#[component]
pub fn BackupModal(
    theme: ThemeMode,
    settings: UserSettings,
    is_open: bool,
    on_close: EventHandler<()>,
    on_export_json: EventHandler<()>,
    on_restore_json: EventHandler<String>,
    on_save_settings: EventHandler<UserSettings>,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    let mut tab = use_signal(|| "backup".to_string());
    let mut restore_text = use_signal(|| "".to_string());
    let mut api_key_val = use_signal(|| settings.gemini_api_key.clone().unwrap_or_default());
    let mut model_val = use_signal(|| settings.gemini_model.clone().unwrap_or_else(|| "gemini-2.5-flash".to_string()));
    let mut status_msg = use_signal(|| None::<String>);

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
                        span { class: "text-lg", "⚙️" }
                        h3 { class: "font-bold text-sm", "Pengaturan & Backup Data" }
                    }
                    button {
                        class: "p-1.5 rounded-full hover:bg-white/10 opacity-60 hover:opacity-100 text-sm font-bold",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                // Tabs
                div {
                    class: "flex border-b text-xs font-bold",
                    style: "border-color: {theme.border_color()};",
                    button {
                        class: "flex-1 py-2.5 text-center transition-colors border-b-2",
                        style: if tab() == "backup" {
                            format!("border-color: {}; color: {};", theme.accent_color(), theme.accent_color())
                        } else {
                            "border-color: transparent; opacity: 0.6;".to_string()
                        },
                        onclick: move |_| tab.set("backup".to_string()),
                        "💾 Backup & Restore"
                    }
                    button {
                        class: "flex-1 py-2.5 text-center transition-colors border-b-2",
                        style: if tab() == "ai" {
                            format!("border-color: {}; color: {};", theme.accent_color(), theme.accent_color())
                        } else {
                            "border-color: transparent; opacity: 0.6;".to_string()
                        },
                        onclick: move |_| tab.set("ai".to_string()),
                        "✨ Konfigurasi Gemini AI"
                    }
                }

                // Status Message Banner
                if let Some(msg) = status_msg() {
                    div {
                        class: "p-3 mx-4 mt-3 rounded-xl bg-teal-500/10 border border-teal-500/20 text-teal-300 text-xs flex justify-between",
                        span { "{msg}" }
                        button { class: "font-bold opacity-70", onclick: move |_| status_msg.set(None), "✕" }
                    }
                }

                // Content
                div {
                    class: "p-5 flex-1 overflow-y-auto space-y-4",

                    if tab() == "backup" {
                        div {
                            class: "space-y-4",
                            // Export Section
                            div {
                                class: "p-4 rounded-2xl bg-black/20 border flex flex-col gap-2",
                                style: "border-color: {theme.border_color()};",
                                h4 { class: "text-sm font-bold", "Ekspor Cadangan Koleksi" }
                                p { class: "text-xs opacity-60 leading-relaxed", "Unduh cadangan data rak buku, bookmark, dan pengaturan dalam format JSON terenkripsi lokal." }
                                button {
                                    class: "mt-1 py-2 px-4 rounded-xl text-xs font-bold self-start transition-all",
                                    style: format!("background-color: {}; color: #000;", theme.accent_color()),
                                    onclick: move |_| {
                                        on_export_json.call(());
                                        status_msg.set(Some("Cadangan JSON berhasil diekspor!".to_string()));
                                    },
                                    "📥 Ekspor Data JSON"
                                }
                            }

                            // Restore Section
                            div {
                                class: "p-4 rounded-2xl bg-black/20 border flex flex-col gap-2",
                                style: "border-color: {theme.border_color()};",
                                h4 { class: "text-sm font-bold", "Pulihkan Data dari Cadangan" }
                                p { class: "text-xs opacity-60", "Tempelkan teks data JSON cadangan Anda di bawah ini:" }
                                textarea {
                                    class: "w-full h-24 p-2 text-xs rounded-xl bg-black/40 border outline-none font-mono",
                                    style: "border-color: {theme.border_color()};",
                                    placeholder: "Tempelkan teks data JSON cadangan Anda di sini...",
                                    value: "{restore_text}",
                                    oninput: move |e| restore_text.set(e.value()),
                                }
                                button {
                                    class: "py-2 px-4 rounded-xl text-xs font-bold self-start transition-all border border-white/20 hover:bg-white/10",
                                    onclick: move |_| {
                                        let text = restore_text().trim().to_string();
                                        if text.is_empty() {
                                            status_msg.set(Some("Silakan tempel teks JSON terlebih dahulu.".to_string()));
                                            return;
                                        }
                                        on_restore_json.call(text);
                                        restore_text.set("".to_string());
                                        status_msg.set(Some("Data perpustakaan berhasil dipulihkan!".to_string()));
                                    },
                                    "📤 Pulihkan Koleksi"
                                }
                            }
                        }
                    } else if tab() == "ai" {
                        div {
                            class: "space-y-4",
                            div {
                                class: "flex flex-col gap-1.5",
                                label { class: "text-xs font-bold opacity-80", "Gemini API Key" }
                                input {
                                    r#type: "password",
                                    class: "w-full px-3 py-2 text-xs rounded-xl border bg-black/20 outline-none",
                                    style: "border-color: {theme.border_color()};",
                                    placeholder: "AIzaSy...",
                                    value: "{api_key_val}",
                                    oninput: move |e| api_key_val.set(e.value()),
                                }
                                p { class: "text-[10px] opacity-50", "Dapatkan API Key gratis di aistudio.google.com" }
                            }

                            div {
                                class: "flex flex-col gap-1.5",
                                label { class: "text-xs font-bold opacity-80", "Pilihan Model AI" }
                                select {
                                    class: "w-full px-3 py-2 text-xs rounded-xl border bg-black/40 outline-none",
                                    style: "border-color: {theme.border_color()}; color: {theme.text_primary()};",
                                    value: "{model_val}",
                                    onchange: move |e| model_val.set(e.value()),
                                    option { value: "gemini-2.5-flash", "Gemini 2.5 Flash (Tercepat & Cerdas)" }
                                    option { value: "gemini-1.5-flash", "Gemini 1.5 Flash (Stabil)" }
                                    option { value: "gemini-1.5-pro", "Gemini 1.5 Pro (Paling Akurat)" }
                                }
                            }

                            button {
                                class: "w-full py-2.5 rounded-xl text-xs font-bold transition-all mt-4",
                                style: format!("background-color: {}; color: #000;", theme.accent_color()),
                                onclick: {
                                    let mut s = settings.clone();
                                    move |_| {
                                        s.gemini_api_key = Some(api_key_val().trim().to_string());
                                        s.gemini_model = Some(model_val().trim().to_string());
                                        on_save_settings.call(s.clone());
                                        status_msg.set(Some("Konfigurasi AI berhasil disimpan!".to_string()));
                                    }
                                },
                                "Simpan Pengaturan AI"
                            }
                        }
                    }
                }
            }
        }
    }
}
