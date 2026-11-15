use dioxus::prelude::*;
use crate::components::MorphingDownloadButton;
use crate::db::models::Book;
use crate::engine::ArchiveBook;
use crate::theme::ThemeMode;

#[component]
pub fn ExploreBookItem(
    book: ArchiveBook,
    theme: ThemeMode,
    is_downloaded: bool,
    on_imported: EventHandler<Book>,
) -> Element {
    let thumb_url = book.thumbnail_url();
    let author_display = book.author_display();
    let downloads_count = book.downloads.unwrap_or(0);
    let title = book.title.clone();

    rsx! {
        div {
            class: "explore-book-item w-full rounded-3xl p-4 sm:p-5 border transition-all duration-300 flex flex-col justify-between select-none shadow-sm gap-4",
            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; box-shadow: {theme.card_shadow()};",

            // Top Row: Thumbnail + Info
            div {
                class: "flex items-start gap-4",

                // 1. Cover Thumbnail with 3D Depth
                div {
                    class: "book-cover-realistic w-16 sm:w-20 aspect-[2/3] rounded-xl flex-shrink-0 border overflow-hidden",
                    style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",
                    img {
                        class: "w-full h-full object-cover select-none pointer-events-none",
                        src: "{thumb_url}",
                        alt: "{title}",
                        loading: "lazy",
                    }
                }

                // 2. Info Stack
                div {
                    class: "flex-1 min-w-0 flex flex-col",
                    h3 {
                        class: "font-serif text-[16px] sm:text-[18px] font-semibold leading-tight line-clamp-2",
                        style: "color: {theme.text_primary()};",
                        title: "{title}",
                        "{title}"
                    }
                    p {
                        class: "font-sans text-xs opacity-70 mt-1 truncate",
                        style: "color: {theme.text_secondary()};",
                        "{author_display}"
                    }

                    if let Some(desc) = &book.description {
                        p {
                            class: "font-sans text-[11px] opacity-60 mt-2 line-clamp-2 leading-relaxed",
                            style: "color: {theme.text_secondary()};",
                            "{desc}"
                        }
                    }

                    // Format Badges
                    div {
                        class: "flex items-center gap-1.5 mt-2.5",
                        if book.has_epub {
                            span {
                                class: "px-2 py-0.5 rounded text-[10px] font-sans font-bold uppercase",
                                style: "background-color: {theme.accent_color()}18; color: {theme.accent_color()};",
                                "EPUB"
                            }
                        }
                        if book.has_pdf {
                            span {
                                class: "px-2 py-0.5 rounded text-[10px] font-sans font-bold uppercase",
                                style: "background-color: #ef444418; color: #ef4444;",
                                "PDF"
                            }
                        }
                    }
                }
            }

            // Bottom Action Row: Downloads count + Morphing Download Button
            div {
                class: "flex items-center justify-between pt-2 border-t mt-auto",
                style: "border-color: {theme.border_color()};",

                div {
                    class: "flex items-center gap-1.5 text-xs opacity-60 font-sans",
                    style: "color: {theme.text_muted()};",
                    svg {
                        class: "w-3.5 h-3.5",
                        width: "14",
                        height: "14",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                        polyline { points: "7 10 12 15 17 10" }
                        line { x1: "12", y1: "15", x2: "12", y2: "3" }
                    }
                    span { "{downloads_count} unduhan" }
                }

                // Apple Morphing Download Button
                MorphingDownloadButton {
                    book: book.clone(),
                    theme: theme,
                    is_already_downloaded: is_downloaded,
                    on_imported: move |b| on_imported.call(b),
                }
            }
        }
    }
}
