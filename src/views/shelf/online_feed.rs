use dioxus::prelude::*;
use crate::components::MorphingDownloadButton;
use crate::db::models::Book;
use crate::engine::{search_archive, ArchiveBook};
use crate::theme::ThemeMode;

#[component]
pub fn OnlineFeed(
    theme: ThemeMode,
    downloaded_ids: Vec<String>,
    on_imported_book: EventHandler<Book>,
) -> Element {
    let mut selected_category = use_signal(|| "classics".to_string());
    let mut online_books = use_signal(Vec::<ArchiveBook>::new);
    let mut is_loading = use_signal(|| false);

    let categories = [
        ("Klasik", "classics"),
        ("Filsafat", "philosophy"),
        ("Sains", "science"),
        ("Sejarah", "history"),
        ("Fiksi", "fiction"),
    ];

    use_effect(move || {
        let cat = selected_category.read().clone();
        spawn(async move {
            is_loading.set(true);
            if let Ok(books) = search_archive(&cat, 1).await {
                online_books.set(books);
            }
            is_loading.set(false);
        });
    });

    rsx! {
        div {
            class: "mt-12 flex flex-col gap-5 w-full",

            // Section Header
            div {
                class: "flex flex-col sm:flex-row sm:items-end justify-between gap-3 select-none w-full",
                div {
                    class: "flex flex-col",
                    h2 {
                        class: "font-serif text-2xl font-medium tracking-tight",
                        style: "color: {theme.text_primary()};",
                        "Jelajahi Literatur Online"
                    }
                    p {
                        class: "font-sans text-xs opacity-60 mt-0.5",
                        style: "color: {theme.text_secondary()};",
                        "Koleksi domain publik dari Project Gutenberg & Internet Archive"
                    }
                }

                // Category Chips
                div {
                    class: "flex items-center gap-1.5 overflow-x-auto pb-1 select-none",
                    for (label, query) in categories {
                        {
                            let q_str = query.to_string();
                            let is_active = *selected_category.read() == q_str;
                            rsx! {
                                button {
                                    class: "px-3 py-1 rounded-full text-xs font-sans font-medium whitespace-nowrap apple-tap transition-all border",
                                    style: if is_active {
                                        format!("background-color: {}; color: {}; border-color: transparent;", theme.accent_color(), theme.accent_contrast_text())
                                    } else {
                                        format!("background-color: {}; color: {}; border-color: {};", theme.surface_color(), theme.text_secondary(), theme.border_color())
                                    },
                                    onclick: move |_| selected_category.set(q_str.clone()),
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }

            // Books Grid or Loading
            if *is_loading.read() && online_books.read().is_empty() {
                div {
                    class: "py-12 flex flex-col items-center justify-center gap-3 w-full",
                    div {
                        class: "w-6 h-6 border-2 border-t-transparent rounded-full animate-spin",
                        style: "border-color: {theme.accent_color()}; border-top-color: transparent;",
                    }
                    span {
                        class: "text-xs font-sans opacity-60",
                        style: "color: {theme.text_secondary()};",
                        "Memuat katalog literatur..."
                    }
                }
            } else {
                div {
                    class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 w-full",
                    for book in online_books.read().iter().take(6) {
                        {
                            let id = book.identifier.clone();
                            let is_done = downloaded_ids.contains(&id);
                            let on_imp = on_imported_book.clone();
                            rsx! {
                                div {
                                    key: "{id}",
                                    class: "p-4 rounded-3xl border flex gap-3.5 items-center shadow-sm hover:shadow-md transition-all justify-between",
                                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",

                                    // Cover Thumbnail with Depth
                                    div {
                                        class: "book-cover-realistic w-14 h-20 rounded-xl overflow-hidden flex-shrink-0 shadow-sm border",
                                        style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",
                                        img {
                                            class: "w-full h-full object-cover",
                                            src: "{book.thumbnail_url()}",
                                            alt: "{book.title}",
                                            loading: "lazy",
                                        }
                                    }

                                    // Details & Morphing Download Button
                                    div {
                                        class: "flex flex-col flex-1 min-w-0 justify-between h-20 py-0.5",
                                        div {
                                            class: "flex flex-col",
                                            h4 {
                                                class: "font-serif text-sm font-semibold leading-tight line-clamp-2",
                                                style: "color: {theme.text_primary()};",
                                                "{book.title}"
                                            }
                                            span {
                                                class: "text-[11px] font-sans opacity-60 truncate mt-0.5",
                                                style: "color: {theme.text_secondary()};",
                                                "{book.author_display()}"
                                            }
                                        }

                                        div {
                                            class: "flex items-center justify-between mt-auto pt-1",
                                            div {
                                                class: "flex items-center gap-1",
                                                if book.has_epub {
                                                    span {
                                                        class: "px-1.5 py-0.5 rounded text-[9px] font-sans font-bold uppercase",
                                                        style: "background-color: {theme.accent_color()}18; color: {theme.accent_color()};",
                                                        "EPUB"
                                                    }
                                                }
                                                if book.has_pdf {
                                                    span {
                                                        class: "px-1.5 py-0.5 rounded text-[9px] font-sans font-bold uppercase",
                                                        style: "background-color: #ef444418; color: #ef4444;",
                                                        "PDF"
                                                    }
                                                }
                                            }

                                            MorphingDownloadButton {
                                                book: book.clone(),
                                                theme: theme,
                                                is_already_downloaded: is_done,
                                                on_imported: move |b| on_imp.call(b),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
