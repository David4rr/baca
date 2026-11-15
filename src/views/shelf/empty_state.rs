use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn EmptyShelfState(
    search_query: String,
    theme: ThemeMode,
    on_import: EventHandler<()>,
    on_explore: EventHandler<()>,
) -> Element {
    let is_empty = search_query.is_empty();

    rsx! {
        div {
            class: "flex-1 flex flex-col items-center justify-center p-12 text-center rounded-[32px] border border-dashed my-8 animate-fade-in select-none",
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
                if is_empty { "Rak Buku Masih Kosong" } else { "Buku Tidak Ditemukan" }
            }
            p {
                class: "text-[13px] max-w-sm mb-6 opacity-70 leading-relaxed font-sans",
                style: "color: {theme.text_secondary()};",
                if is_empty {
                    "Mulai membaca dengan mengimpor berkas EPUB lokal atau telusuri ribuan karya klasik di Arsip Online."
                } else {
                    "Coba kata kunci lain atau periksa ejaan judul buku."
                }
            }

            if is_empty {
                div {
                    class: "flex items-center gap-3",
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
    }
}
