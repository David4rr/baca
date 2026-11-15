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
            class: "explore-book-item w-full rounded-3xl p-4 sm:p-5 border transition-all duration-300 flex flex-col justify-between select-none shadow-sm hover:shadow-md gap-4",
            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; box-shadow: {theme.card_shadow()};",

            // Top Row: Thumbnail + Info
            div {
                class: "flex items-start gap-4",

                // 1. Cover Thumbnail with 3D Spine Depth
                div {
                    class: "book-cover-realistic relative w-20 sm:w-24 aspect-[2/3] rounded-2xl flex-shrink-0 border overflow-hidden shadow-md",
                    style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",
                    img {
                        class: "w-full h-full object-cover select-none pointer-events-none",
                        src: "{thumb_url}",
                        alt: "{title}",
                        loading: "lazy",
                    }
                    div { class: "absolute inset-y-0 left-0 w-2.5 bg-gradient-to-r from-black/25 via-white/10 to-transparent pointer-events-none" }
                }

                // 2. Info Stack
                div {
                    class: "flex-1 min-w-0 flex flex-col justify-between self-stretch",
                    div {
                        h3 {
                            class: "font-serif text-[16px] sm:text-[18px] font-semibold leading-tight line-clamp-2",
                            style: "color: {theme.text_primary()};",
                            title: "{title}",
                            "{title}"
                        }
                        div { class: "flex items-center gap-1.5 mt-1.5 opacity-70",
                            svg { class: "w-3 h-3 flex-shrink-0", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
                                circle { cx: "12", cy: "7", r: "4" }
                            }
                            p { class: "font-sans text-xs truncate", style: "color: {theme.text_secondary()};",
                                "{author_display}"
                            }
                        }

                        if let Some(desc) = &book.description {
                            p {
                                class: "font-sans text-[11px] opacity-60 mt-2 line-clamp-2 leading-relaxed hidden sm:block",
                                style: "color: {theme.text_secondary()};",
                                "{desc}"
                            }
                        }
                    }

                    // Format Badges
                    div {
                        class: "flex items-center gap-1.5 mt-2.5",
                        if book.has_epub {
                            span {
                                class: "px-2 py-0.5 rounded-md text-[10px] font-sans font-bold uppercase tracking-wider border",
                                style: "background-color: {theme.accent_color()}15; color: {theme.accent_color()}; border-color: {theme.accent_color()}30;",
                                "EPUB"
                            }
                        }
                        if book.has_pdf {
                            span {
                                class: "px-2 py-0.5 rounded-md text-[10px] font-sans font-bold uppercase tracking-wider border",
                                style: "background-color: #ef444415; color: #ef4444; border-color: #ef444430;",
                                "PDF"
                            }
                        }
                    }
                }
            }

            // Bottom Action Row: Downloads count + Apple Dropdown Download Button
            div {
                class: "flex items-center justify-between pt-3 border-t mt-auto",
                style: "border-color: {theme.border_color()};",

                div {
                    class: "flex items-center gap-1.5 text-xs opacity-60 font-sans",
                    style: "color: {theme.text_muted()};",
                    svg {
                        class: "w-3.5 h-3.5",
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

                // Apple Morphing Dropdown Download Button
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
