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
            class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-sm animate-fade-in",
            onclick: move |_| on_close.call(()),

            div {
                class: "w-full max-w-lg max-h-[85vh] rounded-[28px] border shadow-2xl flex flex-col overflow-hidden animate-slide-up",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()}; box-shadow: {theme.card_shadow()};",
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "p-5 border-b flex items-center justify-between",
                    style: "border-color: {theme.border_color()};",
                    div {
                        class: "flex items-center gap-2.5",
                        div {
                            class: "w-8 h-8 rounded-full flex items-center justify-center",
                            style: "width: 32px; height: 32px; min-width: 32px; min-height: 32px; background-color: {theme.surface_subtle()}; color: {theme.text_primary()};",
                            svg {
                                class: "w-4 h-4",
                                width: "16",
                                height: "16",
                                style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                line { x1: "4", y1: "21", x2: "4", y2: "14" }
                                line { x1: "4", y1: "10", x2: "4", y2: "3" }
                                line { x1: "12", y1: "21", x2: "12", y2: "12" }
                                line { x1: "12", y1: "8", x2: "12", y2: "3" }
                                line { x1: "20", y1: "21", x2: "20", y2: "16" }
                                line { x1: "20", y1: "12", x2: "20", y2: "3" }
                                line { x1: "1", y1: "14", x2: "7", y2: "14" }
                                line { x1: "9", y1: "8", x2: "15", y2: "8" }
                                line { x1: "17", y1: "16", x2: "23", y2: "16" }
                            }
                        }
                        div {
                            h3 { class: "font-serif text-lg font-medium", "Pengaturan & Cadangan" }
                            p { class: "font-sans text-[11px] opacity-60", "Kelola data & preferensi aplikasi" }
                        }
                    }
                    button {
                        class: "w-8 h-8 rounded-full flex items-center justify-center apple-tap opacity-60 hover:opacity-100 transition-all",
                        style: "width: 32px; height: 32px; min-width: 32px; min-height: 32px; background-color: {theme.surface_subtle()}; color: {theme.text_primary()};",
                        onclick: move |_| on_close.call(()),
                        svg {
                            class: "w-4 h-4",
                            width: "16",
                            height: "16",
                            style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            line { x1: "18", y1: "6", x2: "6", y2: "18" }
                            line { x1: "6", y1: "6", x2: "18", y2: "18" }
                        }
                    }
                }

                // Apple Segmented Tabs
                div {
                    class: "p-3 border-b flex gap-2",
                    style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",
                    button {
                        class: "flex-1 py-2 text-center rounded-xl text-xs font-sans font-medium apple-tap transition-all shadow-sm",
                        style: if tab() == "backup" {
                            format!("background-color: {}; color: {};", theme.accent_color(), theme.accent_contrast_text())
                        } else {
                            format!("background-color: transparent; color: {};", theme.text_secondary())
                        },
                        onclick: move |_| tab.set("backup".to_string()),
                        "Cadangkan & Pulihkan"
                    }
                    button {
                        class: "flex-1 py-2 text-center rounded-xl text-xs font-sans font-medium apple-tap transition-all shadow-sm",
                        style: if tab() == "ai" {
                            format!("background-color: {}; color: {};", theme.accent_color(), theme.accent_contrast_text())
                        } else {
                            format!("background-color: transparent; color: {};", theme.text_secondary())
                        },
                        onclick: move |_| tab.set("ai".to_string()),
                        "Konfigurasi AI Gemini"
                    }
                }

                // Content
                div {
                    class: "p-6 flex-1 overflow-y-auto space-y-5",

                    if tab() == "backup" {
                        div {
                            class: "space-y-5 animate-fade-in",

                            // Export Block
                            div {
                                class: "p-4 rounded-2xl border flex flex-col gap-3",
                                style: "background-color: {theme.surface_subtle()}; border-color: {theme.border_color()};",
                                div {
                                    h4 { class: "font-serif text-sm font-semibold", "Ekspor Cadangan Pustaka" }
                                    p { class: "font-sans text-xs opacity-70 leading-relaxed", "Simpan seluruh daftar buku, posisi membaca, catatan bookmark, dan statistik ke berkas JSON lokal." }
                                }
                                button {
                                    class: "py-2.5 px-4 rounded-xl text-xs font-sans font-semibold flex items-center justify-center gap-2 shadow-sm apple-tap transition-all",
                                    style: "background-color: {theme.accent_color()}; color: {theme.accent_contrast_text()};",
                                    onclick: move |_| {
                                        on_export_json.call(());
                                        status_msg.set(Some("Cadangan JSON berhasil diekspor ke folder aplikasi!".to_string()));
                                    },
                                    svg {
                                        class: "w-3.5 h-3.5",
                                        width: "14",
                                        height: "14",
                                        style: "width: 14px; height: 14px; min-width: 14px; min-height: 14px;",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                        polyline { points: "7 10 12 15 17 10" }
                                        line { x1: "12", y1: "15", x2: "12", y2: "3" }
                                    }
                                    span { "Ekspor Berkas Cadangan (.JSON)" }
                                }
                            }

                            // Restore Block
                            div {
                                class: "p-4 rounded-2xl border flex flex-col gap-3",
                                style: "background-color: {theme.surface_subtle()}; border-color: {theme.border_color()};",
                                div {
                                    h4 { class: "font-serif text-sm font-semibold", "Pulihkan Data Cadangan" }
                                    p { class: "font-sans text-xs opacity-70 leading-relaxed", "Tempelkan teks data JSON cadangan Anda untuk memulihkan seluruh koleksi rak dan progres membaca." }
                                }
                                textarea {
                                    class: "w-full p-3 rounded-xl border text-xs font-mono outline-none resize-none bg-white dark:bg-stone-900",
                                    style: "border-color: {theme.border_color()};",
                                    rows: "4",
                                    placeholder: "Tempelkan teks data JSON cadangan Anda di sini...",
                                    value: "{restore_text}",
                                    oninput: move |e| restore_text.set(e.value()),
                                }
                                button {
                                    class: "py-2.5 px-4 rounded-xl text-xs font-sans font-semibold border apple-tap transition-all shadow-sm disabled:opacity-40 disabled:pointer-events-none",
                                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                                    disabled: restore_text().trim().is_empty(),
                                    onclick: move |_| {
                                        let text = restore_text().trim().to_string();
                                        on_restore_json.call(text);
                                        restore_text.set("".to_string());
                                        status_msg.set(Some("Data cadangan berhasil dipulihkan!".to_string()));
                                    },
                                    "Pulihkan Data dari Teks"
                                }
                            }
                        }
                    } else {
                        div {
                            class: "space-y-4 animate-fade-in",
                            div {
                                class: "p-4 rounded-2xl border space-y-3",
                                style: "background-color: {theme.surface_subtle()}; border-color: {theme.border_color()};",
                                div {
                                    h4 { class: "font-serif text-sm font-semibold", "Pengaturan API Key Gemini" }
                                    p { class: "font-sans text-xs opacity-70 leading-relaxed", "Kunci API disimpan secara lokal di basis data SQLite pada perangkat Anda." }
                                }
                                input {
                                    r#type: "password",
                                    class: "w-full px-3.5 py-2.5 rounded-xl border text-xs font-mono outline-none bg-white dark:bg-stone-900",
                                    style: "border-color: {theme.border_color()};",
                                    placeholder: "AIzaSy...",
                                    value: "{api_key_val}",
                                    oninput: move |e| api_key_val.set(e.value()),
                                }
                            }

                            div {
                                class: "p-4 rounded-2xl border space-y-3",
                                style: "background-color: {theme.surface_subtle()}; border-color: {theme.border_color()};",
                                div {
                                    h4 { class: "font-serif text-sm font-semibold", "Pilihan Model AI" }
                                    p { class: "font-sans text-xs opacity-70 leading-relaxed", "Pilih model kecerdasan buatan Google untuk kecepatan atau penalaran mendalam." }
                                }
                                select {
                                    class: "w-full px-3.5 py-2.5 rounded-xl border text-xs font-sans outline-none bg-white dark:bg-stone-900",
                                    style: "border-color: {theme.border_color()};",
                                    value: "{model_val}",
                                    onchange: move |e| model_val.set(e.value()),
                                    option { value: "gemini-2.5-flash", "Gemini 2.5 Flash (Tercepat & Ringan - Rekomendasi)" }
                                    option { value: "gemini-1.5-pro", "Gemini 1.5 Pro (Penalaran Mendalam)" }
                                }
                            }

                            button {
                                class: "w-full py-3 rounded-2xl text-xs font-sans font-semibold shadow-sm apple-tap transition-all",
                                style: "background-color: {theme.accent_color()}; color: {theme.accent_contrast_text()};",
                                onclick: move |_| {
                                    let mut s = settings.clone();
                                    s.gemini_api_key = Some(api_key_val().trim().to_string());
                                    s.gemini_model = Some(model_val().clone());
                                    on_save_settings.call(s);
                                    status_msg.set(Some("Pengaturan Gemini AI berhasil disimpan!".to_string()));
                                },
                                "Simpan Konfigurasi AI"
                            }
                        }
                    }

                    if let Some(msg) = status_msg.read().as_ref() {
                        div {
                            class: "p-3 rounded-xl border text-xs text-emerald-600 font-sans leading-relaxed animate-fade-in",
                            style: "background-color: rgba(16, 185, 129, 0.08); border-color: rgba(16, 185, 129, 0.2);",
                            "{msg}"
                        }
                    }
                }
            }
        }
    }
}
