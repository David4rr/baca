use dioxus::prelude::*;
use crate::db::models::Book;
use crate::theme::ThemeMode;

#[component]
pub fn BookCard(
    book: Book,
    theme: ThemeMode,
    on_select: EventHandler<Book>,
    on_delete: EventHandler<String>,
) -> Element {
    let book_clone = book.clone();
    let book_id = book.id.clone();
    let mut show_menu = use_signal(|| false);
    let progress = (book.progress_percent * 100.0).round() as u32;
    let is_pdf = book.format.to_lowercase() == "pdf";

    rsx! {
        div {
            class: "book-card group relative flex flex-col cursor-pointer apple-tap select-none transition-all duration-300 hover:-translate-y-1",
            onclick: move |_| on_select.call(book_clone.clone()),

            // 1. Cover Container with Apple-standard realistic 3D depth & spine
            div {
                class: "book-cover-realistic relative w-full aspect-[2/3] rounded-2xl overflow-hidden shadow-md group-hover:shadow-xl transition-all duration-300 border",
                style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",

                if let Some(cover) = &book.cover_image {
                    img {
                        class: "w-full h-full object-cover select-none pointer-events-none transition-transform duration-500 group-hover:scale-105",
                        src: "{cover}",
                        alt: "{book.title}",
                        loading: "lazy",
                    }
                } else {
                    div {
                        class: "w-full h-full flex flex-col justify-between p-4 select-none relative overflow-hidden",
                        style: "background: linear-gradient(135deg, {theme.surface_color()}, {theme.surface_subtle()});",

                        div { class: "flex items-center justify-between",
                            span {
                                class: "px-2 py-0.5 rounded-full text-[9px] font-sans font-bold uppercase tracking-wider",
                                style: if is_pdf { "background-color: #ef444420; color: #ef4444;" } else { "background-color: {theme.accent_color()}20; color: {theme.accent_color()};" },
                                if is_pdf { "PDF" } else { "EPUB" }
                            }
                        }

                        div {
                            class: "font-serif text-[15px] font-medium leading-snug line-clamp-4 my-auto",
                            style: "color: {theme.text_primary()};",
                            "{book.title}"
                        }

                        div { class: "pt-2 border-t border-black/5 dark:border-white/5",
                            div {
                                class: "font-sans text-[11px] font-medium opacity-70 truncate",
                                style: "color: {theme.text_secondary()};",
                                "{book.author}"
                            }
                        }
                    }
                }

                // Realistic 3D Spine Ridge Overlay
                div { class: "absolute inset-y-0 left-0 w-3 bg-gradient-to-r from-black/25 via-white/10 to-transparent pointer-events-none" }

                // Format Pill on top-left of image
                if book.cover_image.is_some() {
                    div { class: "absolute top-2.5 left-2.5 z-10",
                        span {
                            class: "px-2 py-0.5 rounded-md text-[9px] font-sans font-bold uppercase tracking-wider backdrop-blur-md shadow-xs text-white",
                            style: if is_pdf { "background-color: rgba(239, 68, 68, 0.85);" } else { "background-color: rgba(0, 0, 0, 0.55);" },
                            if is_pdf { "PDF" } else { "EPUB" }
                        }
                    }
                }

                // 3-Dots Options Menu Button
                button {
                    class: "absolute top-2.5 right-2.5 z-10 rounded-full bg-black/60 hover:bg-black/85 text-white flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all duration-200 apple-tap shadow-md backdrop-blur-sm",
                    style: "width: 28px; height: 28px; min-width: 28px; min-height: 28px;",
                    title: "Opsi Buku",
                    onclick: move |e: MouseEvent| {
                        e.stop_propagation();
                        show_menu.toggle();
                    },
                    svg { class: "w-3.5 h-3.5", view_box: "0 0 24 24", fill: "currentColor",
                        circle { cx: "5", cy: "12", r: "2" }
                        circle { cx: "12", cy: "12", r: "2" }
                        circle { cx: "19", cy: "12", r: "2" }
                    }
                }

                if *show_menu.read() {
                    div {
                        class: "absolute top-10 right-2 z-30 p-1.5 rounded-2xl shadow-xl border flex flex-col gap-1 min-w-[150px] backdrop-blur-xl animate-fade-in",
                        style: "background-color: {theme.surface_color()}F5; border-color: {theme.border_color()};",
                        button {
                            class: "flex items-center gap-2 px-3 py-1.5 text-xs font-sans font-medium rounded-xl apple-tap hover:bg-black/5 dark:hover:bg-white/5 text-left",
                            style: "color: {theme.text_primary()};",
                            onclick: {
                                let b = book_clone.clone();
                                move |e: MouseEvent| {
                                    e.stop_propagation();
                                    show_menu.set(false);
                                    on_select.call(b.clone());
                                }
                            },
                            "📖 Baca Buku"
                        }
                        button {
                            class: "flex items-center gap-2 px-3 py-1.5 text-xs font-sans font-medium rounded-xl text-rose-500 apple-tap hover:bg-rose-500/10 text-left",
                            onclick: {
                                let id = book_id.clone();
                                move |e: MouseEvent| {
                                    e.stop_propagation();
                                    show_menu.set(false);
                                    on_delete.call(id.clone());
                                }
                            },
                            "🗑️ Hapus"
                        }
                    }
                }

                // Progress Bar Overlay on Bottom of Cover
                if progress > 0 {
                    div { class: "absolute bottom-0 inset-x-0 h-1 bg-black/30 backdrop-blur-xs",
                        div {
                            class: "h-full transition-all duration-500",
                            style: if progress >= 100 { "width: 100%; background-color: #10B981;" } else { "width: {progress}%; background-color: {theme.accent_color()};" },
                        }
                    }
                }
            }

            // 2. Refined Editorial Typography
            div { class: "mt-3 flex flex-col",
                h3 {
                    class: "font-serif text-[14px] font-semibold line-clamp-1 leading-snug transition-colors group-hover:opacity-80",
                    style: "color: {theme.text_primary()};",
                    title: "{book.title}",
                    "{book.title}"
                }
                div { class: "flex items-center justify-between mt-0.5",
                    p {
                        class: "font-sans text-[12px] truncate opacity-65 flex-1 mr-2",
                        style: "color: {theme.text_secondary()};",
                        "{book.author}"
                    }
                    span {
                        class: "font-sans text-[11px] font-semibold whitespace-nowrap",
                        style: if progress >= 100 { "color: #10B981;" } else { "color: {theme.text_secondary()}; opacity: 0.8;" },
                        if progress >= 100 { "Selesai" } else if progress > 0 { "{progress}%" } else { "Baru" }
                    }
                }
            }
        }
    }
}
