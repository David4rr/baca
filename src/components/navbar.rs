use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn Navbar(
    title: String,
    theme: ThemeMode,
    is_reader: bool,
    is_explore: bool,
    on_back: EventHandler<()>,
    on_explore: EventHandler<()>,
    on_import: EventHandler<()>,
    on_toggle_stats: EventHandler<()>,
    on_toggle_settings: EventHandler<()>,
    on_toggle_search: EventHandler<()>,
    on_toggle_ai: EventHandler<()>,
    on_toggle_toc: EventHandler<()>,
    on_toggle_typography: EventHandler<()>,
) -> Element {
    // If not reader and not explore, the ShelfView has its own native Apple layout
    if !is_reader && !is_explore {
        return rsx! {};
    }

    rsx! {
        header {
            class: "w-full h-14 border-b flex items-center justify-between px-6 select-none sticky top-0 z-30 backdrop-blur-md transition-colors duration-300",
            style: "background-color: {theme.surface_color()}E6; border-color: {theme.border_color()}; color: {theme.text_primary()};",

            // Left Section: Back Button
            div {
                class: "flex items-center gap-3",
                button {
                    class: "w-9 h-9 rounded-full border flex items-center justify-center apple-tap transition-all shadow-sm",
                    style: "width: 36px; height: 36px; min-width: 36px; min-height: 36px; border-color: {theme.border_color()}; background-color: {theme.surface_color()}; color: {theme.text_primary()};",
                    title: "Kembali ke Rak",
                    onclick: move |_| on_back.call(()),
                    svg {
                        class: "w-4 h-4",
                        width: "16",
                        height: "16",
                        style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                }
                span {
                    class: "font-serif text-lg font-medium tracking-tight truncate max-w-xs sm:max-w-md",
                    "{title}"
                }
            }

            // Right Section Actions (if in reader)
            div {
                class: "flex items-center gap-1.5",
                if is_reader {
                    button {
                        class: "w-8 h-8 rounded-full flex items-center justify-center apple-tap opacity-70 hover:opacity-100",
                        style: "width: 32px; height: 32px; min-width: 32px; min-height: 32px;",
                        title: "Cari Teks",
                        onclick: move |_| on_toggle_search.call(()),
                        svg {
                            class: "w-3.5 h-3.5",
                            width: "14",
                            height: "14",
                            style: "width: 14px; height: 14px; min-width: 14px; min-height: 14px;",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            circle { cx: "11", cy: "11", r: "8" }
                            line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                        }
                    }
                    button {
                        class: "w-8 h-8 rounded-full flex items-center justify-center apple-tap opacity-70 hover:opacity-100",
                        style: "width: 32px; height: 32px; min-width: 32px; min-height: 32px;",
                        title: "Tanya AI",
                        onclick: move |_| on_toggle_ai.call(()),
                        svg {
                            class: "w-3.5 h-3.5",
                            width: "14",
                            height: "14",
                            style: "width: 14px; height: 14px; min-width: 14px; min-height: 14px;",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z" }
                        }
                    }
                    button {
                        class: "w-8 h-8 rounded-full flex items-center justify-center apple-tap font-serif text-xs font-semibold opacity-70 hover:opacity-100",
                        style: "width: 32px; height: 32px; min-width: 32px; min-height: 32px;",
                        title: "Tipografi",
                        onclick: move |_| on_toggle_typography.call(()),
                        "Aa"
                    }
                    button {
                        class: "w-8 h-8 rounded-full flex items-center justify-center apple-tap opacity-70 hover:opacity-100",
                        style: "width: 32px; height: 32px; min-width: 32px; min-height: 32px;",
                        title: "Daftar Bab",
                        onclick: move |_| on_toggle_toc.call(()),
                        svg {
                            class: "w-3.5 h-3.5",
                            width: "14",
                            height: "14",
                            style: "width: 14px; height: 14px; min-width: 14px; min-height: 14px;",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            line { x1: "8", y1: "6", x2: "21", y2: "6" }
                            line { x1: "8", y1: "12", x2: "21", y2: "12" }
                            line { x1: "8", y1: "18", x2: "21", y2: "18" }
                            line { x1: "3", y1: "6", x2: "3.01", y2: "6" }
                            line { x1: "3", y1: "12", x2: "3.01", y2: "12" }
                            line { x1: "3", y1: "18", x2: "3.01", y2: "18" }
                        }
                    }
                }
            }
        }
    }
}
