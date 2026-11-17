use dioxus::prelude::*;
use crate::db::models::Book;
use crate::theme::ThemeMode;

#[component]
pub fn HeroCard(
    book: Book,
    theme: ThemeMode,
    on_select_book: EventHandler<Book>,
    on_delete_book: EventHandler<String>,
) -> Element {
    let book_click = book.clone();
    let book_delete = book.clone();
    let progress = (book.progress_percent * 100.0).round() as u32;
    let remaining_chapters = book.total_chapters.saturating_sub(book.current_chapter).max(1);
    let est_minutes_left = (remaining_chapters * 4).max(1);

    rsx! {
        div {
            class: "hero-bento-card relative rounded-[28px] p-5 sm:p-6 mb-10 border transition-all duration-300 animate-slide-up shadow-sm select-none",
            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; box-shadow: {theme.card_shadow()};",

            // Top Row: Book Cover & Details
            div {
                class: "flex items-start gap-5 sm:gap-6",

                // Book Cover with Realistic Depth
                div {
                    class: "book-cover-realistic w-20 sm:w-24 aspect-[2/3] rounded-xl flex-shrink-0 cursor-pointer apple-tap",
                    role: "button",
                    tabindex: "0",
                    aria_label: "Buka buku {book.title}",
                    onclick: {
                        let b = book_click.clone();
                        move |_| on_select_book.call(b.clone())
                    },
                    onkeydown: {
                        let b = book_click.clone();
                        move |e: KeyboardEvent| {
                            if e.key() == Key::Enter || e.key() == Key::Character(" ".to_string()) {
                                on_select_book.call(b.clone());
                            }
                        }
                    },
                    if let Some(cover) = &book.cover_image {
                        img {
                            class: "w-full h-full object-cover select-none pointer-events-none",
                            src: "{cover}",
                            alt: "{book.title}",
                        }
                    } else {
                        div {
                            class: "w-full h-full flex flex-col items-center justify-center p-2 text-center",
                            style: "background-color: {theme.surface_subtle()};",
                            span { class: "font-serif text-[11px] font-semibold line-clamp-3", "{book.title}" }
                        }
                    }
                }

                // Book Information Stack
                div {
                    class: "flex-1 min-w-0 flex flex-col justify-between self-stretch",

                    div {
                        div {
                            class: "flex items-start justify-between gap-2",
                            h2 {
                                class: "font-serif text-[18px] sm:text-[21px] font-semibold leading-tight line-clamp-2 cursor-pointer hover:opacity-80 transition-opacity",
                                style: "color: {theme.text_primary()};",
                                onclick: {
                                    let b = book_click.clone();
                                    move |_| on_select_book.call(b.clone())
                                },
                                "{book.title}"
                            }

                            // Direct Delete Button (eliminates redundant menu on last read)
                            button {
                                class: "w-10 h-10 rounded-full flex items-center justify-center apple-tap transition-all opacity-70 hover:opacity-100 hover:text-rose-500 hover:bg-rose-500/10 flex-shrink-0",
                                style: "width: 40px; height: 40px; min-width: 40px; min-height: 40px; background-color: {theme.surface_subtle()}; color: {theme.text_secondary()};",
                                title: "Hapus Buku dari Rak",
                                aria_label: "Hapus buku dari rak",
                                onclick: {
                                    let id = book_delete.id.clone();
                                    move |e: MouseEvent| {
                                        e.stop_propagation();
                                        on_delete_book.call(id.clone());
                                    }
                                },
                                svg {
                                    class: "w-4 h-4",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M3 6h18" }
                                    path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
                                    path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
                                }
                            }
                        }

                        p {
                            class: "font-sans text-[13px] opacity-70 mt-1 truncate",
                            style: "color: {theme.text_secondary()};",
                            "{book.author}"
                        }
                    }

                    // Progress Section
                    div {
                        class: "mt-4",
                        div {
                            class: "flex items-center justify-between text-[12px] font-sans font-medium mb-1.5",
                            span {
                                style: "color: {theme.text_muted()};",
                                if progress >= 100 { "Selesai dibaca" } else { "± {est_minutes_left} menit tersisa" }
                            }
                            span {
                                class: "font-semibold",
                                style: "color: {theme.text_primary()};",
                                "{progress}%"
                            }
                        }

                        // Thin Apple Progress Bar
                        div {
                            class: "w-full h-1.5 rounded-full overflow-hidden",
                            style: "background-color: {theme.surface_subtle()};",
                            div {
                                class: "h-full rounded-full transition-all duration-500",
                                style: "width: {progress}%; background-color: {theme.accent_color()};",
                            }
                        }
                    }
                }
            }

            // Bottom Row: Dark Tactile "Continue Reading" Button with Nested Trailing Chevron
            div {
                class: "mt-5 pt-1",
                button {
                    class: "w-full h-12 rounded-2xl px-5 flex items-center justify-between text-sm font-medium shadow-sm apple-tap transition-all group",
                    style: "background-color: {theme.accent_color()}; color: {theme.accent_contrast_text()};",
                    onclick: {
                        let b = book_click.clone();
                        move |_| on_select_book.call(b.clone())
                    },
                    span { "Continue Reading" }
                    div {
                        class: "w-7 h-7 rounded-full flex items-center justify-center transition-transform group-hover:translate-x-0.5",
                        style: "width: 28px; height: 28px; min-width: 28px; min-height: 28px; background-color: rgba(255,255,255,0.18);",
                        svg {
                            class: "w-3.5 h-3.5",
                            width: "14",
                            height: "14",
                            style: "width: 14px; height: 14px; min-width: 14px; min-height: 14px;",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "9 18 15 12 9 6" }
                        }
                    }
                }
            }
        }
    }
}
