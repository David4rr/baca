use dioxus::prelude::*;
use crate::components::BookCard;
use crate::db::models::Book;
use crate::theme::ThemeMode;

#[component]
pub fn ShelfView(
    books: Vec<Book>,
    theme: ThemeMode,
    search_query: String,
    on_search: EventHandler<String>,
    on_select_book: EventHandler<Book>,
    on_delete_book: EventHandler<String>,
    on_import: EventHandler<()>,
    on_explore: EventHandler<()>,
    on_stats: EventHandler<()>,
    on_settings: EventHandler<()>,
) -> Element {
    let mut is_search_visible = use_signal(|| false);

    let query = search_query.to_lowercase();
    let filtered_books: Vec<Book> = books
        .iter()
        .filter(|b| {
            if query.is_empty() {
                true
            } else {
                b.title.to_lowercase().contains(&query) || b.author.to_lowercase().contains(&query)
            }
        })
        .cloned()
        .collect();

    let featured_book = filtered_books.first().cloned();
    let recent_books: Vec<Book> = if filtered_books.len() > 1 {
        filtered_books.iter().skip(1).take(6).cloned().collect()
    } else {
        vec![]
    };

    rsx! {
        div {
            class: "shelf-view flex-1 flex flex-col w-full max-w-4xl mx-auto px-5 sm:px-8 pt-8 pb-32 transition-colors duration-300",

            // 1. Apple-Standard Editorial Header
            div {
                class: "flex items-start justify-between mb-8 animate-fade-in",
                div {
                    class: "flex flex-col",
                    h1 {
                        class: "font-serif text-[34px] sm:text-[40px] font-medium tracking-tight leading-none",
                        style: "color: {theme.text_primary()};",
                        "My Library"
                    }
                    p {
                        class: "font-sans text-[13px] sm:text-[14px] mt-2 font-normal tracking-normal opacity-70",
                        style: "color: {theme.text_secondary()};",
                        "Pick up where you left off"
                    }
                }

                // Top Right Action Buttons (Search toggle & Import button)
                div {
                    class: "flex items-center gap-2.5",

                    // Import Button (+)
                    button {
                        class: "w-10 h-10 rounded-full flex items-center justify-center shadow-sm border apple-tap transition-all",
                        style: "width: 40px; height: 40px; min-width: 40px; min-height: 40px; background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        title: "Import Buku (EPUB)",
                        onclick: move |_| on_import.call(()),
                        svg {
                            class: "w-4 h-4",
                            width: "16",
                            height: "16",
                            style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line { x1: "12", y1: "5", x2: "12", y2: "19" }
                            line { x1: "5", y1: "12", x2: "19", y2: "12" }
                        }
                    }

                    // Circular Search Button (matching reference)
                    button {
                        class: "w-10 h-10 rounded-full flex items-center justify-center shadow-sm border apple-tap transition-all",
                        style: "width: 40px; height: 40px; min-width: 40px; min-height: 40px; background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        title: "Cari Buku",
                        onclick: move |_| {
                            let cur = *is_search_visible.read();
                            is_search_visible.set(!cur);
                        },
                        svg {
                            class: "w-4 h-4",
                            width: "16",
                            height: "16",
                            style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "11", cy: "11", r: "8" }
                            line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                        }
                    }
                }
            }

            // Expandable Instant Search Bar
            if *is_search_visible.read() || !search_query.is_empty() {
                div {
                    class: "mb-6 animate-slide-up",
                    div {
                        class: "relative w-full",
                        input {
                            class: "w-full pl-11 pr-4 py-3 rounded-2xl border text-sm outline-none transition-all duration-200 focus:ring-2",
                            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                            placeholder: "Cari judul buku atau nama pengarang...",
                            value: "{search_query}",
                            autofocus: true,
                            oninput: move |e| on_search.call(e.value()),
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
            }

            // 2. Empty State
            if filtered_books.is_empty() {
                div {
                    class: "flex-1 flex flex-col items-center justify-center p-12 text-center rounded-[32px] border border-dashed my-8 animate-fade-in",
                    style: "border-color: {theme.border_color()}; background-color: {theme.surface_color()};",

                    div {
                        class: "w-16 h-16 rounded-full flex items-center justify-center mb-4",
                        style: "width: 64px; height: 64px; min-width: 64px; min-height: 64px; background-color: {theme.surface_subtle()}; color: {theme.text_secondary()};",
                        svg {
                            class: "w-8 h-8",
                            width: "32",
                            height: "32",
                            style: "width: 32px; height: 32px; min-width: 32px; min-height: 32px;",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.8",
                            path { d: "M4 19.5A2.5 2.5 0 0 1 6.5 17H20" }
                            path { d: "M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" }
                        }
                    }

                    h2 {
                        class: "font-serif text-2xl font-medium mb-2",
                        style: "color: {theme.text_primary()};",
                        if search_query.is_empty() { "Rak Buku Masih Kosong" } else { "Buku Tidak Ditemukan" }
                    }
                    p {
                        class: "text-[13px] max-w-sm mb-6 opacity-70 leading-relaxed font-sans",
                        style: "color: {theme.text_secondary()};",
                        if search_query.is_empty() {
                            "Mulai membaca dengan mengimpor berkas EPUB lokal atau telusuri ribuan buku domain publik gratis di Internet Archive."
                        } else {
                            "Coba cari dengan kata kunci lain atau periksa ejaan judul."
                        }
                    }

                    div {
                        class: "flex items-center gap-3",
                        if search_query.is_empty() {
                            button {
                                class: "px-6 py-3 rounded-2xl text-sm font-medium shadow-sm apple-tap transition-all",
                                style: "background-color: {theme.accent_color()}; color: {theme.accent_contrast_text()};",
                                onclick: move |_| on_import.call(()),
                                "Impor Berkas EPUB"
                            }
                            button {
                                class: "px-6 py-3 rounded-2xl text-sm font-medium border apple-tap transition-all",
                                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                                onclick: move |_| on_explore.call(()),
                                "Jelajahi Online"
                            }
                        }
                    }
                }
            } else {
                // 3. Hero Bento Card ("Continue Reading" directly matching reference)
                if let Some(book) = featured_book {
                    {
                        let book_click = book.clone();
                        let book_delete = book.clone();
                        let progress = (book.progress_percent * 100.0).round() as u32;
                        let est_minutes_left = ((1.0 - book.progress_percent) * 35.0).max(3.0).round() as u32;

                        rsx! {
                            div {
                                class: "hero-bento-card relative rounded-[28px] p-5 sm:p-6 mb-10 border transition-all duration-300 animate-slide-up shadow-sm",
                                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; box-shadow: {theme.card_shadow()};",

                                // Top Row: Book Cover & Details
                                div {
                                    class: "flex items-start gap-5 sm:gap-6",

                                    // Book Cover with Realistic Depth
                                    div {
                                        class: "book-cover-realistic w-20 sm:w-24 aspect-[2/3] rounded-xl flex-shrink-0 cursor-pointer",
                                        onclick: {
                                            let b = book_click.clone();
                                            move |_| on_select_book.call(b.clone())
                                        },
                                        if let Some(cover) = &book.cover_image {
                                            img {
                                                class: "w-full h-full object-cover",
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

                                                // 3-dots Context Menu Button (matching reference)
                                                button {
                                                    class: "w-8 h-8 rounded-full flex items-center justify-center flex-shrink-0 apple-tap transition-colors",
                                                    style: "width: 32px; height: 32px; min-width: 32px; min-height: 32px; background-color: {theme.surface_subtle()}; color: {theme.text_secondary()};",
                                                    title: "Hapus Buku",
                                                    onclick: move |_| on_delete_book.call(book_delete.id.clone()),
                                                    svg {
                                                        class: "w-4 h-4",
                                                        width: "16",
                                                        height: "16",
                                                        style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                                                        view_box: "0 0 24 24",
                                                        fill: "currentColor",
                                                        circle { cx: "5", cy: "12", r: "2" }
                                                        circle { cx: "12", cy: "12", r: "2" }
                                                        circle { cx: "19", cy: "12", r: "2" }
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
                                                    if progress == 100 { "Selesai dibaca" } else { "{est_minutes_left} mnt tersisa di bab ini" }
                                                }
                                                span {
                                                    class: "font-semibold",
                                                    style: "color: {theme.text_primary()};",
                                                    "{progress}%"
                                                }
                                            }

                                            // Thin Progress Bar
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
                                        class: "w-full h-12 rounded-2xl px-5 flex items-center justify-between text-sm font-medium shadow-sm apple-tap transition-all",
                                        style: "background-color: {theme.accent_color()}; color: {theme.accent_contrast_text()};",
                                        onclick: {
                                            let b = book_click.clone();
                                            move |_| on_select_book.call(b.clone())
                                        },
                                        span { "Lanjutkan Membaca" }
                                        div {
                                            class: "w-7 h-7 rounded-full flex items-center justify-center transition-transform group-hover:translate-x-0.5",
                                            style: "width: 28px; height: 28px; min-width: 28px; min-height: 28px; background-color: rgba(255,255,255,0.15);",
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
                }

                // 4. "Recent" Section (matching reference)
                if !recent_books.is_empty() {
                    div {
                        class: "mb-10",
                        div {
                            class: "flex items-center justify-between mb-4",
                            h2 {
                                class: "font-serif text-[20px] font-semibold tracking-tight",
                                style: "color: {theme.text_primary()};",
                                "Recent"
                            }
                            span {
                                class: "font-sans text-[12px] font-medium opacity-60 hover:opacity-100 cursor-pointer transition-opacity",
                                style: "color: {theme.text_secondary()};",
                                "Lihat semua"
                            }
                        }

                        // Horizontal Scroll / Responsive Grid for Recent Books
                        div {
                            class: "grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 gap-4 sm:gap-5",
                            for b in recent_books {
                                BookCard {
                                    key: "{b.id}",
                                    book: b.clone(),
                                    theme: theme,
                                    on_select: move |selected| on_select_book.call(selected),
                                    on_delete: move |id| on_delete_book.call(id),
                                }
                            }
                        }
                    }
                }

                // 5. "All Books" Section (if more than 7 books or searching)
                if filtered_books.len() > 7 || !search_query.is_empty() {
                    div {
                        class: "mb-8",
                        h2 {
                            class: "font-serif text-[20px] font-semibold tracking-tight mb-4",
                            style: "color: {theme.text_primary()};",
                            "Semua Koleksi ({filtered_books.len()})"
                        }
                        div {
                            class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-5",
                            for b in filtered_books {
                                BookCard {
                                    key: "{b.id}",
                                    book: b.clone(),
                                    theme: theme,
                                    on_select: move |selected| on_select_book.call(selected),
                                    on_delete: move |id| on_delete_book.call(id),
                                }
                            }
                        }
                    }
                }
            }

            // 6. Floating Apple Island Dock (Bottom Navigation Bar matching reference)
            div {
                class: "fixed bottom-6 left-1/2 -translate-x-1/2 z-40 rounded-full px-6 py-3 flex items-center gap-7 sm:gap-9 shadow-lg border backdrop-blur-2xl transition-all duration-300",
                style: "position: fixed; bottom: 24px; left: 50%; transform: translateX(-50%); z-index: 40; border-radius: 9999px; padding: 10px 24px; display: flex; align-items: center; gap: 28px; background-color: {theme.surface_color()}EE; border: 1px solid {theme.border_color()}; box-shadow: 0 16px 40px rgba(0,0,0,0.12); -webkit-backdrop-filter: blur(24px); backdrop-filter: blur(24px);",

                // Home / Shelf (Active)
                button {
                    class: "apple-tap p-1.5 rounded-full transition-transform",
                    style: "color: {theme.text_primary()};",
                    title: "Rak Buku",
                    svg {
                        class: "w-5 h-5",
                        width: "20",
                        height: "20",
                        style: "width: 20px; height: 20px; min-width: 20px; min-height: 20px;",
                        view_box: "0 0 24 24",
                        fill: "currentColor",
                        path { d: "M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z" }
                    }
                }

                // Explore / Internet Archive
                button {
                    class: "apple-tap p-1.5 rounded-full transition-transform opacity-60 hover:opacity-100",
                    style: "color: {theme.text_secondary()};",
                    title: "Jelajahi & Unduh Online",
                    onclick: move |_| on_explore.call(()),
                    svg {
                        class: "w-5 h-5",
                        width: "20",
                        height: "20",
                        style: "width: 20px; height: 20px; min-width: 20px; min-height: 20px;",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        circle { cx: "12", cy: "12", r: "10" }
                        line { x1: "2", y1: "12", x2: "22", y2: "12" }
                        path { d: "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" }
                    }
                }

                // Statistics
                button {
                    class: "apple-tap p-1.5 rounded-full transition-transform opacity-60 hover:opacity-100",
                    style: "color: {theme.text_secondary()};",
                    title: "Statistik Membaca",
                    onclick: move |_| on_stats.call(()),
                    svg {
                        class: "w-5 h-5",
                        width: "20",
                        height: "20",
                        style: "width: 20px; height: 20px; min-width: 20px; min-height: 20px;",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line { x1: "18", y1: "20", x2: "18", y2: "10" }
                        line { x1: "12", y1: "20", x2: "12", y2: "4" }
                        line { x1: "6", y1: "20", x2: "6", y2: "14" }
                    }
                }

                // Settings & Backup
                button {
                    class: "apple-tap p-1.5 rounded-full transition-transform opacity-60 hover:opacity-100",
                    style: "color: {theme.text_secondary()};",
                    title: "Cadangkan & Pengaturan",
                    onclick: move |_| on_settings.call(()),
                    svg {
                        class: "w-5 h-5",
                        width: "20",
                        height: "20",
                        style: "width: 20px; height: 20px; min-width: 20px; min-height: 20px;",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line { x1: "4", y1: "21", x2: "4", y2: "14" }
                        line { x1: "4", y1: "10", x2: "4", y2: "3" }
                        line { x1: "12", y1: "21", x2: "12", y2: "12" }
                        line { x1: "12", y1: "8", x2: "12", y2: "3" }
                        line { x1: "20", y1: "21", x2: "20", y2: "16" }
                        line { x1: "20", y1: "12", x2: "20", y2: "3" }
                        line { x1: "1", y1: "14", x2: "7", y2: "14" }
                        line { x1: "9", y1: "8", x2: "15", y2: "8" }
                        line { x1: "17", y1: "16", x2: "23", y2: "16" }
                    }
                }
            }
        }
    }
}
