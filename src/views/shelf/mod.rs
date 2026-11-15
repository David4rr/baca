pub mod dock;
pub mod empty_state;
pub mod hero_card;
pub mod recent_grid;

use dioxus::prelude::*;
use crate::db::models::Book;
use crate::theme::ThemeMode;
pub use dock::FloatingDock;
pub use empty_state::EmptyShelfState;
pub use hero_card::HeroCard;
pub use recent_grid::RecentGrid;

#[component]
pub fn ShelfView(
    books: Vec<Book>,
    theme: ThemeMode,
    search_query: String,
    on_search: EventHandler<String>,
    on_select_book: EventHandler<Book>,
    on_delete_book: EventHandler<String>,
    on_import: EventHandler<()>,
    on_explore: EventHandler<()>,
    on_stats: EventHandler<()>,
    on_settings: EventHandler<()>,
) -> Element {
    let mut is_search_visible = use_signal(|| false);

    let query = search_query.to_lowercase();
    let filtered_books: Vec<Book> = books
        .iter()
        .filter(|b| {
            if query.is_empty() {
                true
            } else {
                b.title.to_lowercase().contains(&query) || b.author.to_lowercase().contains(&query)
            }
        })
        .cloned()
        .collect();

    let featured_book = filtered_books.first().cloned();
    let recent_books: Vec<Book> = if filtered_books.len() > 1 {
        filtered_books.iter().skip(1).take(6).cloned().collect()
    } else {
        vec![]
    };

    rsx! {
        div {
            class: "shelf-view flex-1 flex flex-col w-full max-w-4xl mx-auto px-5 sm:px-8 pt-8 pb-32 transition-colors duration-300 animate-fade-in",

            // 1. Apple-Standard Editorial Header
            div {
                class: "flex items-start justify-between mb-8 select-none",
                div {
                    class: "flex flex-col",
                    h1 {
                        class: "font-serif text-[34px] sm:text-[40px] font-medium tracking-tight leading-none",
                        style: "color: {theme.text_primary()};",
                        "My Library"
                    }
                    p {
                        class: "font-sans text-[13px] sm:text-[14px] mt-2 font-normal tracking-normal opacity-70",
                        style: "color: {theme.text_secondary()};",
                        "Pick up where you left off"
                    }
                }

                div {
                    class: "flex items-center gap-2.5",
                    button {
                        class: "w-10 h-10 rounded-full flex items-center justify-center shadow-sm border apple-tap transition-all",
                        style: "width: 40px; height: 40px; min-width: 40px; min-height: 40px; background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        title: "Import Buku (EPUB)",
                        onclick: move |_| on_import.call(()),
                        svg {
                            class: "w-4 h-4",
                            width: "16",
                            height: "16",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line { x1: "12", y1: "5", x2: "12", y2: "19" }
                            line { x1: "5", y1: "12", x2: "19", y2: "12" }
                        }
                    }
                    button {
                        class: "w-10 h-10 rounded-full flex items-center justify-center shadow-sm border apple-tap transition-all",
                        style: "width: 40px; height: 40px; min-width: 40px; min-height: 40px; background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        title: "Cari Buku",
                        onclick: move |_| {
                            let cur = *is_search_visible.read();
                            is_search_visible.set(!cur);
                        },
                        svg {
                            class: "w-4 h-4",
                            width: "16",
                            height: "16",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "11", cy: "11", r: "8" }
                            line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                        }
                    }
                }
            }

            // Expandable Search Bar
            if *is_search_visible.read() || !search_query.is_empty() {
                div {
                    class: "mb-6 animate-slide-up",
                    div {
                        class: "relative w-full",
                        input {
                            class: "w-full pl-11 pr-4 py-3 rounded-2xl border text-sm outline-none transition-all duration-200 focus:ring-2",
                            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                            placeholder: "Cari judul buku atau pengarang...",
                            value: "{search_query}",
                            autofocus: true,
                            oninput: move |e| on_search.call(e.value()),
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

            // 2. Empty State or Book Collections
            if filtered_books.is_empty() {
                EmptyShelfState {
                    search_query: search_query,
                    theme: theme,
                    on_import: move |_| on_import.call(()),
                    on_explore: move |_| on_explore.call(()),
                }
            } else {
                if let Some(book) = featured_book {
                    HeroCard {
                        book: book.clone(),
                        theme: theme,
                        on_select_book: move |b| on_select_book.call(b),
                        on_delete_book: move |id| on_delete_book.call(id),
                    }
                }

                RecentGrid {
                    books: filtered_books,
                    recent_books: recent_books,
                    search_query: search_query,
                    theme: theme,
                    on_select_book: move |b| on_select_book.call(b),
                    on_delete_book: move |id| on_delete_book.call(id),
                }
            }

            // 3. Floating Apple Island Dock
            FloatingDock {
                theme: theme,
                on_explore: move |_| on_explore.call(()),
                on_stats: move |_| on_stats.call(()),
                on_settings: move |_| on_settings.call(()),
            }
        }
    }
}
