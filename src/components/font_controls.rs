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
        // Frosted Glass Backdrop
        div {
            class: "fixed inset-0 z-40 bg-black/30 backdrop-blur-sm animate-fade-in",
            onclick: move |_| on_close.call(()),
        }

        // Apple Bottom/Floating Sheet Modal
        div {
            class: "fixed bottom-6 right-6 z-50 w-96 max-w-[94vw] p-6 rounded-[28px] shadow-2xl border flex flex-col gap-5 animate-slide-up transition-all",
            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()}; box-shadow: {theme.card_shadow()};",

            // Header
            div {
                class: "flex items-center justify-between pb-2 border-b",
                style: "border-color: {theme.border_color()};",
                div {
                    h3 { class: "font-serif text-lg font-medium", "Tipografi & Tampilan" }
                    p { class: "font-sans text-[11px] opacity-60", "Sesuaikan kenyamanan membaca Anda" }
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

            // Theme Mode Row
            div {
                class: "flex flex-col gap-2",
                span { class: "font-sans text-[11px] font-semibold tracking-wider uppercase opacity-50", "TEMA BACA" }
                div {
                    class: "grid grid-cols-4 gap-2",
                    for (t_name, t_val) in [("Warm", "warm"), ("Light", "light"), ("AMOLED", "amoled"), ("Slate", "slate")] {
                        button {
                            key: "{t_val}",
                            class: "py-2 px-2 rounded-xl text-xs font-sans font-medium text-center border apple-tap transition-all shadow-sm",
                            style: if current.theme == t_val {
                                format!("border-color: {}; background-color: {}; color: {};", theme.accent_color(), theme.accent_color(), theme.accent_contrast_text())
                            } else {
                                format!("border-color: {}; background-color: {}; color: {};", theme.border_color(), theme.surface_subtle(), theme.text_secondary())
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
                span { class: "font-sans text-[11px] font-semibold tracking-wider uppercase opacity-50", "JENIS HURUF" }
                div {
                    class: "grid grid-cols-3 gap-2",
                    for (f_label, f_val) in [("Serif", "serif"), ("Sans", "sans"), ("Mono", "mono")] {
                        button {
                            key: "{f_val}",
                            class: "py-2 px-2 rounded-xl text-xs font-medium border apple-tap transition-all shadow-sm",
                            style: if current.font_family == f_val {
                                format!("border-color: {}; background-color: {}; color: {};", theme.accent_color(), theme.accent_color(), theme.accent_contrast_text())
                            } else {
                                format!("border-color: {}; background-color: {}; color: {};", theme.border_color(), theme.surface_subtle(), theme.text_secondary())
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
                class: "flex flex-col gap-2",
                div {
                    class: "flex items-center justify-between text-xs font-sans",
                    span { class: "font-semibold tracking-wider uppercase opacity-50 text-[11px]", "UKURAN TEKS" }
                    span { class: "font-semibold", "{current.font_size}px" }
                }
                div {
                    class: "flex items-center gap-3",
                    button {
                        class: "w-10 h-10 rounded-xl border flex items-center justify-center text-sm font-semibold apple-tap transition-all shadow-sm",
                        style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",
                        disabled: current.font_size <= 12,
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
                    input {
                        r#type: "range",
                        min: "12",
                        max: "32",
                        value: "{current.font_size}",
                        class: "flex-1 accent-stone-900 cursor-pointer h-1.5 rounded-full bg-stone-200",
                        oninput: {
                            let mut s = current.clone();
                            move |e: FormEvent| {
                                if let Ok(sz) = e.value().parse::<u32>() {
                                    s.font_size = sz;
                                    on_change_settings.call(s.clone());
                                }
                            }
                        },
                    }
                    button {
                        class: "w-10 h-10 rounded-xl border flex items-center justify-center text-base font-semibold apple-tap transition-all shadow-sm",
                        style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",
                        disabled: current.font_size >= 32,
                        onclick: {
                            let mut s = current.clone();
                            move |_| {
                                if s.font_size < 32 {
                                    s.font_size += 1;
                                    on_change_settings.call(s.clone());
                                }
                            }
                        },
                        "A+"
                    }
                }
            }

            // Line Height Controls
            div {
                class: "flex flex-col gap-2",
                span { class: "font-sans text-[11px] font-semibold tracking-wider uppercase opacity-50", "JARAK BARIS" }
                div {
                    class: "grid grid-cols-3 gap-2",
                    for (lh_label, lh_val) in [("Rapat", 1.4), ("Ideal", 1.8), ("Longgar", 2.2)] {
                        button {
                            key: "{lh_label}",
                            class: "py-1.5 px-2 rounded-xl text-xs font-sans font-medium border apple-tap transition-all shadow-sm",
                            style: if (current.line_height - lh_val).abs() < 0.1 {
                                format!("border-color: {}; background-color: {}; color: {};", theme.accent_color(), theme.accent_color(), theme.accent_contrast_text())
                            } else {
                                format!("border-color: {}; background-color: {}; color: {};", theme.border_color(), theme.surface_subtle(), theme.text_secondary())
                            },
                            onclick: {
                                let mut s = current.clone();
                                move |_| {
                                    s.line_height = lh_val;
                                    on_change_settings.call(s.clone());
                                }
                            },
                            "{lh_label}"
                        }
                    }
                }
            }
        }
    }
}
