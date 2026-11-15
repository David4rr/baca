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
                        let dest_path = books_dir.join(format!("{}.epub", id));

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
            class: "explore-view flex-1 flex flex-col max-w-4xl mx-auto w-full px-5 sm:px-8 pt-8 pb-32 animate-fade-in transition-colors duration-300",

            // 1. Apple-Standard Header & Circular Back Button
            div {
                class: "flex items-start justify-between gap-4 mb-8",
                div {
                    class: "flex items-center gap-3.5",
                    button {
                        class: "w-10 h-10 rounded-full flex items-center justify-center shadow-sm border apple-tap transition-all flex-shrink-0",
                        style: "width: 40px; height: 40px; min-width: 40px; min-height: 40px; background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        title: "Kembali ke Rak Buku",
                        onclick: move |_| on_back.call(()),
                        svg {
                            class: "w-4 h-4",
                            width: "16",
                            height: "16",
                            style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "15 18 9 12 15 6" }
                        }
                    }
                    div {
                        h1 {
                            class: "font-serif text-[28px] sm:text-[34px] font-medium tracking-tight leading-none",
                            style: "color: {theme.text_primary()};",
                            "Jelajahi Arsip Buku"
                        }
                        p {
                            class: "font-sans text-[13px] mt-1.5 opacity-70",
                            style: "color: {theme.text_secondary()};",
                            "Jutaan buku domain publik bebas dari Internet Archive"
                        }
                    }
                }
            }

            // 2. Search Bar
            div {
                class: "mb-6",
                div {
                    class: "relative w-full",
                    input {
                        r#type: "text",
                        class: "w-full pl-11 pr-4 py-3 rounded-2xl border text-sm transition-all outline-none focus:ring-2",
                        style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        placeholder: "Ketik judul buku, nama penulis, atau topik...",
                        value: "{search_query}",
                        oninput: move |e| search_for_input(e.value()),
                    }
                    div {
                        class: "absolute left-4 top-1/2 -translate-y-1/2 opacity-40 pointer-events-none",
                        svg {
                            class: "w-4 h-4",
                            width: "16",
                            height: "16",
                            style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            circle { cx: "11", cy: "11", r: "8" }
                            line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                        }
                    }
                }
            }

            // 3. Category Chips (Apple pill style)
            div {
                class: "flex flex-wrap gap-2 mb-8",
                for (name, query) in categories {
                    {
                        let mut do_chip_search = search_fn.clone();
                        let q = query.to_string();
                        rsx! {
                            button {
                                class: "px-4 py-1.5 rounded-full text-xs font-sans font-medium border apple-tap transition-all shadow-sm",
                                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_secondary()};",
                                onclick: move |_| do_chip_search(q.clone()),
                                "{name}"
                            }
                        }
                    }
                }
            }

            // 4. Error Message Banner
            if let Some(err) = error_msg.read().as_ref() {
                div {
                    class: "p-4 mb-6 rounded-2xl border text-xs text-rose-500 font-sans animate-fade-in",
                    style: "background-color: rgba(244, 63, 94, 0.08); border-color: rgba(244, 63, 94, 0.2);",
                    "{err}"
                }
            }

            // 5. Loading State
            if *is_loading.read() {
                div {
                    class: "flex flex-col items-center justify-center p-16 text-center animate-fade-in",
                    div {
                        class: "w-8 h-8 border-2 border-t-transparent rounded-full animate-spin mb-4",
                        style: "border-color: {theme.text_primary()}; border-top-color: transparent;",
                    }
                    p { class: "text-xs font-sans opacity-60", "Mencari buku di perpustakaan dunia..." }
                }
            } else if results.read().is_empty() {
                div {
                    class: "p-12 text-center rounded-[28px] border border-dashed my-8",
                    style: "border-color: {theme.border_color()}; background-color: {theme.surface_color()};",
                    h3 {
                        class: "font-serif text-xl font-medium mb-1",
                        style: "color: {theme.text_primary()};",
                        "Tidak Ada Hasil"
                    }
                    p {
                        class: "text-xs font-sans opacity-60",
                        style: "color: {theme.text_secondary()};",
                        "Coba gunakan kata kunci pencarian yang berbeda."
                    }
                }
            } else {
                // 6. Book Results Grid
                div {
                    class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-5 sm:gap-6",
                    for book in results.read().iter() {
                        {
                            let is_downloading = downloading_ids.read().contains(&book.identifier);
                            let is_success = success_ids.read().contains(&book.identifier);
                            let book_data = book.clone();
                            let mut trigger_download = on_download.clone();

                            rsx! {
                                div {
                                    key: "{book.identifier}",
                                    class: "group relative rounded-2xl p-3 border flex flex-col justify-between transition-all duration-300 shadow-sm",
                                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; box-shadow: {theme.card_shadow()};",

                                    // Cover Image with 3D depth
                                    div {
                                        class: "book-cover-realistic w-full aspect-[2/3] rounded-xl overflow-hidden mb-3",
                                        style: "background-color: {theme.surface_subtle()};",
                                        {
                                            let thumb = book.thumbnail_url();
                                            rsx! {
                                                img {
                                                    class: "w-full h-full object-cover",
                                                    src: "{thumb}",
                                                    alt: "{book.title}",
                                                    loading: "lazy",
                                                }
                                            }
                                        }
                                    }

                                    // Metadata
                                    div {
                                        class: "flex-1 flex flex-col justify-between",
                                        div {
                                            h3 {
                                                class: "font-serif text-[14px] font-semibold line-clamp-2 leading-snug mb-1",
                                                style: "color: {theme.text_primary()};",
                                                title: "{book.title}",
                                                "{book.title}"
                                            }
                                            p {
                                                class: "font-sans text-[12px] opacity-70 truncate mb-3",
                                                style: "color: {theme.text_secondary()};",
                                                "{book.creator.clone().unwrap_or_else(|| \"Penulis Tidak Diketahui\".to_string())}"
                                            }
                                        }

                                        // Action Button
                                        button {
                                            class: "w-full h-9 rounded-xl text-xs font-sans font-medium flex items-center justify-center gap-1.5 apple-tap transition-all shadow-sm disabled:pointer-events-none",
                                            style: if is_success {
                                                "background-color: #10B981; color: #FFF;".to_string()
                                            } else {
                                                format!("background-color: {}; color: {};", theme.accent_color(), theme.accent_contrast_text())
                                            },
                                            disabled: is_downloading || is_success,
                                            onclick: move |_| trigger_download(book_data.clone()),

                                            if is_downloading {
                                                div { class: "w-3.5 h-3.5 border-2 border-t-transparent border-white rounded-full animate-spin" }
                                                span { "Mengunduh..." }
                                            } else if is_success {
                                                svg {
                                                    class: "w-3.5 h-3.5",
                                                    width: "14",
                                                    height: "14",
                                                    style: "width: 14px; height: 14px; min-width: 14px; min-height: 14px;",
                                                    view_box: "0 0 24 24",
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    stroke_width: "2.5",
                                                    polyline { points: "20 6 9 17 4 12" }
                                                }
                                                span { "Tersimpan di Rak" }
                                            } else {
                                                svg {
                                                    class: "w-3.5 h-3.5",
                                                    width: "14",
                                                    height: "14",
                                                    style: "width: 14px; height: 14px; min-width: 14px; min-height: 14px;",
                                                    view_box: "0 0 24 24",
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    stroke_width: "2.5",
                                                    path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                                    polyline { points: "7 10 12 15 17 10" }
                                                    line { x1: "12", y1: "15", x2: "12", y2: "3" }
                                                }
                                                span { "Unduh EPUB" }
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
