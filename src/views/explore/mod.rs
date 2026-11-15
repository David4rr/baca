pub mod book_item;
pub mod header;

use dioxus::prelude::*;
use crate::engine::{download_archive_book, get_archive_epub_file, parse_epub, search_archive, ArchiveBook};
use crate::db::models::Book;
use crate::theme::ThemeMode;
pub use book_item::ExploreBookItem;
pub use header::ExploreHeader;
use std::collections::HashSet;

#[component]
pub fn ExploreView(
    theme: ThemeMode,
    on_import_book: EventHandler<Book>,
    on_back: EventHandler<()>,
) -> Element {
    let mut search_query = use_signal(|| "".to_string());
    let mut results = use_signal(Vec::<ArchiveBook>::new);
    let mut is_loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);
    let mut downloading_ids = use_signal(HashSet::<String>::new);
    let mut success_ids = use_signal(HashSet::<String>::new);

    // Initial search load
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

    let on_download = {
        let on_import_book = on_import_book.clone();
        move |book: ArchiveBook| {
            let id = book.identifier.clone();
            let book_clone = book.clone();
            let on_import = on_import_book.clone();
            downloading_ids.write().insert(id.clone());

            spawn(async move {
                match get_archive_epub_file(&id).await {
                    Ok(Some(epub_name)) => {
                        let app_dir = crate::db::Database::get_app_dir();
                        let books_dir = app_dir.join("books");
                        let _ = std::fs::create_dir_all(&books_dir);
                        let clean_filename = id.replace(|c: char| !c.is_alphanumeric(), "_");
                        let dest_path = books_dir.join(format!("{}.epub", clean_filename));

                        match download_archive_book(&id, &epub_name, &dest_path).await {
                            Ok(_) => {
                                if let Ok(parsed) = parse_epub(&dest_path) {
                                    let now = chrono::Utc::now().timestamp();
                                    let new_book = Book {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        title: if !parsed.title.is_empty() { parsed.title } else { book_clone.title.clone() },
                                        author: if !parsed.author.is_empty() { parsed.author } else { book_clone.creator.clone().unwrap_or_else(|| "Unknown".to_string()) },
                                        file_path: dest_path.to_string_lossy().to_string(),
                                        format: "epub".to_string(),
                                        cover_image: parsed.cover_image.or_else(|| Some(book_clone.thumbnail_url())),
                                        total_chapters: parsed.chapters.len(),
                                        current_chapter: 0,
                                        progress_percent: 0.0,
                                        last_read_at: now,
                                        created_at: now,
                                    };

                                    if let Ok(db) = crate::db::Database::init() {
                                        let _ = db.insert_or_update_book(&new_book);
                                    }

                                    success_ids.write().insert(id.clone());
                                    on_import.call(new_book);
                                }
                            }
                            Err(e) => {
                                error_msg.set(Some(format!("Gagal mengunduh: {}", e)));
                            }
                        }
                    }
                    Ok(None) => {
                        error_msg.set(Some("Format EPUB tidak tersedia untuk buku ini.".to_string()));
                    }
                    Err(e) => {
                        error_msg.set(Some(format!("Error: {}", e)));
                    }
                }
                downloading_ids.write().remove(&id);
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

    let mut search_for_header = search_fn.clone();

    rsx! {
        div {
            class: "explore-view flex-1 flex flex-col max-w-4xl mx-auto w-full px-5 sm:px-8 pt-8 pb-32 animate-fade-in transition-colors duration-300",

            // 1. Header & Search Input
            ExploreHeader {
                search_query: search_query.read().clone(),
                theme: theme,
                on_back: move |_| on_back.call(()),
                on_search: move |q| search_for_header(q),
            }

            // 2. Category Pills
            div {
                class: "flex items-center gap-2 overflow-x-auto pb-4 mb-6 select-none",
                for (name, q) in categories {
                    {
                        let q_str = q.to_string();
                        let is_active = *search_query.read() == q_str;
                        let mut search_cat = search_fn.clone();
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
                    class: "mb-6 p-4 rounded-2xl border text-xs font-sans text-rose-500 animate-slide-up flex items-center justify-between",
                    style: "background-color: {theme.surface_color()}; border-color: rgba(244, 63, 94, 0.2);",
                    span { "{err}" }
                    button {
                        class: "ml-2 opacity-60 hover:opacity-100",
                        onclick: move |_| error_msg.set(None),
                        "✕"
                    }
                }
            }

            // 4. Results Grid or Loading
            if *is_loading.read() {
                div {
                    class: "flex-1 flex flex-col items-center justify-center p-16 animate-fade-in",
                    div { class: "w-8 h-8 border-2 border-stone-400 border-t-transparent rounded-full animate-spin mb-4" }
                    p { class: "text-xs font-sans opacity-60", "Mencari buku di arsip online..." }
                }
            } else if results.read().is_empty() {
                div {
                    class: "flex-1 flex flex-col items-center justify-center p-16 text-center opacity-60 animate-fade-in",
                    p { class: "font-serif text-lg mb-1", "Tidak ada buku yang cocok" }
                    p { class: "text-xs font-sans", "Coba gunakan kata kunci pencarian yang lebih umum." }
                }
            } else {
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-4 animate-fade-in",
                    for book in results.read().iter() {
                        {
                            let id = book.identifier.clone();
                            let is_down = downloading_ids.read().contains(&id);
                            let is_done = success_ids.read().contains(&id);
                            let mut download_handler = on_download.clone();
                            rsx! {
                                ExploreBookItem {
                                    key: "{id}",
                                    book: book.clone(),
                                    theme: theme,
                                    is_downloading: is_down,
                                    is_downloaded: is_done,
                                    on_download: move |b| download_handler(b),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
