use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn ThemeSection(
    current_theme: ThemeMode,
    on_select_theme: EventHandler<String>,
) -> Element {
    let themes = [
        ("warm", "Warm Linen", "#FBF0D9", "#5F4B32"),
        ("light", "Pure Light", "#FFFFFF", "#1C1C1E"),
        ("amoled", "AMOLED Dark", "#000000", "#F2F2F7"),
        ("slate", "Midnight Slate", "#1C1C1E", "#E5E5EA"),
    ];

    rsx! {
        div {
            class: "p-6 rounded-3xl border shadow-sm flex flex-col gap-4",
            style: "background-color: {current_theme.surface_color()}; border-color: {current_theme.border_color()};",

            div {
                class: "flex flex-col",
                h3 {
                    class: "font-serif text-lg font-semibold",
                    style: "color: {current_theme.text_primary()};",
                    "Tema & Tampilan"
                }
                span {
                    class: "text-xs font-sans opacity-60 mt-0.5",
                    style: "color: {current_theme.text_secondary()};",
                    "Pilih palet visual yang nyaman untuk sesi membaca Anda"
                }
            }

            div {
                class: "grid grid-cols-2 sm:grid-cols-4 gap-3 select-none",
                for (id, name, bg, text) in themes {
                    {
                        let id_str = id.to_string();
                        let is_active = current_theme.as_str() == id;
                        let on_select = on_select_theme;

                        rsx! {
                            button {
                                class: "p-3.5 rounded-2xl border flex flex-col items-center gap-2.5 apple-tap transition-all relative",
                                style: if is_active {
                                    format!("background-color: {}; border-color: {}; ring-2 ring-{};", current_theme.bg_color(), current_theme.accent_color(), current_theme.accent_color())
                                } else {
                                    format!("background-color: {}; border-color: {};", current_theme.bg_color(), current_theme.border_color())
                                },
                                onclick: move |_| on_select.call(id_str.clone()),

                                div {
                                    class: "w-10 h-10 rounded-full border shadow-inner flex items-center justify-center font-serif text-xs font-bold",
                                    style: "background-color: {bg}; color: {text}; border-color: {current_theme.border_color()};",
                                    "Aa"
                                }

                                span {
                                    class: "text-xs font-sans font-medium tracking-tight",
                                    style: if is_active {
                                        format!("color: {}; font-weight: 700;", current_theme.accent_color())
                                    } else {
                                        format!("color: {};", current_theme.text_primary())
                                    },
                                    "{name}"
                                }

                                if is_active {
                                    div {
                                        class: "w-2 h-2 rounded-full absolute top-2 right-2",
                                        style: "background-color: {current_theme.accent_color()};",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
