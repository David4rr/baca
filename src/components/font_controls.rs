use dioxus::prelude::*;
use crate::db::models::UserSettings;
use crate::theme::ThemeMode;

#[component]
pub fn FontControls(
    settings: UserSettings,
    theme: ThemeMode,
    is_open: bool,
    on_close: EventHandler<()>,
    on_change_settings: EventHandler<UserSettings>,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    let current = settings.clone();

    rsx! {
        // Overlay
        div {
            class: "fixed inset-0 z-40 bg-black/40 backdrop-blur-xs",
            onclick: move |_| on_close.call(()),
        }

        // Modal Sheet
        div {
            class: "fixed bottom-6 right-6 z-50 w-88 max-w-[92vw] p-5 rounded-2xl shadow-2xl border flex flex-col gap-4",
            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",

            div {
                class: "flex items-center justify-between border-b pb-3",
                style: "border-color: {theme.border_color()};",
                span { class: "font-semibold text-sm", "Typography & Theme" }
                button {
                    class: "text-lg opacity-60 hover:opacity-100",
                    onclick: move |_| on_close.call(()),
                    "✕"
                }
            }

            // Theme Mode Row
            div {
                class: "flex flex-col gap-2",
                span { class: "text-xs font-medium opacity-70", "PALETTE" }
                div {
                    class: "grid grid-cols-4 gap-2",
                    for (t_name, t_val) in [("AMOLED", "amoled"), ("Warm", "warm"), ("Slate", "slate"), ("Light", "light")] {
                        button {
                            key: "{t_val}",
                            class: "py-1.5 px-2 rounded-xl text-xs font-semibold text-center border transition-all",
                            style: if current.theme == t_val {
                                format!("border-color: {}; background-color: {}; color: #FFF;", theme.accent_color(), theme.accent_color())
                            } else {
                                format!("border-color: {};", theme.border_color())
                            },
                            onclick: {
                                let mut s = current.clone();
                                let t = t_val.to_string();
                                move |_| {
                                    s.theme = t.clone();
                                    on_change_settings.call(s.clone());
                                }
                            },
                            "{t_name}"
                        }
                    }
                }
            }

            // Font Family Row
            div {
                class: "flex flex-col gap-2",
                span { class: "text-xs font-medium opacity-70", "TYPEFACE" }
                div {
                    class: "grid grid-cols-3 gap-2",
                    for (f_label, f_val) in [("Serif", "serif"), ("Sans", "sans"), ("Mono", "mono")] {
                        button {
                            key: "{f_val}",
                            class: "py-1.5 px-2 rounded-xl text-xs font-semibold border transition-all",
                            style: if current.font_family == f_val {
                                format!("border-color: {}; background-color: {}; color: #FFF;", theme.accent_color(), theme.accent_color())
                            } else {
                                format!("border-color: {};", theme.border_color())
                            },
                            onclick: {
                                let mut s = current.clone();
                                let f = f_val.to_string();
                                move |_| {
                                    s.font_family = f.clone();
                                    on_change_settings.call(s.clone());
                                }
                            },
                            "{f_label}"
                        }
                    }
                }
            }

            // Font Sizing Controls
            div {
                class: "flex items-center justify-between pt-1",
                span { class: "text-xs font-medium opacity-70", "SIZE ({current.font_size}px)" }
                div {
                    class: "flex items-center gap-2",
                    button {
                        class: "w-8 h-8 rounded-lg border font-bold flex items-center justify-center hover:bg-white/10",
                        style: "border-color: {theme.border_color()};",
                        onclick: {
                            let mut s = current.clone();
                            move |_| {
                                if s.font_size > 12 {
                                    s.font_size -= 1;
                                    on_change_settings.call(s.clone());
                                }
                            }
                        },
                        "A-"
                    }
                    button {
                        class: "w-8 h-8 rounded-lg border font-bold flex items-center justify-center hover:bg-white/10",
                        style: "border-color: {theme.border_color()};",
                        onclick: {
                            let mut s = current.clone();
                            move |_| {
                                if s.font_size < 36 {
                                    s.font_size += 1;
                                    on_change_settings.call(s.clone());
                                }
                            }
                        },
                        "A+"
                    }
                }
            }
        }
    }
}
