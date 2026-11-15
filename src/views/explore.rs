use dioxus::prelude::*;
use crate::engine::{download_archive_book, get_archive_epub_file, parse_epub, search_archive, ArchiveBook};
use crate::db::models::Book;
use crate::theme::ThemeMode;
use std::collections::HashSet;

#[component]
pub fn ExploreView(
    theme: ThemeMode,
    on_import_book: EventHandler<Book>,
    on_back: EventHandler<()>,
) -> Element {
    let mut search_query = use_signal(|| "".to_string());
    let mut results = use_signal(|| Vec::<ArchiveBook>::new());
    let mut is_loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);
    let mut downloading_ids = use_signal(|| HashSet::<String>::new());
    let mut success_ids = use_signal(|| HashSet::<String>::new());

    // Initial search load (popular free classics)
    use_effect(move || {
        spawn(async move {
            is_loading.set(true);
            match search_archive("classics fiction", 1).await {
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
                        let target_path = books_dir.join(format!("{}.epub", id));

                        match download_archive_book(&id, &epub_name, &target_path).await {
                            Ok(()) => {
                                if let Ok(parsed) = parse_epub(&target_path) {
                                    let now = chrono::Utc::now().timestamp();
                                    let new_book = Book {
                                        id: id.clone(),
                                        title: parsed.title,
                                        author: parsed.author,
                                        file_path: target_path.to_string_lossy().to_string(),
                                        format: "epub".to_string(),
                                        cover_image: parsed.cover_image,
                                        total_chapters: parsed.chapters.len(),
                                        current_chapter: 0,
                                        progress_percent: 0.0,
                                        last_read_at: now,
                                        created_at: now,
                                    };
                                    on_import.call(new_book);
                                    success_ids.write().insert(id.clone());
                                }
                            }
                            Err(e) => {
                                error_msg.set(Some(format!("Gagal mengunduh: {}", e)));
                            }
                        }
                    }
                    Ok(None) => {
                        error_msg.set(Some(format!(
                            "Format EPUB tidak tersedia untuk '{}'. Tersedia dalam format lain di Archive.org.",
                            book_clone.title
                        )));
                    }
                    Err(e) => {
                        error_msg.set(Some(format!("Gagal mengambil metadata: {}", e)));
                    }
                }
                downloading_ids.write().remove(&id);
            });
        }
    };

    let categories = [
        ("Semua", "classics fiction"),
        ("Sastra", "literature"),
        ("Filsafat", "philosophy"),
        ("Sains", "science"),
        ("Sejarah", "history"),
        ("Indonesia", "indonesia"),
    ];

    let mut search_for_input = search_fn.clone();

    rsx! {
        div {
            class: "explore-view flex-1 flex flex-col p-6 max-w-7xl mx-auto w-full",

            // Header & Search
            div {
                class: "flex flex-col md:flex-row md:items-center justify-between gap-4 mb-6",
                div {
                    class: "flex items-center gap-3",
                    button {
                        class: "p-2 rounded-xl border border-white/10 hover:bg-white/10 transition-colors text-sm font-semibold flex items-center gap-2",
                        onclick: move |_| on_back.call(()),
                        "← Kembali ke Rak"
                    }
                    div {
                        h2 { class: "text-2xl font-bold tracking-tight", "Cari & Unduh Buku Online" }
                        p { class: "text-xs opacity-60", "Jutaan buku domain publik bebas unduh dari Internet Archive" }
                    }
                }

                // Search Input
                div {
                    class: "relative w-full md:w-96",
                    input {
                        r#type: "text",
                        class: "w-full px-4 py-2.5 pl-10 rounded-2xl border text-sm transition-all outline-none focus:ring-2",
                        style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        placeholder: "Ketik judul buku atau nama penulis...",
                        value: "{search_query}",
                        oninput: move |e| search_for_input(e.value()),
                    }
                    span {
                        class: "absolute left-3.5 top-2.5 text-base opacity-40 pointer-events-none",
                        "🔍"
                    }
                }
            }

            // Category Chips
            div {
                class: "flex flex-wrap gap-2 mb-6",
                for (name, query) in categories {
                    {
                        let mut do_chip_search = search_fn.clone();
                        let q = query.to_string();
                        rsx! {
                            button {
                                class: "px-3.5 py-1.5 rounded-full text-xs font-medium border transition-colors hover:opacity-100",
                                style: if search_query() == query {
                                    format!("background-color: {}; color: #000; border-color: transparent;", theme.accent_color())
                                } else {
                                    format!("background-color: {}; border-color: {}; color: {}; opacity: 0.8;", theme.surface_color(), theme.border_color(), theme.text_primary())
                                },
                                onclick: move |_| do_chip_search(q.clone()),
                                "{name}"
                            }
                        }
                    }
                }
            }

            // Error banner if any
            if let Some(err) = error_msg() {
                div {
                    class: "mb-6 p-4 rounded-2xl bg-rose-500/10 border border-rose-500/20 text-rose-300 text-xs flex items-center justify-between",
                    span { "{err}" }
                    button {
                        class: "opacity-60 hover:opacity-100 font-bold ml-4",
                        onclick: move |_| error_msg.set(None),
                        "✕"
                    }
                }
            }

            // Content Grid
            if *is_loading.read() {
                div {
                    class: "flex-1 flex flex-col items-center justify-center py-20 opacity-60 gap-3",
                    div { class: "w-8 h-8 border-2 border-white/20 border-t-white rounded-full animate-spin" }
                    p { class: "text-sm", "Mencari katalog buku di Internet Archive..." }
                }
            } else if results.read().is_empty() {
                div {
                    class: "flex-1 flex flex-col items-center justify-center py-20 opacity-50 text-center",
                    p { class: "text-base font-semibold mb-1", "Tidak ada hasil yang ditemukan" }
                    p { class: "text-xs", "Coba gunakan kata kunci judul atau nama pengarang lain." }
                }
            } else {
                div {
                    class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4",
                    for book in results.read().iter() {
                        {
                            let id = book.identifier.clone();
                            let is_downloading = downloading_ids.read().contains(&id);
                            let is_downloaded = success_ids.read().contains(&id);
                            let book_to_dl = book.clone();
                            let mut do_download = on_download.clone();

                            rsx! {
                                div {
                                    key: "{id}",
                                    class: "flex flex-col p-4 rounded-2xl border transition-all duration-200 hover:shadow-lg",
                                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",

                                    div {
                                        class: "flex gap-3 mb-3",
                                        // Cover Image
                                        div {
                                            class: "w-16 h-24 rounded-lg overflow-hidden shrink-0 bg-black/20 flex items-center justify-center relative",
                                            img {
                                                src: "https://archive.org/services/img/{id}",
                                                class: "w-full h-full object-cover",
                                                loading: "lazy",
                                            }
                                        }

                                        // Metadata
                                        div {
                                            class: "flex-1 min-w-0 flex flex-col",
                                            h4 {
                                                class: "text-sm font-bold leading-tight line-clamp-2 mb-1",
                                                "{book.title}"
                                            }
                                            if let Some(creator) = &book.creator {
                                                p {
                                                    class: "text-xs opacity-70 truncate mb-1",
                                                    "{creator}"
                                                }
                                            }
                                            if let Some(dl) = book.downloads {
                                                p {
                                                    class: "text-[10px] opacity-50 flex items-center gap-1 mt-auto",
                                                    "📥 {dl} downloads"
                                                }
                                            }
                                        }
                                    }

                                    // Action Button
                                    button {
                                        class: "mt-auto w-full py-2 px-3 rounded-xl text-xs font-semibold flex items-center justify-center gap-2 transition-all",
                                        disabled: is_downloading || is_downloaded,
                                        style: if is_downloaded {
                                            "background-color: rgba(34, 197, 94, 0.2); color: #4ade80; border: 1px solid rgba(34, 197, 94, 0.3);".to_string()
                                        } else if is_downloading {
                                            "background-color: rgba(255, 255, 255, 0.1); color: #ccc; cursor: not-allowed;".to_string()
                                        } else {
                                            format!("background-color: {}; color: #000; font-weight: 700;", theme.accent_color())
                                        },
                                        onclick: move |_| do_download(book_to_dl.clone()),

                                        if is_downloaded {
                                            "✓ Tersimpan di Rak"
                                        } else if is_downloading {
                                            "⏳ Mengunduh EPUB..."
                                        } else {
                                            "⬇ Unduh EPUB Gratis"
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
