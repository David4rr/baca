use dioxus::prelude::*;
use crate::engine::ArchiveBook;
use crate::theme::ThemeMode;

#[component]
pub fn ExploreBookItem(
    book: ArchiveBook,
    theme: ThemeMode,
    is_downloading: bool,
    is_downloaded: bool,
    on_download: EventHandler<ArchiveBook>,
) -> Element {
    let book_clone = book.clone();
    let thumb_url = book.thumbnail_url();
    let author_display = book.creator.clone().unwrap_or_else(|| "Pengarang Tidak Diketahui".to_string());
    let downloads_count = book.downloads.unwrap_or(0);

    rsx! {
        div {
            class: "explore-book-item rounded-2xl p-4 sm:p-5 border transition-all duration-300 flex items-start gap-4 select-none shadow-sm",
            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; box-shadow: {theme.card_shadow()};",

            // 1. Book Thumbnail with 3D Spine Depth
            div {
                class: "book-cover-realistic w-16 sm:w-20 aspect-[2/3] rounded-xl flex-shrink-0 border overflow-hidden",
                style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",

                img {
                    class: "w-full h-full object-cover select-none pointer-events-none",
                    src: "{thumb_url}",
                    alt: "{book.title}",
                    loading: "lazy",
                }
            }

            // 2. Book Info Stack
            div {
                class: "flex-1 min-w-0 flex flex-col justify-between self-stretch",

                div {
                    h3 {
                        class: "font-serif text-[16px] sm:text-[18px] font-medium leading-tight line-clamp-2",
                        style: "color: {theme.text_primary()};",
                        title: "{book.title}",
                        "{book.title}"
                    }
                    p {
                        class: "font-sans text-[12px] opacity-70 mt-1 truncate",
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
                }

                // 3. Action Row (Downloads badge & Download Button)
                div {
                    class: "mt-4 flex items-center justify-between gap-3 pt-2",

                    div {
                        class: "flex items-center gap-1.5 text-[11px] opacity-60 font-sans",
                        style: "color: {theme.text_muted()};",
                        svg {
                            class: "w-3.5 h-3.5",
                            width: "14",
                            height: "14",
                            style: "width: 14px; height: 14px; min-width: 14px; min-height: 14px;",
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

                    if is_downloaded {
                        span {
                            class: "text-[12px] font-sans font-medium text-emerald-600 dark:text-emerald-400 flex items-center gap-1",
                            "✓ Tersedia di Rak"
                        }
                    } else if is_downloading {
                        div {
                            class: "flex items-center gap-2 px-3 py-1.5 rounded-xl text-xs font-sans font-medium opacity-80",
                            style: "background-color: {theme.surface_subtle()}; color: {theme.text_secondary()};",
                            div { class: "w-3 h-3 border-2 border-stone-400 border-t-transparent rounded-full animate-spin" }
                            span { "Mengunduh..." }
                        }
                    } else {
                        button {
                            class: "px-4 py-1.5 rounded-xl text-xs font-sans font-medium apple-tap transition-all shadow-sm flex items-center gap-1.5",
                            style: "background-color: {theme.accent_color()}; color: {theme.accent_contrast_text()};",
                            onclick: move |_| on_download.call(book_clone.clone()),
                            "Unduh EPUB"
                        }
                    }
                }
            }
        }
    }
}
