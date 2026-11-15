use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn ExploreHeader(
    search_query: String,
    theme: ThemeMode,
    on_back: EventHandler<()>,
    on_search: EventHandler<String>,
) -> Element {
    let search_handler = on_search.clone();

    rsx! {
        div {
            class: "explore-header flex flex-col mb-6 select-none",

            // 1. Back button & Title
            div {
                class: "flex items-start justify-between gap-4 mb-6",
                div {
                    class: "flex items-center gap-3.5",
                    button {
                        class: "w-10 h-10 rounded-full flex items-center justify-center shadow-sm border apple-tap transition-all flex-shrink-0",
                        style: "width: 40px; height: 40px; min-width: 40px; min-height: 40px; background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        title: "Kembali ke Rak Buku",
                        onclick: move |_| on_back.call(()),
                        svg {
                            class: "w-4 h-4",
                            width: "16",
                            height: "16",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "15 18 9 12 15 6" }
                        }
                    }
                    div {
                        h1 {
                            class: "font-serif text-[28px] sm:text-[34px] font-medium tracking-tight leading-none",
                            style: "color: {theme.text_primary()};",
                            "Jelajahi Arsip Buku"
                        }
                        p {
                            class: "font-sans text-[13px] mt-1.5 opacity-70",
                            style: "color: {theme.text_secondary()};",
                            "Ribuan karya klasik dunia domain publik (Gutenberg & Internet Archive)"
                        }
                    }
                }
            }

            // 2. Search Input
            div {
                class: "relative w-full",
                input {
                    class: "w-full pl-11 pr-4 py-3 rounded-2xl border text-sm outline-none transition-all duration-200 focus:ring-2",
                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    placeholder: "Ketik judul buku, pengarang (misal: Sherlock Holmes, Jane Austen)...",
                    value: "{search_query}",
                    oninput: move |e| search_handler(e.value()),
                }
                div {
                    class: "absolute left-4 top-1/2 -translate-y-1/2 opacity-40 pointer-events-none",
                    svg {
                        class: "w-4 h-4",
                        width: "16",
                        height: "16",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        circle { cx: "11", cy: "11", r: "8" }
                        line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                    }
                }
            }
        }
    }
}
