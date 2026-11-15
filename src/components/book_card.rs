use dioxus::prelude::*;
use crate::db::models::Book;
use crate::theme::ThemeMode;

#[component]
pub fn BookCard(
    book: Book,
    theme: ThemeMode,
    on_select: EventHandler<Book>,
    on_delete: EventHandler<String>,
) -> Element {
    let book_clone = book.clone();
    let book_id = book.id.clone();
    let progress = (book.progress_percent * 100.0).round() as u32;

    rsx! {
        div {
            class: "book-card group relative flex flex-col cursor-pointer transition-all duration-300 hover:-translate-y-1",
            onclick: move |_| on_select.call(book_clone.clone()),

            // Cover Container
            div {
                class: "cover-container relative w-full aspect-[2/3] rounded-2xl overflow-hidden shadow-lg border transition-shadow duration-300 group-hover:shadow-2xl",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",

                if let Some(cover) = &book.cover_image {
                    img {
                        class: "w-full h-full object-cover",
                        src: "{cover}",
                        alt: "{book.title}",
                    }
                } else {
                    div {
                        class: "w-full h-full flex flex-col items-center justify-center p-4 text-center select-none",
                        style: "background: linear-gradient(135deg, {theme.surface_color()}, {theme.bg_color()});",
                        span {
                            class: "text-4xl mb-2 opacity-50",
                            "📖"
                        }
                        span {
                            class: "font-serif text-sm font-semibold line-clamp-3",
                            style: "color: {theme.text_primary()};",
                            "{book.title}"
                        }
                    }
                }

                // Progress Badge
                if progress > 0 {
                    div {
                        class: "absolute bottom-2 right-2 px-2 py-0.5 rounded-full text-xs font-medium backdrop-blur-md shadow",
                        style: "background-color: rgba(0,0,0,0.65); color: #FFF;",
                        "{progress}%"
                    }
                }

                // Delete Button on Hover
                button {
                    class: "absolute top-2 right-2 w-7 h-7 rounded-full bg-black/60 text-white/80 hover:text-white hover:bg-red-600 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity",
                    title: "Delete Book",
                    onclick: move |e| {
                        e.stop_propagation();
                        on_delete.call(book_id.clone());
                    },
                    "✕"
                }
            }

            // Book Details
            div {
                class: "mt-3 flex flex-col",
                h3 {
                    class: "font-semibold text-sm truncate",
                    style: "color: {theme.text_primary()};",
                    title: "{book.title}",
                    "{book.title}"
                }
                p {
                    class: "text-xs truncate opacity-75 mt-0.5",
                    style: "color: {theme.text_secondary()};",
                    "{book.author}"
                }
            }
        }
    }
}
