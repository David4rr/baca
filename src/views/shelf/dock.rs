use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn FloatingDock(
    theme: ThemeMode,
    #[props(default = false)]
    has_update: bool,
    on_explore: EventHandler<()>,
    on_stats: EventHandler<()>,
    on_settings: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "fixed bottom-6 left-1/2 -translate-x-1/2 z-40 rounded-full px-6 py-3 flex items-center gap-7 sm:gap-9 shadow-lg border backdrop-blur-2xl transition-all duration-300",
            style: "position: fixed; bottom: 24px; left: 50%; transform: translateX(-50%); z-index: 40; border-radius: 9999px; padding: 10px 24px; display: flex; align-items: center; gap: 28px; background-color: {theme.surface_color()}EE; border: 1px solid {theme.border_color()}; box-shadow: 0 16px 40px rgba(0,0,0,0.12); -webkit-backdrop-filter: blur(24px); backdrop-filter: blur(24px);",

            // 1. Home / Shelf (Active solid fill)
            button {
                class: "apple-tap p-2.5 rounded-full transition-transform",
                style: "color: {theme.text_primary()};",
                title: "Rak Buku",
                svg {
                    class: "w-5 h-5",
                    width: "20",
                    height: "20",
                    style: "width: 20px; height: 20px; min-width: 20px; min-height: 20px;",
                    view_box: "0 0 24 24",
                    fill: "currentColor",
                    path { d: "M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z" }
                }
            }

            // 2. Explore / Internet Archive
            button {
                class: "apple-tap p-2.5 rounded-full transition-transform opacity-60 hover:opacity-100",
                style: "color: {theme.text_secondary()};",
                title: "Jelajahi & Unduh Online",
                onclick: move |_| on_explore.call(()),
                svg {
                    class: "w-5 h-5",
                    width: "20",
                    height: "20",
                    style: "width: 20px; height: 20px; min-width: 20px; min-height: 20px;",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    circle { cx: "12", cy: "12", r: "10" }
                    line { x1: "2", y1: "12", x2: "22", y2: "12" }
                    path { d: "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" }
                }
            }

            // 3. Reading Statistics
            button {
                class: "apple-tap p-2.5 rounded-full transition-transform opacity-60 hover:opacity-100",
                style: "color: {theme.text_secondary()};",
                title: "Statistik Membaca",
                onclick: move |_| on_stats.call(()),
                svg {
                    class: "w-5 h-5",
                    width: "20",
                    height: "20",
                    style: "width: 20px; height: 20px; min-width: 20px; min-height: 20px;",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    line { x1: "18", y1: "20", x2: "18", y2: "10" }
                    line { x1: "12", y1: "20", x2: "12", y2: "4" }
                    line { x1: "6", y1: "20", x2: "6", y2: "14" }
                }
            }

            // 4. Settings & Backup
            button {
                class: "apple-tap p-2.5 rounded-full transition-transform opacity-60 hover:opacity-100 relative",
                style: "color: {theme.text_secondary()};",
                title: "Cadangkan & Pengaturan",
                onclick: move |_| on_settings.call(()),
                if has_update {
                    span {
                        class: "w-2 h-2 rounded-full absolute top-0.5 right-0.5 animate-pulse",
                        style: "background-color: {theme.accent_color()};",
                    }
                }
                svg {
                    class: "w-5 h-5",
                    width: "20",
                    height: "20",
                    style: "width: 20px; height: 20px; min-width: 20px; min-height: 20px;",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    line { x1: "4", y1: "21", x2: "4", y2: "14" }
                    line { x1: "4", y1: "10", x2: "4", y2: "3" }
                    line { x1: "12", y1: "21", x2: "12", y2: "12" }
                    line { x1: "12", y1: "8", x2: "12", y2: "3" }
                    line { x1: "20", y1: "21", x2: "20", y2: "16" }
                    line { x1: "20", y1: "12", x2: "20", y2: "3" }
                    line { x1: "1", y1: "14", x2: "7", y2: "14" }
                    line { x1: "9", y1: "8", x2: "15", y2: "8" }
                    line { x1: "17", y1: "16", x2: "23", y2: "16" }
                }
            }
        }
    }
}
