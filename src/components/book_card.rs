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

    let is_midnight = book.title.to_lowercase().contains("midnight");
    let is_psychology = book.title.to_lowercase().contains("psychology");
    let is_atomic = book.title.to_lowercase().contains("atomic");

    let cover_bg = if is_midnight {
        "background-color: #172033; color: #F8FAFC;"
    } else if is_psychology {
        "background-color: #F5EFE6; color: #1C1917;"
    } else if is_atomic {
        "background-color: #ECE5D8; color: #1C1917;"
    } else {
        "background-color: #F0ECE4; color: #1C1917;"
    };

    rsx! {
        div {
            class: "book-card group relative flex flex-col cursor-pointer apple-tap",
            onclick: move |_| on_select.call(book_clone.clone()),

            // Cover Container with Apple-standard realistic depth
            div {
                class: "book-cover-realistic relative w-full aspect-[2/3] rounded-2xl overflow-hidden shadow-md",
                style: "{cover_bg} border: 1px solid rgba(0,0,0,0.06);",

                if let Some(cover) = &book.cover_image {
                    img {
                        class: "w-full h-full object-cover select-none pointer-events-none",
                        src: "{cover}",
                        alt: "{book.title}",
                    }
                } else {
                    // Refined Editorial Typeface Cover (matching reference Book reader.jpg)
                    div {
                        class: "w-full h-full flex flex-col justify-between p-3.5 select-none",
                        style: "border-left: 3px solid rgba(0,0,0,0.12);",

                        div {
                            class: "font-serif text-[13px] font-bold leading-tight line-clamp-3",
                            "{book.title}"
                        }

                        div {
                            class: "w-6 h-0.5 rounded-full my-auto opacity-30",
                            style: if is_midnight { "background-color: #FFF;" } else { "background-color: #000;" },
                        }

                        div {
                            class: "font-sans text-[10px] uppercase tracking-wider font-semibold opacity-70 truncate",
                            "{book.author}"
                        }
                    }
                }

                // Delete Button on Hover
                button {
                    class: "absolute top-2 right-2 rounded-full bg-black/60 text-white flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all duration-200 apple-tap shadow-sm",
                    style: "width: 24px; height: 24px;",
                    title: "Hapus Buku",
                    onclick: move |e| {
                        e.stop_propagation();
                        on_delete.call(book_id.clone());
                    },
                    svg {
                        width: "12",
                        height: "12",
                        style: "width: 12px; height: 12px;",
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

            // Book Details below cover (Editorial Style matching reference)
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
                        span { class: "font-semibold text-emerald-600 dark:text-emerald-400", "Finished" }
                    } else if progress > 0 {
                        span { "{progress}%" }
                    } else {
                        span { class: "opacity-60", "New" }
                    }
                }
            }
        }
    }
}
