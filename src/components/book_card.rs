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
            class: "book-card group relative flex flex-col cursor-pointer apple-tap select-none",
            onclick: move |_| on_select.call(book_clone.clone()),

            // Cover Container with Apple-standard realistic 3D depth
            div {
                class: "book-cover-realistic relative w-full aspect-[2/3] rounded-2xl overflow-hidden shadow-md border",
                style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",

                if let Some(cover) = &book.cover_image {
                    img {
                        class: "w-full h-full object-cover select-none pointer-events-none transition-transform duration-500 group-hover:scale-105",
                        src: "{cover}",
                        alt: "{book.title}",
                        loading: "lazy",
                    }
                } else {
                    div {
                        class: "w-full h-full flex flex-col justify-between p-3.5 select-none",
                        style: "border-left: 4px solid {theme.accent_color()}; background-color: {theme.surface_color()};",

                        div {
                            class: "font-serif text-[13px] font-semibold leading-tight line-clamp-3",
                            style: "color: {theme.text_primary()};",
                            "{book.title}"
                        }

                        div {
                            class: "w-6 h-0.5 rounded-full my-auto opacity-30",
                            style: "background-color: {theme.text_muted()};",
                        }

                        div {
                            class: "font-sans text-[10px] uppercase tracking-wider font-semibold opacity-70 truncate",
                            style: "color: {theme.text_secondary()};",
                            "{book.author}"
                        }
                    }
                }

                // Delete Button on Hover
                button {
                    class: "absolute top-2 right-2 rounded-full bg-black/60 hover:bg-black/80 text-white flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all duration-200 apple-tap shadow-sm",
                    style: "width: 26px; height: 26px; min-width: 26px; min-height: 26px;",
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

            // Book Details below cover (Refined Editorial Typography)
            div {
                class: "mt-2.5 flex flex-col",
                h3 {
                    class: "font-serif text-[14px] font-medium truncate leading-tight transition-colors group-hover:opacity-80",
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
                        span { class: "font-semibold text-emerald-600 dark:text-emerald-400", "Selesai" }
                    } else if progress > 0 {
                        span { "{progress}%" }
                    } else {
                        span { class: "opacity-60", "Baru" }
                    }
                }
            }
        }
    }
}
