use dioxus::prelude::*;
use crate::db::models::Book;
use crate::db::Database;
use crate::engine::archive::{download_format_file, ArchiveBook};
use crate::theme::ThemeMode;

#[derive(Debug, Clone, PartialEq)]
pub enum MorphState {
    Idle,
    FormatSelect,
    Downloading { format: String },
    Success,
    Failed(String),
}

#[component]
pub fn MorphingDownloadButton(
    book: ArchiveBook,
    theme: ThemeMode,
    is_already_downloaded: bool,
    on_imported: EventHandler<Book>,
) -> Element {
    let mut state = use_signal(|| if is_already_downloaded { MorphState::Success } else { MorphState::Idle });
    let mut progress = use_signal(|| 0.05f32);

    let has_epub = book.has_epub;
    let has_pdf = book.has_pdf;

    let book_arc = std::sync::Arc::new(book);

    let run_download = {
        let book_arc = book_arc.clone();
        move |fmt: &'static str| {
            let fmt_str = fmt.to_string();
            state.set(MorphState::Downloading { format: fmt_str.clone() });
            progress.set(0.08);

            let b = (*book_arc).clone();
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<f32>();

            spawn(async move {
                while let Some(pct) = rx.recv().await { progress.set(pct); }
            });

            spawn(async move {
                let res = download_format_file(&b, &fmt_str, move |pct| { let _ = tx.send(pct); }).await;
                match res {
                    Ok(new_book) => {
                        let _ = Database::init().map(|db| db.insert_or_update_book(&new_book));
                        state.set(MorphState::Success);
                        on_imported.call(new_book);
                    }
                    Err(err) => state.set(MorphState::Failed(err)),
                }
            });
        }
    };

    let mut dl_idle = run_download.clone();
    let mut dl_epub = run_download.clone();
    let mut dl_pdf = run_download;

    rsx! {
        div { class: "inline-flex items-center transition-all duration-300",
            match state.read().clone() {
                MorphState::Idle => rsx! {
                    button {
                        class: "h-9 px-4 rounded-full font-sans text-xs font-semibold flex items-center gap-1.5 shadow-sm apple-tap transition-all hover:opacity-90",
                        style: "background-color: {theme.surface_color()}; border: 1px solid {theme.border_color()}; color: {theme.text_primary()};",
                        onclick: move |e| {
                            e.stop_propagation();
                            if has_epub && has_pdf { state.set(MorphState::FormatSelect); }
                            else if has_pdf { dl_idle("pdf"); }
                            else { dl_idle("epub"); }
                        },
                        svg { class: "w-3.5 h-3.5 opacity-70", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.2", stroke_linecap: "round", stroke_linejoin: "round",
                            path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                            polyline { points: "7 10 12 15 17 10" }
                            line { x1: "12", y1: "15", x2: "12", y2: "3" }
                        }
                        span { "Download" }
                    }
                },
                MorphState::FormatSelect => rsx! {
                    div {
                        class: "h-9 px-1.5 rounded-full flex items-center gap-1 shadow-md border animate-spring-pop",
                        style: "background-color: {theme.surface_color()}; border-color: {theme.accent_color()}40;",
                        if has_epub {
                            button {
                                class: "h-7 px-2.5 rounded-full text-[11px] font-sans font-semibold text-white shadow-xs apple-tap",
                                style: "background-color: {theme.accent_color()};",
                                onclick: move |e| { e.stop_propagation(); dl_epub("epub"); },
                                "EPUB"
                            }
                        }
                        if has_pdf {
                            button {
                                class: "h-7 px-2.5 rounded-full text-[11px] font-sans font-semibold border apple-tap",
                                style: "border-color: {theme.border_color()}; color: {theme.text_primary()};",
                                onclick: move |e| { e.stop_propagation(); dl_pdf("pdf"); },
                                "PDF"
                            }
                        }
                        button {
                            class: "w-6 h-6 rounded-full flex items-center justify-center text-xs opacity-50 hover:opacity-100",
                            onclick: move |e| { e.stop_propagation(); state.set(MorphState::Idle); },
                            "✕"
                        }
                    }
                },
                MorphState::Downloading { format } => rsx! {
                    div {
                        class: "h-9 px-3.5 rounded-full flex items-center gap-2.5 border shadow-sm relative overflow-hidden",
                        style: "background-color: {theme.surface_color()}; border-color: {theme.accent_color()}50;",
                        div {
                            class: "absolute left-0 top-0 bottom-0 opacity-20 transition-all duration-300",
                            style: "width: {*progress.read() * 100.0}%; background-color: {theme.accent_color()};",
                        }
                        div { class: "w-3 h-3 rounded-full border-2 border-t-transparent animate-spin z-10", style: "border-color: {theme.accent_color()}; border-top-color: transparent;" }
                        span { class: "text-xs font-sans font-medium z-10", style: "color: {theme.text_primary()};",
                            "{format.to_uppercase()} {(*progress.read() * 100.0) as u32}%"
                        }
                    }
                },
                MorphState::Success => rsx! {
                    div {
                        class: "h-9 px-3.5 rounded-full flex items-center gap-1.5 border shadow-xs animate-spring-pop",
                        style: "background-color: #10B98115; border-color: #10B98140; color: #10B981;",
                        svg { class: "w-3.5 h-3.5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round", polyline { points: "20 6 9 17 4 12" } }
                        span { class: "text-xs font-sans font-semibold", "✓ Tersimpan" }
                    }
                },
                MorphState::Failed(err) => rsx! {
                    button {
                        class: "h-9 px-3 rounded-full flex items-center gap-1.5 border shadow-xs animate-shake",
                        style: "background-color: #EF444415; border-color: #EF444440; color: #EF4444;",
                        title: "{err}",
                        onclick: move |e| { e.stop_propagation(); state.set(MorphState::Idle); },
                        span { class: "text-xs font-sans font-medium", "✕ Gagal • Coba Lagi" }
                    }
                },
            }
        }
    }
}
