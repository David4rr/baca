use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn Navbar(
    title: String,
    theme: ThemeMode,
    is_reader: bool,
    on_back: EventHandler<()>,
    on_import: EventHandler<()>,
    on_toggle_toc: EventHandler<()>,
    on_toggle_typography: EventHandler<()>,
) -> Element {
    rsx! {
        header {
            class: "w-full h-16 border-b flex items-center justify-between px-6 select-none sticky top-0 z-30 backdrop-blur-md",
            style: "background-color: {theme.surface_color()}E6; border-color: {theme.border_color()}; color: {theme.text_primary()};",

            // Left Section
            div {
                class: "flex items-center gap-3",
                if is_reader {
                    button {
                        class: "px-3 py-1.5 rounded-xl border flex items-center gap-2 text-sm font-medium hover:bg-white/10 transition-all",
                        style: "border-color: {theme.border_color()};",
                        onclick: move |_| on_back.call(()),
                        "← Library"
                    }
                } else {
                    span {
                        class: "font-serif text-2xl font-bold tracking-tight",
                        style: "color: {theme.accent_color()};",
                        "Baca."
                    }
                }
            }

            // Middle Section
            div {
                class: "flex-1 text-center px-4 max-w-lg truncate",
                if is_reader {
                    span {
                        class: "text-sm font-semibold truncate",
                        title: "{title}",
                        "{title}"
                    }
                }
            }

            // Right Section Actions
            div {
                class: "flex items-center gap-2",
                if is_reader {
                    button {
                        class: "p-2 rounded-xl border hover:bg-white/10 transition-all text-sm font-medium",
                        style: "border-color: {theme.border_color()};",
                        title: "Table of Contents",
                        onclick: move |_| on_toggle_toc.call(()),
                        "📑 Chapters"
                    }
                    button {
                        class: "p-2 rounded-xl border hover:bg-white/10 transition-all text-sm font-medium",
                        style: "border-color: {theme.border_color()};",
                        title: "Appearance & Typography",
                        onclick: move |_| on_toggle_typography.call(()),
                        "Aa"
                    }
                } else {
                    button {
                        class: "px-4 py-2 rounded-xl text-sm font-semibold shadow-md flex items-center gap-2 transition-transform hover:scale-105 active:scale-95",
                        style: "background-color: {theme.accent_color()}; color: #FFF;",
                        onclick: move |_| on_import.call(()),
                        "+ Add Book"
                    }
                }
            }
        }
    }
}
