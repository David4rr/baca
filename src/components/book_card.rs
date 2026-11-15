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
            class: "book-card group relative flex flex-col cursor-pointer apple-tap",
            onclick: move |_| on_select.call(book_clone.clone()),

            // Cover Container with Apple-standard realistic depth
            div {
                class: "book-cover-realistic relative w-full aspect-[2/3] rounded-2xl overflow-hidden shadow-md",
                style: "background-color: {theme.surface_subtle()};",

                if let Some(cover) = &book.cover_image {
                    img {
                        class: "w-full h-full object-cover select-none pointer-events-none",
                        src: "{cover}",
                        alt: "{book.title}",
                    }
                } else {
                    div {
                        class: "w-full h-full flex flex-col items-center justify-between p-4 text-center select-none",
                        style: "background: linear-gradient(145deg, {theme.surface_color()}, {theme.surface_subtle()});",
                        div { class: "w-full h-4" }
                        div {
                            class: "font-serif text-sm font-semibold px-2 line-clamp-4 leading-relaxed",
                            style: "color: {theme.text_primary()};",
                            "{book.title}"
                        }
                        div {
                            class: "font-sans text-[11px] font-medium opacity-60 truncate max-w-full",
                            style: "color: {theme.text_secondary()};",
                            "{book.author}"
                        }
                    }
                }

                // Delete Button on Hover (Clean Apple circular button)
                button {
                    class: "absolute top-2 right-2 w-7 h-7 rounded-full bg-black/60 backdrop-blur-md text-white/90 hover:bg-red-600 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all duration-200 apple-tap shadow-sm",
                    title: "Hapus Buku",
                    onclick: move |e| {
                        e.stop_propagation();
                        on_delete.call(book_id.clone());
                    },
                    svg {
                        class: "w-3.5 h-3.5",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line { x1: "18", y1: "6", x2: "6", y2: "18" }
                        line { x1: "6", y1: "6", x2: "18", y2: "18" }
                    }
                }
            }

            // Book Details (Editorial Style matching reference)
            div {
                class: "mt-2.5 flex flex-col",
                h3 {
                    class: "font-serif text-[14px] font-semibold truncate leading-tight",
                    style: "color: {theme.text_primary()};",
                    title: "{book.title}",
                    "{book.title}"
                }
                p {
                    class: "font-sans text-[12px] truncate opacity-70 mt-0.5",
                    style: "color: {theme.text_secondary()};",
                    "{book.author}"
                }
                div {
                    class: "font-sans text-[11px] font-medium mt-1",
                    style: "color: {theme.text_secondary()};",
                    if progress == 100 {
                        span { class: "text-emerald-600 dark:text-emerald-400 font-semibold", "Selesai" }
                    } else if progress > 0 {
                        span { "{progress}% dibaca" }
                    } else {
                        span { class: "opacity-60", "Baru" }
                    }
                }
            }
        }
    }
}
