use dioxus::prelude::*;
use crate::components::BookCard;
use crate::db::models::Book;
use crate::theme::ThemeMode;

#[component]
pub fn RecentGrid(
    books: Vec<Book>,
    recent_books: Vec<Book>,
    search_query: String,
    theme: ThemeMode,
    on_select_book: EventHandler<Book>,
    on_delete_book: EventHandler<String>,
) -> Element {
    let has_recent = !recent_books.is_empty();
    let show_all = books.len() > 7 || !search_query.is_empty();

    rsx! {
        div {
            class: "shelf-books-section flex flex-col",

            if has_recent {
                div {
                    class: "mb-10 animate-fade-in",
                    div {
                        class: "flex items-center justify-between mb-4",
                        h2 {
                            class: "font-serif text-[20px] font-semibold tracking-tight",
                            style: "color: {theme.text_primary()};",
                            "Recent"
                        }
                        span {
                            class: "font-sans text-[12px] font-medium opacity-60 hover:opacity-100 cursor-pointer transition-opacity",
                            style: "color: {theme.text_secondary()};",
                            "View all"
                        }
                    }

                    div {
                        class: "grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 gap-4 sm:gap-5",
                        for b in recent_books {
                            BookCard {
                                key: "{b.id}",
                                book: b.clone(),
                                theme: theme,
                                on_select: move |selected| on_select_book.call(selected),
                                on_delete: move |id| on_delete_book.call(id),
                            }
                        }
                    }
                }
            }

            if show_all {
                div {
                    class: "mb-8 animate-fade-in",
                    h2 {
                        class: "font-serif text-[20px] font-semibold tracking-tight mb-4",
                        style: "color: {theme.text_primary()};",
                        "Semua Koleksi ({books.len()})"
                    }
                    div {
                        class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-5",
                        for b in books {
                            BookCard {
                                key: "{b.id}",
                                book: b.clone(),
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
}
