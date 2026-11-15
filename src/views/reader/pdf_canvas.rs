use dioxus::prelude::*;
use crate::db::models::Book;
use crate::theme::ThemeMode;

#[component]
pub fn PdfCanvas(
    book: Book,
    theme: ThemeMode,
    on_back: EventHandler<()>,
) -> Element {
    let file_url = if book.file_path.starts_with("http") || book.file_path.starts_with("file://") {
        book.file_path.clone()
    } else {
        format!("file://{}", book.file_path)
    };

    rsx! {
        div {
            class: "w-full max-w-4xl mx-auto px-4 sm:px-6 pt-4 pb-20 flex flex-col flex-1 h-screen animate-fade-in",

            // Top Header
            div {
                class: "flex items-center justify-between pb-4 select-none",
                button {
                    class: "w-10 h-10 rounded-full flex items-center justify-center border shadow-sm apple-tap transition-all",
                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    onclick: move |_| on_back.call(()),
                    svg {
                        class: "w-5 h-5",
                        width: "20",
                        height: "20",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                }

                div {
                    class: "flex flex-col items-center max-w-[60%] text-center",
                    h2 {
                        class: "font-serif text-base font-semibold truncate",
                        style: "color: {theme.text_primary()};",
                        "{book.title}"
                    }
                    span {
                        class: "text-xs font-sans opacity-60 truncate",
                        style: "color: {theme.text_secondary()};",
                        "{book.author}"
                    }
                }

                span {
                    class: "px-2.5 py-1 rounded-full text-[10px] font-sans font-bold uppercase tracking-wider",
                    style: "background-color: #ef444420; color: #ef4444;",
                    "PDF"
                }
            }

            // PDF Viewer Area (Native iframe embedding in WebView)
            div {
                class: "flex-1 w-full rounded-2xl overflow-hidden border shadow-lg relative min-h-[70vh]",
                style: "border-color: {theme.border_color()}; background-color: {theme.surface_color()};",
                iframe {
                    class: "w-full h-full border-none",
                    src: "{file_url}",
                    title: "{book.title}",
                }
            }

            // Bottom status bar
            div {
                class: "flex items-center justify-between pt-3 px-2 text-xs font-sans opacity-50",
                style: "color: {theme.text_secondary()};",
                span { "Dokumen PDF Asli" }
                span { "Gunakan cubit/geser layar untuk navigasi halaman" }
            }
        }
    }
}
