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
        // Backdrop overlay
        div {
            class: "fixed inset-0 z-40 bg-black/50 backdrop-blur-sm transition-opacity",
            onclick: move |_| on_close.call(()),
        }

        // Side Drawer
        div {
            class: "fixed inset-y-0 left-0 z-50 w-80 max-w-[85vw] shadow-2xl flex flex-col transition-transform transform translate-x-0 border-r",
            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",

            // Drawer Header
            div {
                class: "flex items-center justify-between p-5 border-b",
                style: "border-color: {theme.border_color()};",
                h2 { class: "font-semibold text-base", "Table of Contents" }
                button {
                    class: "p-1 rounded-lg hover:bg-white/10 text-xl leading-none",
                    onclick: move |_| on_close.call(()),
                    "✕"
                }
            }

            // Chapter List
            div {
                class: "flex-1 overflow-y-auto p-3 space-y-1",
                for item in toc {
                    {
                        let idx = item.chapter_index;
                        let is_active = idx == current_index;
                        rsx! {
                            button {
                                key: "{idx}",
                                class: "w-full text-left px-4 py-2.5 rounded-xl text-sm transition-all flex items-center justify-between",
                                style: if is_active {
                                    format!("background-color: {}; color: #FFF; font-weight: 600;", theme.accent_color())
                                } else {
                                    format!("color: {};", theme.text_primary())
                                },
                                onclick: move |_| {
                                    on_select.call(idx);
                                    on_close.call(());
                                },
                                span { class: "truncate pr-2", "{item.title}" }
                                span {
                                    class: "text-xs opacity-60 flex-shrink-0",
                                    "#{idx + 1}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
