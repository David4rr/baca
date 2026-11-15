use dioxus::prelude::*;
use crate::engine::{search_archive, ArchiveBook};
use crate::theme::ThemeMode;

#[component]
pub fn OnlineFeed(
    theme: ThemeMode,
    on_download_book: EventHandler<ArchiveBook>,
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
            class: "mt-12 flex flex-col gap-5",

            // Section Header
            div {
                class: "flex flex-col sm:flex-row sm:items-end justify-between gap-2 select-none",
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
                    class: "py-12 flex flex-col items-center justify-center gap-3",
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
                    class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4",
                    for book in online_books.read().iter().take(6) {
                        {
                            let b_clone = book.clone();
                            let on_dl = on_download_book.clone();
                            rsx! {
                                div {
                                    class: "p-4 rounded-2xl border flex gap-3.5 items-center shadow-sm hover:shadow-md transition-all",
                                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",

                                    // Cover Thumbnail
                                    div {
                                        class: "w-14 h-20 rounded-lg overflow-hidden flex-shrink-0 shadow-sm border",
                                        style: "border-color: {theme.border_color()}; background-color: {theme.bg_color()};",
                                        img {
                                            class: "w-full h-full object-cover",
                                            src: "{book.thumbnail_url()}",
                                            alt: "{book.title}",
                                            loading: "lazy",
                                        }
                                    }

                                    // Details & Download Button
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
                                                span {
                                                    class: "px-1.5 py-0.5 rounded text-[9px] font-sans font-bold uppercase",
                                                    style: "background-color: {theme.accent_color()}15; color: {theme.accent_color()};",
                                                    "EPUB"
                                                }
                                                span {
                                                    class: "px-1.5 py-0.5 rounded text-[9px] font-sans font-bold uppercase",
                                                    style: "background-color: #ef444415; color: #ef4444;",
                                                    "PDF"
                                                }
                                            }

                                            button {
                                                class: "px-2.5 py-1 rounded-full text-xs font-sans font-medium apple-tap transition-all flex items-center gap-1 border",
                                                style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                                                onclick: move |_| on_dl.call(b_clone.clone()),
                                                "Unduh"
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
