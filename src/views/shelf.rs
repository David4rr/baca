use dioxus::prelude::*;
use crate::components::BookCard;
use crate::db::models::Book;
use crate::theme::ThemeMode;

#[component]
pub fn ShelfView(
    books: Vec<Book>,
    theme: ThemeMode,
    search_query: String,
    on_search: EventHandler<String>,
    on_select_book: EventHandler<Book>,
    on_delete_book: EventHandler<String>,
    on_import: EventHandler<()>,
) -> Element {
    let query = search_query.to_lowercase();
    let filtered_books: Vec<Book> = books
        .into_iter()
        .filter(|b| {
            if query.is_empty() {
                true
            } else {
                b.title.to_lowercase().contains(&query) || b.author.to_lowercase().contains(&query)
            }
        })
        .collect();

    rsx! {
        div {
            class: "shelf-container flex-1 flex flex-col p-6 max-w-7xl mx-auto w-full",

            // Hero & Search Header
            div {
                class: "flex flex-col md:flex-row items-start md:items-center justify-between gap-4 mb-8",
                div {
                    class: "flex flex-col",
                    h1 {
                        class: "text-3xl font-bold tracking-tight font-serif",
                        style: "color: {theme.text_primary()};",
                        "Your Library"
                    }
                    p {
                        class: "text-sm mt-1 opacity-70",
                        style: "color: {theme.text_secondary()};",
                        "Offline, zero tracking, pure reading experience."
                    }
                }

                // Instant Search Bar
                div {
                    class: "relative w-full md:w-80",
                    input {
                        class: "w-full px-4 py-2.5 rounded-xl border text-sm outline-none transition-all focus:ring-2",
                        style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        placeholder: "Search title or author...",
                        value: "{search_query}",
                        oninput: move |e| on_search.call(e.value()),
                    }
                }
            }

            // Grid or Empty State
            if filtered_books.is_empty() {
                div {
                    class: "flex-1 flex flex-col items-center justify-center p-12 text-center rounded-3xl border border-dashed my-8",
                    style: "border-color: {theme.border_color()}; background-color: {theme.surface_color()}40;",

                    span { class: "text-6xl mb-4", "📚" }
                    h2 {
                        class: "text-xl font-semibold mb-2",
                        style: "color: {theme.text_primary()};",
                        if search_query.is_empty() { "Your shelf is empty" } else { "No matching books found" }
                    }
                    p {
                        class: "text-sm max-w-sm mb-6 opacity-70",
                        style: "color: {theme.text_secondary()};",
                        if search_query.is_empty() {
                            "Import your favorite EPUB or PDF books to read them offline with complete privacy."
                        } else {
                            "Try searching with a different keyword or title."
                        }
                    }

                    if search_query.is_empty() {
                        button {
                            class: "px-6 py-3 rounded-2xl text-sm font-semibold shadow-lg transition-transform hover:scale-105 active:scale-95",
                            style: "background-color: {theme.accent_color()}; color: #FFF;",
                            onclick: move |_| on_import.call(()),
                            "Import an EPUB or PDF"
                        }
                    }
                }
            } else {
                div {
                    class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-6",
                    for b in filtered_books {
                        BookCard {
                            key: "{b.id}",
                            book: b,
                            theme: theme,
                            on_select: move |selected| on_select_book.call(selected),
                            on_delete: move |id| on_delete_book.call(id),
                        }
                    }
                }
            }
        }
    }
}
