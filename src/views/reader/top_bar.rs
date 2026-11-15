use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn ReaderTopBar(
    theme: ThemeMode,
    on_back: EventHandler<()>,
    on_toggle_search: EventHandler<()>,
    on_toggle_ai: EventHandler<()>,
    on_toggle_typography: EventHandler<()>,
    on_toggle_toc: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "sticky top-0 z-30 w-full max-w-3xl mx-auto px-6 py-4 flex items-center justify-between pointer-events-auto backdrop-blur-md transition-all duration-300 select-none",
            style: "background-color: {theme.bg_color()}CC;",

            // Circular Back Button (<)
            button {
                class: "w-10 h-10 rounded-full flex items-center justify-center shadow-sm border apple-tap transition-all",
                style: "width: 40px; height: 40px; min-width: 40px; min-height: 40px; background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                title: "Kembali ke Rak Buku",
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

            // Top Right Action Cluster: Aa, TOC, Search, AI (matching reference)
            div {
                class: "flex items-center gap-2",

                button {
                    class: "w-9 h-9 rounded-full flex items-center justify-center apple-tap opacity-70 hover:opacity-100 transition-all",
                    style: "width: 36px; height: 36px; min-width: 36px; min-height: 36px; color: {theme.text_primary()};",
                    title: "Cari Teks dalam Bab",
                    onclick: move |_| on_toggle_search.call(()),
                    svg {
                        class: "w-4 h-4",
                        width: "16",
                        height: "16",
                        style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        circle { cx: "11", cy: "11", r: "8" }
                        line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                    }
                }

                button {
                    class: "w-9 h-9 rounded-full flex items-center justify-center apple-tap opacity-70 hover:opacity-100 transition-all",
                    style: "width: 36px; height: 36px; min-width: 36px; min-height: 36px; color: {theme.text_primary()};",
                    title: "Tanya AI (Gemini)",
                    onclick: move |_| on_toggle_ai.call(()),
                    svg {
                        class: "w-4 h-4",
                        width: "16",
                        height: "16",
                        style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path { d: "m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z" }
                    }
                }

                button {
                    class: "w-9 h-9 rounded-full flex items-center justify-center apple-tap font-serif text-[15px] font-medium opacity-80 hover:opacity-100 transition-all",
                    style: "width: 36px; height: 36px; min-width: 36px; min-height: 36px; color: {theme.text_primary()};",
                    title: "Pengaturan Tipografi",
                    onclick: move |_| on_toggle_typography.call(()),
                    "Aa"
                }

                button {
                    class: "w-9 h-9 rounded-full flex items-center justify-center apple-tap opacity-70 hover:opacity-100 transition-all",
                    style: "width: 36px; height: 36px; min-width: 36px; min-height: 36px; color: {theme.text_primary()};",
                    title: "Daftar Isi",
                    onclick: move |_| on_toggle_toc.call(()),
                    svg {
                        class: "w-4 h-4",
                        width: "16",
                        height: "16",
                        style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
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
