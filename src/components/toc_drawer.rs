use dioxus::prelude::*;
use crate::engine::TocItem;
use crate::theme::ThemeMode;

#[component]
pub fn TocDrawer(
    toc: Vec<TocItem>,
    current_index: usize,
    theme: ThemeMode,
    is_open: bool,
    on_close: EventHandler<()>,
    on_select: EventHandler<usize>,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    rsx! {
        // Frosted Backdrop overlay
        div {
            class: "fixed inset-0 z-40 bg-black/40 backdrop-blur-sm animate-fade-in",
            onclick: move |_| on_close.call(()),
        }

        // Apple Side Drawer with Spring Motion
        div {
            class: "fixed inset-y-0 left-0 z-50 w-84 max-w-[85vw] shadow-2xl flex flex-col border-r animate-slide-up transition-all",
            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()}; box-shadow: {theme.card_shadow()};",

            // Drawer Header
            div {
                class: "flex items-center justify-between p-6 border-b",
                style: "border-color: {theme.border_color()};",
                div {
                    h2 { class: "font-serif text-xl font-medium", "Daftar Isi" }
                    p { class: "font-sans text-[11px] opacity-60", "{toc.len()} Bab Tersedia" }
                }
                button {
                    class: "w-8 h-8 rounded-full flex items-center justify-center apple-tap opacity-60 hover:opacity-100 transition-all",
                    style: "background-color: {theme.surface_subtle()}; color: {theme.text_primary()};",
                    onclick: move |_| on_close.call(()),
                    svg {
                        class: "w-4 h-4",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        line { x1: "18", y1: "6", x2: "6", y2: "18" }
                        line { x1: "6", y1: "6", x2: "18", y2: "18" }
                    }
                }
            }

            // Chapter List
            div {
                class: "flex-1 overflow-y-auto p-4 space-y-1.5",
                for item in toc {
                    {
                        let idx = item.chapter_index;
                        let is_active = idx == current_index;
                        rsx! {
                            button {
                                key: "{idx}",
                                class: "w-full text-left px-4 py-3 rounded-2xl text-[13px] font-sans transition-all flex items-center justify-between apple-tap shadow-sm",
                                style: if is_active {
                                    format!("background-color: {}; color: {}; font-weight: 600;", theme.accent_color(), theme.accent_contrast_text())
                                } else {
                                    format!("background-color: transparent; color: {};", theme.text_primary())
                                },
                                onclick: move |_| {
                                    on_select.call(idx);
                                    on_close.call(());
                                },
                                span {
                                    class: "truncate mr-2 font-serif text-[14px]",
                                    "{item.title}"
                                }
                                if is_active {
                                    span {
                                        class: "text-[10px] uppercase font-sans tracking-wider px-2 py-0.5 rounded-full",
                                        style: "background-color: rgba(255,255,255,0.2);",
                                        "Aktif"
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
