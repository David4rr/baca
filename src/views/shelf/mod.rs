pub mod dock;
pub mod empty_state;
pub mod hero_card;
pub mod online_feed;
pub mod recent_grid;

use dioxus::prelude::*;
use crate::db::models::Book;
use crate::theme::ThemeMode;
pub use dock::FloatingDock;
pub use empty_state::EmptyShelfState;
pub use hero_card::HeroCard;
pub use online_feed::OnlineFeed;
pub use recent_grid::RecentGrid;

#[component]
pub fn ShelfView(
    books: Vec<Book>,
    theme: ThemeMode,
    search_query: String,
    downloaded_ids: Vec<String>,
    #[props(default = false)]
    has_update: bool,
    on_search: EventHandler<String>,
    on_select_book: EventHandler<Book>,
    on_delete_book: EventHandler<String>,
    on_import: EventHandler<()>,
    on_import_book: EventHandler<Book>,
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
            class: "shelf-view flex-1 flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 md:px-8 pt-[calc(env(safe-area-inset-top,0px)+3.25rem)] pb-32 transition-colors duration-300 animate-fade-in",

            // 1. Editorial Header
            div {
                class: "flex items-start justify-between gap-3 mb-8 select-none w-full min-w-0",
                div {
                    class: "flex flex-col min-w-0 flex-1 mr-2",
                    h1 {
                        class: "font-serif text-[28px] sm:text-[40px] font-medium tracking-tight leading-none truncate",
                        style: "color: {theme.text_primary()};",
                        "Perpustakaan Saya"
                    }
                    p {
                        class: "font-sans text-xs sm:text-[14px] mt-2 font-normal tracking-normal opacity-70 truncate",
                        style: "color: {theme.text_secondary()};",
                        "Koleksi buku offline dan temuan literatur baru"
                    }
                }

                div {
                    class: "flex items-center gap-2 flex-shrink-0",
                    button {
                        class: "w-10 h-10 rounded-full flex items-center justify-center shadow-sm border apple-tap transition-all",
                        style: "width: 40px; height: 40px; min-width: 40px; min-height: 40px; background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        title: "Tambah Buku Lokal (EPUB / PDF)",
                        onclick: move |_| on_import.call(()),
                        svg { class: "w-4 h-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.2", stroke_linecap: "round", stroke_linejoin: "round",
                            line { x1: "12", y1: "5", x2: "12", y2: "19" }
                            line { x1: "5", y1: "12", x2: "19", y2: "12" }
                        }
                    }
                    button {
                        class: "w-10 h-10 rounded-full flex items-center justify-center shadow-sm border apple-tap transition-all",
                        style: "width: 40px; height: 40px; min-width: 40px; min-height: 40px; background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        title: "Cari Koleksi",
                        onclick: move |_| {
                            let cur = *is_search_visible.read();
                            is_search_visible.set(!cur);
                        },
                        svg { class: "w-4 h-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.2", stroke_linecap: "round", stroke_linejoin: "round",
                            circle { cx: "11", cy: "11", r: "8" }
                            line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                        }
                    }
                }
            }

            // Search Bar with Symmetrical Spacing
            if *is_search_visible.read() || !search_query.is_empty() {
                div {
                    class: "mb-6 animate-slide-up w-full min-w-0",
                    div {
                        class: "relative w-full min-w-0",
                        input {
                            class: "w-full min-w-0 pl-11 pr-10 py-3 rounded-2xl border text-sm outline-none transition-all duration-200 focus:ring-2 box-border",
                            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                            placeholder: "Cari judul buku atau pengarang...",
                            value: "{search_query}",
                            autofocus: true,
                            oninput: move |e| on_search.call(e.value()),
                        }
                        div { class: "absolute left-4 top-1/2 -translate-y-1/2 opacity-40 pointer-events-none",
                            svg { class: "w-4 h-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                circle { cx: "11", cy: "11", r: "8" }
                                line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                            }
                        }
                        if !search_query.is_empty() {
                            button {
                                class: "absolute right-3.5 top-1/2 -translate-y-1/2 opacity-60 hover:opacity-100 p-1.5 text-xs transition-opacity flex items-center justify-center rounded-full",
                                style: "color: {theme.text_secondary()};",
                                onclick: {
                                    let on_search = on_search.clone();
                                    move |_| on_search.call(String::new())
                                },
                                "✕"
                            }
                        }
                    }
                }
            }

            // 2. Offline Books (Top of Home Screen)
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

            // 3. Online Books Discovery Stream (Bottom of Home Screen)
            OnlineFeed {
                theme: theme,
                downloaded_ids: downloaded_ids,
                on_imported_book: move |b| on_import_book.call(b),
            }

            // 4. Floating Apple Island Dock
            FloatingDock {
                theme: theme,
                has_update: has_update,
                on_explore: move |_| on_explore.call(()),
                on_stats: move |_| on_stats.call(()),
                on_settings: move |_| on_settings.call(()),
            }
        }
    }
}
