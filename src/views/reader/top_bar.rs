use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReadingMode {
    Scroll,
    Paged,
}

#[component]
pub fn ReaderTopBar(
    theme: ThemeMode,
    reading_mode: ReadingMode,
    is_bookmarked: bool,
    on_toggle_mode: EventHandler<()>,
    on_toggle_bookmark: EventHandler<()>,
    on_back: EventHandler<()>,
    on_toggle_search: EventHandler<()>,
    on_toggle_ai: EventHandler<()>,
    on_toggle_typography: EventHandler<()>,
    on_toggle_toc: EventHandler<()>,
) -> Element {
    let is_paged = reading_mode == ReadingMode::Paged;

    rsx! {
        div {
            class: "sticky top-0 z-30 w-full max-w-3xl mx-auto px-4 sm:px-6 pt-[max(env(safe-area-inset-top,0px),0.85rem)] pb-3 flex items-center justify-between pointer-events-auto apple-nav-glass transition-all duration-300 select-none",
            style: "background-color: {theme.bg_color()}DD;",

            // Circular Back Button (<)
            button {
                class: "w-10 h-10 rounded-full flex items-center justify-center shadow-sm border apple-tap transition-all",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                title: "Kembali ke Rak Buku",
                onclick: move |_| on_back.call(()),
                svg { class: "w-4 h-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round",
                    polyline { points: "15 18 9 12 15 6" }
                }
            }

            // Top Right Action Cluster: Mode Toggle, Bookmark, Search, AI, Aa, TOC
            div { class: "flex items-center gap-1.5 sm:gap-2",

                // Reading Mode Toggle Button (Scroll vs Paged / Swipe)
                button {
                    class: "h-9 px-2.5 sm:px-3 rounded-full flex items-center gap-1.5 shadow-sm border apple-tap transition-all",
                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    title: if is_paged { "Mode Halaman Aktif (Ketuk untuk Mode Gulir)" } else { "Mode Gulir Aktif (Ketuk untuk Mode Halaman)" },
                    onclick: move |_| on_toggle_mode.call(()),
                    if is_paged {
                        svg { class: "w-4 h-4 text-emerald-500", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                            path { d: "M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" }
                            path { d: "M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z" }
                        }
                    } else {
                        svg { class: "w-4 h-4 opacity-75", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                            line { x1: "12", y1: "5", x2: "12", y2: "19" }
                            polyline { points: "19 12 12 19 5 12" }
                        }
                    }
                    span { class: "text-xs font-sans font-medium hidden sm:inline",
                        if is_paged { "Halaman" } else { "Gulir" }
                    }
                }

                // Bookmark Toggle Button (🔖)
                button {
                    class: "w-9 h-9 rounded-full flex items-center justify-center apple-tap transition-all",
                    style: if is_bookmarked {
                        format!("color: {}; background-color: {}20;", theme.accent_color(), theme.accent_color())
                    } else {
                        format!("color: {}; opacity: 0.7;", theme.text_primary())
                    },
                    title: if is_bookmarked { "Hapus Markah Bab Ini" } else { "Simpan Markah Bab Ini" },
                    onclick: move |_| on_toggle_bookmark.call(()),
                    svg { class: "w-4 h-4", view_box: "0 0 24 24",
                        fill: if is_bookmarked { "currentColor" } else { "none" },
                        stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        path { d: "M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" }
                    }
                }

                // In-Book Search
                button {
                    class: "w-9 h-9 rounded-full flex items-center justify-center apple-tap opacity-70 hover:opacity-100 transition-all",
                    style: "color: {theme.text_primary()};",
                    title: "Cari Teks dalam Bab",
                    onclick: move |_| on_toggle_search.call(()),
                    svg { class: "w-4 h-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                        circle { cx: "11", cy: "11", r: "8" }
                        line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                    }
                }

                // Gemini AI
                button {
                    class: "w-9 h-9 rounded-full flex items-center justify-center apple-tap opacity-70 hover:opacity-100 transition-all",
                    style: "color: {theme.text_primary()};",
                    title: "Tanya AI (Gemini)",
                    onclick: move |_| on_toggle_ai.call(()),
                    svg { class: "w-4 h-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                        path { d: "m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z" }
                    }
                }

                // Typography Controls
                button {
                    class: "w-9 h-9 rounded-full flex items-center justify-center apple-tap font-serif text-[15px] font-medium opacity-80 hover:opacity-100 transition-all",
                    style: "color: {theme.text_primary()};",
                    title: "Pengaturan Tipografi",
                    onclick: move |_| on_toggle_typography.call(()),
                    "Aa"
                }

                // TOC Drawer
                button {
                    class: "w-9 h-9 rounded-full flex items-center justify-center apple-tap opacity-70 hover:opacity-100 transition-all",
                    style: "color: {theme.text_primary()};",
                    title: "Daftar Isi & Markah",
                    onclick: move |_| on_toggle_toc.call(()),
                    svg { class: "w-4 h-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
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
