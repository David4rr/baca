use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn ExploreHeader(
    search_query: String,
    theme: ThemeMode,
    on_back: EventHandler<()>,
    on_search: EventHandler<String>,
) -> Element {
    let mut local_text = use_signal(|| search_query.clone());
    let mut debounce_id = use_signal(|| 0u64);

    use_effect(use_reactive(&search_query, move |q| {
        if *local_text.peek() != q {
            local_text.set(q);
        }
    }));

    let mut on_input_change = {
        let on_search = on_search;
        move |val: String| {
            local_text.set(val.clone());
            let next_id = *debounce_id.read() + 1;
            debounce_id.set(next_id);
            let search = on_search;

            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(380)).await;
                if *debounce_id.read() == next_id {
                    search.call(val.trim().to_string());
                }
            });
        }
    };

    rsx! {
        div {
            class: "explore-header flex flex-col mb-6 select-none w-full",

            // 1. Symmetrical Top Bar
            div {
                class: "flex items-center justify-between gap-3 mb-5 w-full min-w-0",
                div {
                    class: "flex items-center gap-3 min-w-0 flex-1",
                    button {
                        class: "w-10 h-10 rounded-full flex items-center justify-center shadow-sm border apple-tap transition-all flex-shrink-0",
                        style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
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
                        class: "flex flex-col min-w-0 flex-1",
                        h1 {
                            class: "font-serif text-[24px] sm:text-[32px] font-medium tracking-tight leading-tight truncate",
                            style: "color: {theme.text_primary()};",
                            "Cari Buku Online"
                        }
                        p {
                            class: "font-sans text-xs opacity-60 mt-0.5 truncate",
                            style: "color: {theme.text_secondary()};",
                            "Katalog publik Project Gutenberg & Internet Archive"
                        }
                    }
                }
            }

            // 2. Search Input with Real Space Support & Debounce
            div {
                class: "relative w-full min-w-0",
                input {
                    class: "w-full min-w-0 pl-11 pr-10 py-3 rounded-2xl border text-sm outline-none transition-all duration-200 focus:ring-2 box-border",
                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    placeholder: "Ketik judul buku atau nama pengarang...",
                    value: "{local_text}",
                    oninput: move |e| on_input_change(e.value()),
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
                if !local_text.read().is_empty() {
                    button {
                        class: "absolute right-3.5 top-1/2 -translate-y-1/2 opacity-60 hover:opacity-100 p-1.5 text-xs transition-opacity flex items-center justify-center rounded-full",
                        style: "color: {theme.text_secondary()};",
                        onclick: {
                            let on_search = on_search;
                            move |_| {
                                local_text.set(String::new());
                                on_search.call(String::new());
                            }
                        },
                        svg { class: "w-3.5 h-3.5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5",
                            line { x1: "18", y1: "6", x2: "6", y2: "18" }
                            line { x1: "6", y1: "6", x2: "18", y2: "18" }
                        }
                    }
                }
            }
        }
    }
}
