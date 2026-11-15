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
    rsx! {
        header {
            class: "w-full h-16 border-b flex items-center justify-between px-4 sm:px-6 select-none sticky top-0 z-30 backdrop-blur-md",
            style: "background-color: {theme.surface_color()}E6; border-color: {theme.border_color()}; color: {theme.text_primary()};",

            // Left Section
            div {
                class: "flex items-center gap-2 sm:gap-3",
                if is_reader || is_explore {
                    button {
                        class: "px-3 py-1.5 rounded-xl border flex items-center gap-1.5 text-xs sm:text-sm font-semibold hover:bg-white/10 transition-all",
                        style: "border-color: {theme.border_color()};",
                        onclick: move |_| on_back.call(()),
                        "← Rak Buku"
                    }
                } else {
                    span {
                        class: "font-serif text-2xl font-bold tracking-tight cursor-pointer",
                        style: "color: {theme.accent_color()};",
                        onclick: move |_| on_back.call(()),
                        "Baca."
                    }
                }
            }

            // Middle Section
            div {
                class: "flex-1 text-center px-2 sm:px-4 max-w-lg truncate",
                if is_reader {
                    span {
                        class: "text-xs sm:text-sm font-semibold truncate block",
                        title: "{title}",
                        "{title}"
                    }
                }
            }

            // Right Section Actions
            div {
                class: "flex items-center gap-1.5 sm:gap-2",
                if is_reader {
                    button {
                        class: "p-2 rounded-xl border hover:bg-white/10 transition-all text-xs font-semibold flex items-center gap-1",
                        style: "border-color: {theme.border_color()};",
                        title: "Cari Teks dalam Buku",
                        onclick: move |_| on_toggle_search.call(()),
                        "🔍"
                    }
                    button {
                        class: "p-2 rounded-xl border hover:bg-white/10 transition-all text-xs font-semibold flex items-center gap-1",
                        style: "border-color: {theme.border_color()};",
                        title: "Tanya AI (Gemini)",
                        onclick: move |_| on_toggle_ai.call(()),
                        "✨ AI"
                    }
                    button {
                        class: "p-2 rounded-xl border hover:bg-white/10 transition-all text-xs font-semibold flex items-center gap-1",
                        style: "border-color: {theme.border_color()};",
                        title: "Daftar Bab",
                        onclick: move |_| on_toggle_toc.call(()),
                        "📑 Bab"
                    }
                    button {
                        class: "p-2 rounded-xl border hover:bg-white/10 transition-all text-xs font-semibold",
                        style: "border-color: {theme.border_color()};",
                        title: "Pengaturan Tipografi",
                        onclick: move |_| on_toggle_typography.call(()),
                        "Aa"
                    }
                } else if !is_explore {
                    button {
                        class: "px-3 py-1.5 rounded-xl border hover:bg-white/10 transition-all text-xs font-semibold flex items-center gap-1",
                        style: "border-color: {theme.border_color()};",
                        title: "Cari Buku di Internet Archive",
                        onclick: move |_| on_explore.call(()),
                        "🌐 Unduh Online"
                    }
                    button {
                        class: "p-2 rounded-xl border hover:bg-white/10 transition-all text-xs font-semibold",
                        style: "border-color: {theme.border_color()};",
                        title: "Statistik Membaca",
                        onclick: move |_| on_toggle_stats.call(()),
                        "📊"
                    }
                    button {
                        class: "p-2 rounded-xl border hover:bg-white/10 transition-all text-xs font-semibold",
                        style: "border-color: {theme.border_color()};",
                        title: "Pengaturan & Backup",
                        onclick: move |_| on_toggle_settings.call(()),
                        "⚙️"
                    }
                    button {
                        class: "px-3.5 py-1.5 rounded-xl text-xs sm:text-sm font-bold shadow-md flex items-center gap-1 transition-transform hover:scale-105 active:scale-95",
                        style: format!("background-color: {}; color: #000;", theme.accent_color()),
                        onclick: move |_| on_import.call(()),
                        "+ Impor"
                    }
                }
            }
        }
    }
}
