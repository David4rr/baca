pub mod book_item;
pub mod header;

use dioxus::prelude::*;
use crate::db::models::Book;
use crate::engine::{search_archive, ArchiveBook};
use crate::theme::ThemeMode;
pub use book_item::ExploreBookItem;
pub use header::ExploreHeader;

#[component]
pub fn ExploreView(
    theme: ThemeMode,
    library_books: Vec<Book>,
    on_import_book: EventHandler<Book>,
    on_select_detail: EventHandler<ArchiveBook>,
    on_back: EventHandler<()>,
) -> Element {
    let mut search_query = use_signal(|| "".to_string());
    let mut results = use_signal(Vec::<ArchiveBook>::new);
    let mut is_loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);

    use_effect(move || {
        spawn(async move {
            is_loading.set(true);
            match search_archive("classics", 1).await {
                Ok(books) => results.set(books),
                Err(e) => error_msg.set(Some(e)),
            }
            is_loading.set(false);
        });
    });

    let search_fn = {
        move |query: String| {
            let q = query.trim().to_string();
            search_query.set(q.clone());
            spawn(async move {
                is_loading.set(true);
                error_msg.set(None);
                match search_archive(&q, 1).await {
                    Ok(books) => results.set(books),
                    Err(e) => error_msg.set(Some(e)),
                }
                is_loading.set(false);
            });
        }
    };

    let categories = [
        ("Semua", "classics"),
        ("Sherlock", "sherlock holmes"),
        ("Filsafat", "philosophy"),
        ("Sains", "science"),
        ("Sejarah", "history"),
        ("Austen", "jane austen"),
    ];

    let search_for_header = search_fn;

    rsx! {
        div {
            class: "explore-view flex-1 flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 md:px-8 pt-[calc(env(safe-area-inset-top,0px)+3.25rem)] pb-32 animate-fade-in transition-colors duration-300 min-w-0",

            // 1. Symmetrical Header
            ExploreHeader {
                search_query: search_query.read().clone(),
                theme: theme,
                on_back: move |_| on_back.call(()),
                on_search: search_for_header,
            }

            // 2. Symmetrical Category Pills
            div {
                class: "flex items-center gap-2 overflow-x-auto pb-4 mb-6 select-none w-full min-w-0 max-w-full",
                for (name, q) in categories {
                    {
                        let q_str = q.to_string();
                        let is_active = *search_query.read() == q_str;
                        let mut search_cat = search_fn;
                        rsx! {
                            button {
                                class: "px-3.5 py-1.5 rounded-full text-xs font-sans font-medium whitespace-nowrap apple-tap transition-all border",
                                style: if is_active {
                                    format!("background-color: {}; color: {}; border-color: transparent;", theme.accent_color(), theme.accent_contrast_text())
                                } else {
                                    format!("background-color: {}; color: {}; border-color: {};", theme.surface_color(), theme.text_secondary(), theme.border_color())
                                },
                                onclick: move |_| search_cat(q_str.clone()),
                                "{name}"
                            }
                        }
                    }
                }
            }

            // 3. Error Banner
            if let Some(err) = error_msg.read().as_ref() {
                div {
                    class: "mb-6 p-4 rounded-2xl border text-xs font-sans text-rose-500 animate-slide-up flex items-center justify-between w-full",
                    style: "background-color: {theme.surface_color()}; border-color: rgba(244, 63, 94, 0.2);",
                    span { "{err}" }
                    button {
                        class: "ml-2 opacity-60 hover:opacity-100 p-1",
                        onclick: move |_| error_msg.set(None),
                        svg { class: "w-3.5 h-3.5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5",
                            line { x1: "18", y1: "6", x2: "6", y2: "18" }
                            line { x1: "6", y1: "6", x2: "18", y2: "18" }
                        }
                    }
                }
            }

            // 4. Results Grid with Balanced Margins
            if *is_loading.read() {
                div {
                    class: "flex-1 flex flex-col items-center justify-center p-16 animate-fade-in w-full",
                    div {
                        class: "w-8 h-8 border-2 border-t-transparent rounded-full animate-spin mb-4",
                        style: "border-color: {theme.accent_color()}; border-top-color: transparent;",
                    }
                    p {
                        class: "text-xs font-sans opacity-60",
                        style: "color: {theme.text_secondary()};",
                        "Mencari katalog buku online..."
                    }
                }
            } else if results.read().is_empty() {
                div {
                    class: "flex-1 flex flex-col items-center justify-center p-16 text-center opacity-60 animate-fade-in w-full",
                    p {
                        class: "font-serif text-lg mb-1",
                        style: "color: {theme.text_primary()};",
                        "Tidak ada buku yang cocok"
                    }
                    p {
                        class: "text-xs font-sans",
                        style: "color: {theme.text_secondary()};",
                        "Coba gunakan kata kunci pencarian yang lebih umum."
                    }
                }
            } else {
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-5 w-full animate-fade-in",
                    for book in results.read().iter() {
                        {
                            let id = book.identifier.clone();
                            let clean_id = id.replace(|c: char| !c.is_alphanumeric(), "_");
                            let is_done = library_books.iter().any(|b| {
                                b.title.trim().eq_ignore_ascii_case(book.title.trim())
                                    || (!clean_id.is_empty() && b.file_path.contains(&clean_id))
                                    || b.id == id
                            });
                            let on_imp = on_import_book;
                            let on_det = on_select_detail;
                            rsx! {
                                ExploreBookItem {
                                    key: "{id}",
                                    book: book.clone(),
                                    theme: theme,
                                    is_downloaded: is_done,
                                    on_imported: move |b| on_imp.call(b),
                                    on_select: move |b| on_det.call(b),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
