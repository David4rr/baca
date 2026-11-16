use dioxus::prelude::*;
use crate::components::MorphingDownloadButton;
use crate::db::models::Book;
use crate::engine::ArchiveBook;
use crate::theme::ThemeMode;

#[component]
pub fn BookDetailView(
    book: ArchiveBook,
    library_books: Vec<Book>,
    theme: ThemeMode,
    on_import_book: EventHandler<Book>,
    on_open_book: EventHandler<Book>,
    on_back: EventHandler<()>,
) -> Element {
    let thumb_url = book.thumbnail_url();
    let author_display = book.author_display();
    let downloads_count = book.downloads.unwrap_or(0);
    let title = book.title.clone();
    let has_epub = book.has_epub;
    let has_pdf = book.has_pdf;

    let clean_id = book.identifier.replace(|c: char| !c.is_alphanumeric(), "_");
    let downloaded_book = library_books.iter().find(|b| {
        b.title.trim().eq_ignore_ascii_case(book.title.trim())
            || (!clean_id.is_empty() && b.file_path.contains(&clean_id))
            || b.id == book.identifier
    }).cloned();

    let is_downloaded = downloaded_book.is_some();
    let desc_text = book.description.clone().unwrap_or_else(|| {
        "Koleksi literatur domain publik berkualitas tinggi dari Project Gutenberg dan Internet Archive. Bebas dibaca tanpa batasan hak cipta.".to_string()
    });

    rsx! {
        div {
            class: "book-detail-view flex-1 flex flex-col w-full max-w-3xl mx-auto px-4 sm:px-6 pt-[calc(env(safe-area-inset-top,0px)+1rem)] pb-32 animate-fade-in transition-colors duration-300 min-w-0 select-none",

            // 1. Apple Top Navigation Bar
            div {
                class: "flex items-center justify-between gap-3 mb-8 pb-3 border-b sticky top-0 z-20 backdrop-blur-md",
                style: "border-color: {theme.border_color()}; background-color: {theme.bg_color()}E6;",
                button {
                    class: "w-10 h-10 rounded-full flex items-center justify-center border shadow-sm apple-tap transition-all",
                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    title: "Kembali",
                    onclick: move |_| on_back.call(()),
                    svg { class: "w-4 h-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                }
                h2 {
                    class: "font-serif text-lg font-medium truncate max-w-[60%]",
                    style: "color: {theme.text_primary()};",
                    "Detail Buku"
                }
                div { class: "w-10" } // spacer for symmetry
            }

            // 2. Hero Book Presentation
            div {
                class: "flex flex-col sm:flex-row items-center sm:items-start gap-6 sm:gap-8 mb-8",

                // Large Realistic Cover
                div {
                    class: "book-cover-realistic w-40 sm:w-48 aspect-[2/3] rounded-2xl overflow-hidden shadow-2xl border flex-shrink-0 relative group",
                    style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",
                    img {
                        class: "w-full h-full object-cover select-none pointer-events-none transition-transform duration-500 group-hover:scale-105",
                        src: "{thumb_url}",
                        alt: "{title}",
                    }
                    div { class: "absolute inset-y-0 left-0 w-3 bg-gradient-to-r from-black/30 via-white/10 to-transparent pointer-events-none" }
                }

                // Metadata Stack
                div {
                    class: "flex flex-col flex-1 text-center sm:text-left min-w-0 justify-between self-stretch py-1",
                    div {
                        h1 {
                            class: "font-serif text-2xl sm:text-3xl font-semibold leading-tight tracking-tight",
                            style: "color: {theme.text_primary()};",
                            "{title}"
                        }
                        p {
                            class: "font-sans text-sm sm:text-base opacity-75 mt-2 font-medium",
                            style: "color: {theme.text_secondary()};",
                            "{author_display}"
                        }

                        // Meta Chips
                        div {
                            class: "flex flex-wrap items-center justify-center sm:justify-start gap-2 mt-4",
                            span {
                                class: "px-2.5 py-1 rounded-full text-xs font-sans font-medium border flex items-center gap-1.5",
                                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_secondary()};",
                                svg { class: "w-3 h-3 opacity-60", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                    path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                    polyline { points: "7 10 12 15 17 10" }
                                    line { x1: "12", y1: "15", x2: "12", y2: "3" }
                                }
                                "{downloads_count} Unduhan"
                            }
                            span {
                                class: "px-2.5 py-1 rounded-full text-xs font-sans font-medium border bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border-emerald-500/20",
                                "Domain Publik"
                            }
                            if has_epub {
                                span {
                                    class: "px-2.5 py-1 rounded-full text-xs font-sans font-bold uppercase border",
                                    style: "background-color: {theme.accent_color()}15; color: {theme.accent_color()}; border-color: {theme.accent_color()}30;",
                                    "EPUB"
                                }
                            }
                            if has_pdf {
                                span {
                                    class: "px-2.5 py-1 rounded-full text-xs font-sans font-bold uppercase border bg-rose-500/15 text-rose-600 border-rose-500/30",
                                    "PDF"
                                }
                            }
                        }
                    }

                    // Action Buttons Row
                    div {
                        class: "mt-6 flex flex-wrap items-center justify-center sm:justify-start gap-3 pt-4 border-t",
                        style: "border-color: {theme.border_color()};",
                        if let Some(existing_book) = downloaded_book {
                            button {
                                class: "h-10 px-5 rounded-full font-sans text-xs font-semibold flex items-center gap-2 shadow-md apple-tap transition-all text-white",
                                style: "background-color: {theme.accent_color()};",
                                onclick: {
                                    let b = existing_book.clone();
                                    move |_| on_open_book.call(b.clone())
                                },
                                "📖 Buka Buku Sekarang"
                            }
                        } else {
                            MorphingDownloadButton {
                                book: book.clone(),
                                theme: theme,
                                is_already_downloaded: is_downloaded,
                                on_imported: move |b| on_import_book.call(b),
                            }
                        }
                    }
                }
            }

            // 3. Synopsis / Description Section
            div {
                class: "rounded-3xl p-6 sm:p-7 border shadow-sm select-text",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",
                h3 {
                    class: "font-serif text-lg font-semibold tracking-tight mb-3 select-none",
                    style: "color: {theme.text_primary()};",
                    "Sinopsis & Ringkasan"
                }
                div {
                    class: "font-sans text-sm sm:text-base leading-relaxed opacity-85 text-justify whitespace-pre-line",
                    style: "color: {theme.text_primary()};",
                    "{desc_text}"
                }
            }
        }
    }
}
