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
    let mut expanded = use_signal(|| false);
    let is_searching = !search_query.is_empty();
    let is_expanded = *expanded.read() || is_searching;
    let has_recent = !recent_books.is_empty();

    let visible: Vec<Book> = if is_expanded {
        books.clone()
    } else {
        recent_books.iter().take(6).cloned().collect()
    };
    let can_expand = !is_expanded && (books.len() > 6 || recent_books.len() > 6);

    rsx! {
        div {
            class: "shelf-books-section flex flex-col",

            if has_recent || is_expanded {
                div {
                    class: "mb-10 animate-fade-in",
                    div {
                        class: "flex items-center justify-between mb-4",
                        h2 {
                            class: "font-serif text-[20px] font-semibold tracking-tight",
                            style: "color: {theme.text_primary()};",
                            if is_expanded { "Semua Koleksi ({books.len()})" } else { "Recent" }
                        }
                        if can_expand {
                            button {
                                class: "font-sans text-[12px] font-medium opacity-60 hover:opacity-100 transition-opacity apple-tap",
                                style: "color: {theme.text_secondary()};",
                                onclick: move |_| expanded.set(true),
                                "Lihat Semua"
                            }
                        } else if is_expanded && !is_searching {
                            button {
                                class: "font-sans text-[12px] font-medium opacity-60 hover:opacity-100 transition-opacity apple-tap",
                                style: "color: {theme.text_secondary()};",
                                onclick: move |_| expanded.set(false),
                                "Ringkas"
                            }
                        }
                    }

                    div {
                        class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-3.5 sm:gap-4 md:gap-5",
                        for b in visible {
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
