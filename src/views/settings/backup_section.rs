use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn BackupSection(
    theme: ThemeMode,
    on_export: EventHandler<()>,
    on_restore: EventHandler<String>,
) -> Element {
    let mut restore_json = use_signal(String::new);
    let mut show_restore_input = use_signal(|| false);

    rsx! {
        div {
            class: "p-6 rounded-3xl border shadow-sm flex flex-col gap-4",
            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",

            div {
                class: "flex flex-col",
                h3 {
                    class: "font-serif text-lg font-semibold",
                    style: "color: {theme.text_primary()};",
                    "Cadangkan & Pulihkan Data"
                }
                span {
                    class: "text-xs font-sans opacity-60 mt-0.5",
                    style: "color: {theme.text_secondary()};",
                    "Simpan progres membaca, bookmark, dan pengaturan dalam format JSON"
                }
            }

            div {
                class: "flex items-center gap-3",
                button {
                    class: "flex-1 py-3 px-4 rounded-2xl border font-sans text-xs font-semibold apple-tap shadow-sm transition-all text-center",
                    style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    onclick: move |_| on_export.call(()),
                    "Ekspor Cadangan (JSON)"
                }
                button {
                    class: "flex-1 py-3 px-4 rounded-2xl border font-sans text-xs font-semibold apple-tap shadow-sm transition-all text-center",
                    style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    onclick: move |_| {
                        let cur = *show_restore_input.read();
                        show_restore_input.set(!cur);
                    },
                    "Pulihkan dari JSON"
                }
            }

            if *show_restore_input.read() {
                div {
                    class: "flex flex-col gap-2 pt-2 animate-fade-in",
                    textarea {
                        class: "w-full h-24 p-3 rounded-2xl border font-mono text-xs outline-none transition-all resize-none",
                        style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        placeholder: "Tempel isi file JSON cadangan Anda di sini...",
                        value: "{restore_json}",
                        oninput: move |e| restore_json.set(e.value()),
                    }
                    button {
                        class: "py-2.5 px-4 rounded-xl font-sans text-xs font-bold apple-tap shadow-sm transition-all",
                        style: "background-color: {theme.accent_color()}; color: {theme.accent_contrast_text()};",
                        onclick: move |_| {
                            let text = restore_json.read().trim().to_string();
                            if !text.is_empty() {
                                on_restore.call(text);
                                show_restore_input.set(false);
                            }
                        },
                        "Konfirmasi & Pulihkan"
                    }
                }
            }
        }
    }
}
