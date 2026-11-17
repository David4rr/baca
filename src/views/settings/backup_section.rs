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
    let mut file_error = use_signal(|| None::<String>);

    let on_restore_file = {
        let on_res = on_restore;
        move |evt: FormEvent| {
            if let Some(engine) = evt.files() {
                if let Some(first) = engine.files().first() {
                    let (f_name, fe) = (first.clone(), engine.clone());
                    let on_r = on_res;
                    spawn(async move {
                        match fe.read_file(&f_name).await {
                            Some(bytes) => match String::from_utf8(bytes) {
                                Ok(text) => {
                                    file_error.set(None);
                                    on_r.call(text);
                                }
                                Err(_) => file_error.set(Some("Berkas cadangan bukan teks UTF-8".to_string())),
                            },
                            None => file_error.set(Some("Berkas cadangan tidak dapat dibaca".to_string())),
                        }
                    });
                }
            }
        }
    };

    rsx! {
        div {
            class: "p-6 rounded-3xl border shadow-sm flex flex-col gap-4",
            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",

            input {
                r#type: "file",
                id: "baca-backup-file-picker",
                accept: ".json,application/json",
                style: "position: fixed; left: -9999px; top: 0; width: 1px; height: 1px; opacity: 0;",
                onchange: on_restore_file,
            }

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
                    "Simpan progres membaca, markah buku, dan pengaturan dalam format JSON"
                }
            }

            div {
                class: "grid grid-cols-1 sm:grid-cols-3 gap-2.5",
                button {
                    class: "py-2.5 px-3 rounded-2xl border font-sans text-xs font-semibold apple-tap shadow-sm transition-all text-center",
                    style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    onclick: move |_| on_export.call(()),
                    "Ekspor (.json)"
                }
                button {
                    class: "py-2.5 px-3 rounded-2xl border font-sans text-xs font-semibold apple-tap shadow-sm transition-all text-center",
                    style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    onclick: move |_| {
                        let _ = document::eval("let el = document.getElementById('baca-backup-file-picker'); if (el) el.click();");
                    },
                    "Pilih Berkas .json"
                }
                button {
                    class: "py-2.5 px-3 rounded-2xl border font-sans text-xs font-semibold apple-tap shadow-sm transition-all text-center opacity-80 hover:opacity-100",
                    style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    onclick: move |_| {
                        let cur = *show_restore_input.read();
                        show_restore_input.set(!cur);
                    },
                    "Tempel Teks"
                }
            }

            if let Some(err) = file_error.read().as_ref() {
                p { class: "text-xs font-sans text-rose-500", "{err}" }
            }

            if *show_restore_input.read() {
                div {
                    class: "flex flex-col gap-2 pt-2 animate-fade-in",
                    textarea {
                        class: "w-full h-24 p-3 rounded-2xl border font-mono text-xs outline-none transition-all resize-none",
                        style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        placeholder: "Tempel isi teks JSON cadangan di sini...",
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
