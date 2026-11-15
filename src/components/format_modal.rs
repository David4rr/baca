use dioxus::prelude::*;
use crate::engine::ArchiveBook;
use crate::theme::ThemeMode;

#[component]
pub fn FormatModal(
    book: Option<ArchiveBook>,
    theme: ThemeMode,
    on_select_format: EventHandler<(ArchiveBook, String)>,
    on_close: EventHandler<()>,
) -> Element {
    let Some(b) = book else {
        return rsx! {};
    };

    let title = b.title.clone();
    let author = b.author_display();
    let has_epub = b.has_epub;
    let has_pdf = b.has_pdf;

    let b_epub = b.clone();
    let b_pdf = b.clone();

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-end justify-center bg-black/40 backdrop-blur-sm animate-fade-in",
            onclick: move |_| on_close.call(()),

            div {
                class: "w-full max-w-lg rounded-t-[32px] p-6 pb-10 border-t shadow-2xl animate-sheet-slide flex flex-col gap-5",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",
                onclick: move |e| e.stop_propagation(),

                // Drag handle
                div {
                    class: "w-10 h-1.5 rounded-full mx-auto opacity-30 -mt-1 mb-1",
                    style: "background-color: {theme.text_secondary()};",
                }

                // Book Details Header
                div {
                    class: "flex items-center gap-4",
                    div {
                        class: "w-14 h-20 rounded-lg overflow-hidden flex-shrink-0 shadow-md border",
                        style: "border-color: {theme.border_color()}; background-color: {theme.bg_color()};",
                        img {
                            class: "w-full h-full object-cover",
                            src: "{b.thumbnail_url()}",
                            alt: "{title}",
                        }
                    }
                    div {
                        class: "flex flex-col flex-1 min-w-0",
                        h3 {
                            class: "font-serif text-lg font-medium leading-tight truncate",
                            style: "color: {theme.text_primary()};",
                            "{title}"
                        }
                        p {
                            class: "font-sans text-xs mt-1 truncate opacity-70",
                            style: "color: {theme.text_secondary()};",
                            "{author}"
                        }
                        span {
                            class: "inline-block text-[11px] font-sans font-medium mt-1.5 opacity-50",
                            style: "color: {theme.text_secondary()};",
                            "Pilih format yang ingin diunduh ke perangkat Anda:"
                        }
                    }
                }

                // Format Options
                div {
                    class: "flex flex-col gap-2.5",

                    // EPUB Option
                    button {
                        class: "flex items-center justify-between p-4 rounded-2xl border apple-tap transition-all text-left",
                        style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()};",
                        disabled: !has_epub,
                        onclick: move |_| {
                            on_select_format.call((b_epub.clone(), "epub".to_string()));
                            on_close.call(());
                        },
                        div {
                            class: "flex flex-col",
                            span {
                                class: "font-sans text-sm font-semibold tracking-tight",
                                style: "color: {theme.text_primary()};",
                                "Unduh EPUB"
                            }
                            span {
                                class: "font-sans text-xs opacity-60 mt-0.5",
                                style: "color: {theme.text_secondary()};",
                                "Format reflowable standar • Tipografi editorial"
                            }
                        }
                        span {
                            class: "px-2.5 py-1 rounded-full text-[11px] font-sans font-bold tracking-wider uppercase",
                            style: "background-color: {theme.accent_color()}1F; color: {theme.accent_color()};",
                            "EPUB"
                        }
                    }

                    // PDF Option
                    button {
                        class: "flex items-center justify-between p-4 rounded-2xl border apple-tap transition-all text-left",
                        style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()};",
                        disabled: !has_pdf,
                        onclick: move |_| {
                            on_select_format.call((b_pdf.clone(), "pdf".to_string()));
                            on_close.call(());
                        },
                        div {
                            class: "flex flex-col",
                            span {
                                class: "font-sans text-sm font-semibold tracking-tight",
                                style: "color: {theme.text_primary()};",
                                "Unduh PDF"
                            }
                            span {
                                class: "font-sans text-xs opacity-60 mt-0.5",
                                style: "color: {theme.text_secondary()};",
                                "Tata letak halaman asli • Dokumen grafis"
                            }
                        }
                        span {
                            class: "px-2.5 py-1 rounded-full text-[11px] font-sans font-bold tracking-wider uppercase",
                            style: "background-color: #ef44441F; color: #ef4444;",
                            "PDF"
                        }
                    }
                }

                // Cancel Button
                button {
                    class: "w-full py-3.5 rounded-2xl font-sans text-sm font-medium apple-tap transition-all border",
                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_secondary()};",
                    onclick: move |_| on_close.call(()),
                    "Batal"
                }
            }
        }
    }
}
