use dioxus::prelude::*;
use crate::db::models::Bookmark;
use crate::engine::TocItem;
use crate::theme::ThemeMode;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TocTab {
    Chapters,
    Bookmarks,
}

#[component]
pub fn TocDrawer(
    toc: Vec<TocItem>,
    bookmarks: Vec<Bookmark>,
    current_index: usize,
    theme: ThemeMode,
    is_open: bool,
    on_close: EventHandler<()>,
    on_select: EventHandler<usize>,
    on_delete_bookmark: EventHandler<String>,
) -> Element {
    let mut active_tab = use_signal(|| TocTab::Chapters);

    use_effect(use_reactive(&is_open, move |open| {
        if open {
            active_tab.set(TocTab::Chapters);
        }
    }));

    if !is_open {
        return rsx! {};
    }

    rsx! {
        // Frosted Backdrop overlay
        div {
            class: "fixed inset-0 z-40 bg-black/40 backdrop-blur-sm animate-fade-in",
            onclick: move |_| on_close.call(()),
        }

        // Apple Side Drawer with Spring Motion
        div {
            class: "fixed inset-y-0 left-0 z-50 w-[21rem] max-w-[85vw] shadow-lg flex flex-col border-r animate-drawer transition-all",
            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()}; box-shadow: {theme.card_shadow()}; padding-top: env(safe-area-inset-top, 0px); padding-bottom: env(safe-area-inset-bottom, 0px);",
            role: "dialog",
            aria_modal: "true",
            aria_label: "Daftar isi buku",
            tabindex: "0",
            onkeydown: move |e: KeyboardEvent| {
                if e.key() == Key::Escape { on_close.call(()); }
            },

            // Drawer Header
            div {
                class: "flex items-center justify-between p-5 border-b",
                style: "border-color: {theme.border_color()};",
                div {
                    h2 { class: "font-serif text-lg font-medium", "Navigasi Buku" }
                    p { class: "font-sans text-[11px] opacity-60", "Daftar isi dan markah halaman tersimpan" }
                }
                button {
                    class: "w-10 h-10 rounded-full flex items-center justify-center apple-tap opacity-60 hover:opacity-100 transition-all",
                    style: "background-color: {theme.surface_subtle()}; color: {theme.text_primary()};",
                    title: "Tutup daftar isi",
                    aria_label: "Tutup daftar isi",
                    onclick: move |_| on_close.call(()),
                    svg { class: "w-4 h-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                        line { x1: "18", y1: "6", x2: "6", y2: "18" }
                        line { x1: "6", y1: "6", x2: "18", y2: "18" }
                    }
                }
            }

            // Segmented Tab Switcher (Chapters vs Bookmarks)
            div {
                class: "flex p-1 mx-4 mt-3 rounded-xl border select-none",
                style: "background-color: {theme.surface_subtle()}; border-color: {theme.border_color()};",
                button {
                    class: "flex-1 py-1.5 text-xs font-sans font-medium rounded-lg transition-all",
                    style: if *active_tab.read() == TocTab::Chapters {
                        format!("background-color: {}; color: {}; font-weight: 600;", theme.surface_color(), theme.text_primary())
                    } else {
                        format!("color: {}; opacity: 0.6;", theme.text_secondary())
                    },
                    onclick: move |_| active_tab.set(TocTab::Chapters),
                    "Bab ({toc.len()})"
                }
                button {
                    class: "flex-1 py-1.5 text-xs font-sans font-medium rounded-lg transition-all",
                    style: if *active_tab.read() == TocTab::Bookmarks {
                        format!("background-color: {}; color: {}; font-weight: 600;", theme.surface_color(), theme.text_primary())
                    } else {
                        format!("color: {}; opacity: 0.6;", theme.text_secondary())
                    },
                    onclick: move |_| active_tab.set(TocTab::Bookmarks),
                    "Markah ({bookmarks.len()})"
                }
            }

            // List Content
            div {
                class: "flex-1 overflow-y-auto p-4 space-y-1.5",

                if *active_tab.read() == TocTab::Chapters {
                    for item in toc {
                        {
                            let idx = item.chapter_index;
                            let is_active = idx == current_index;
                            rsx! {
                                button {
                                    key: "{idx}",
                                    class: "w-full text-left px-4 py-3 rounded-2xl text-[13px] font-sans transition-all flex items-center justify-between apple-tap shadow-sm",
                                    style: if is_active {
                                        format!("background-color: {}; color: {}; font-weight: 600;", theme.accent_color(), theme.accent_contrast_text())
                                    } else {
                                        format!("background-color: transparent; color: {};", theme.text_primary())
                                    },
                                    onclick: move |_| {
                                        on_select.call(idx);
                                        on_close.call(());
                                    },
                                    span { class: "truncate mr-2 font-serif text-[14px]", "{item.title}" }
                                    if is_active {
                                        span { class: "text-[10px] uppercase font-sans tracking-wider px-2 py-0.5 rounded-full",
                                            style: "background-color: rgba(255,255,255,0.2);", "Aktif"
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if bookmarks.is_empty() {
                        div { class: "py-16 text-center opacity-40 text-xs font-sans space-y-2",
                            p { "Belum ada markah tersimpan." }
                            p { class: "text-[11px]", "Ketuk ikon markah di bilah atas untuk menandai bab." }
                        }
                    } else {
                        for bm in bookmarks {
                            {
                                let idx = bm.chapter_index;
                                let bm_id = bm.id.clone();
                                rsx! {
                                    div {
                                        key: "{bm.id}",
                                        class: "p-3 rounded-2xl border apple-tap transition-all flex items-center justify-between gap-2 shadow-sm cursor-pointer",
                                        style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",
                                        onclick: move |_| {
                                            on_select.call(idx);
                                            on_close.call(());
                                        },
                                        div { class: "min-w-0 flex-1",
                                            span { class: "font-serif text-[13px] font-semibold truncate block", style: "color: {theme.text_primary()};",
                                                "{bm.chapter_title}"
                                            }
                                            if !bm.text_snippet.is_empty() {
                                                span { class: "font-sans text-[11px] opacity-60 line-clamp-1 block mt-0.5", style: "color: {theme.text_secondary()};",
                                                    "{bm.text_snippet}"
                                                }
                                            }
                                        }
                                        button {
                                            class: "w-7 h-7 rounded-full flex items-center justify-center opacity-40 hover:opacity-100 text-rose-500 hover:bg-rose-500/10 apple-tap transition-all flex-shrink-0",
                                            title: "Hapus Markah",
                                            onclick: {
                                                let id_del = bm_id.clone();
                                                move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    on_delete_bookmark.call(id_del.clone());
                                                }
                                            },
                                            svg { class: "w-3.5 h-3.5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                                line { x1: "18", y1: "6", x2: "6", y2: "18" }
                                                line { x1: "6", y1: "6", x2: "18", y2: "18" }
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
