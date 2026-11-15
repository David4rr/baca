use dioxus::prelude::*;
use crate::db::models::Book;
use crate::db::Database;
use crate::engine::{
    download_archive_book_with_progress, get_archive_format_file, parse_epub, ArchiveBook,
};
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
    let mut state = use_signal(|| {
        if is_already_downloaded {
            MorphState::Success
        } else {
            MorphState::Idle
        }
    });
    let mut progress = use_signal(|| 0.05f32);

    let has_epub = book.has_epub;
    let has_pdf = book.has_pdf;

    let mut start_download = {
        let b = book.clone();
        let on_imported = on_imported.clone();
        move |fmt: String| {
            state.set(MorphState::Downloading { format: fmt.clone() });
            progress.set(0.08);

            let id = b.identifier.clone();
            let b_clone = b.clone();
            let fmt_clone = fmt.clone();
            let on_imported = on_imported.clone();

            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<f32>();
            spawn(async move {
                while let Some(pct) = rx.recv().await {
                    progress.set(pct);
                }
            });

            spawn(async move {
                match get_archive_format_file(&id, &fmt_clone).await {
                    Ok(Some(filename)) => {
                        let app_dir = Database::get_app_dir();
                        let books_dir = app_dir.join("books");
                        let _ = std::fs::create_dir_all(&books_dir);
                        let clean_filename = id.replace(|c: char| !c.is_alphanumeric(), "_");
                        let ext = if fmt_clone == "pdf" { "pdf" } else { "epub" };
                        let dest_path = books_dir.join(format!("{}.{}", clean_filename, ext));

                        let tx_clone = tx.clone();
                        let res = download_archive_book_with_progress(
                            &id,
                            &filename,
                            &dest_path,
                            move |pct| {
                                let _ = tx_clone.send(pct);
                            },
                        )
                        .await;

                        match res {
                            Ok(_) => {
                                let now = chrono::Utc::now().timestamp();
                                let new_book = if fmt_clone == "pdf" {
                                    Book {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        title: b_clone.title.clone(),
                                        author: b_clone.author_display(),
                                        file_path: dest_path.to_string_lossy().to_string(),
                                        format: "pdf".to_string(),
                                        cover_image: Some(b_clone.thumbnail_url()),
                                        total_chapters: 1,
                                        current_chapter: 0,
                                        progress_percent: 0.0,
                                        last_read_at: now,
                                        created_at: now,
                                    }
                                } else if let Ok(parsed) = parse_epub(&dest_path) {
                                    Book {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        title: if !parsed.title.is_empty() { parsed.title } else { b_clone.title.clone() },
                                        author: if !parsed.author.is_empty() { parsed.author } else { b_clone.author_display() },
                                        file_path: dest_path.to_string_lossy().to_string(),
                                        format: "epub".to_string(),
                                        cover_image: parsed.cover_image.or_else(|| Some(b_clone.thumbnail_url())),
                                        total_chapters: parsed.chapters.len(),
                                        current_chapter: 0,
                                        progress_percent: 0.0,
                                        last_read_at: now,
                                        created_at: now,
                                    }
                                } else {
                                    Book {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        title: b_clone.title.clone(),
                                        author: b_clone.author_display(),
                                        file_path: dest_path.to_string_lossy().to_string(),
                                        format: "epub".to_string(),
                                        cover_image: Some(b_clone.thumbnail_url()),
                                        total_chapters: 1,
                                        current_chapter: 0,
                                        progress_percent: 0.0,
                                        last_read_at: now,
                                        created_at: now,
                                    }
                                };

                                if let Ok(db) = Database::init() {
                                    let _ = db.insert_or_update_book(&new_book);
                                }
                                state.set(MorphState::Success);
                                on_imported.call(new_book);
                            }
                            Err(e) => state.set(MorphState::Failed(e)),
                        }
                    }
                    Ok(None) => state.set(MorphState::Failed(format!("{} tidak tersedia", fmt_clone.to_uppercase()))),
                    Err(e) => state.set(MorphState::Failed(e)),
                }
            });
        }
    };

    rsx! {
        div {
            class: "inline-flex items-center select-none",

            match state.read().clone() {
                MorphState::Idle => rsx! {
                    button {
                        class: "px-3.5 py-1.5 rounded-full text-xs font-sans font-semibold apple-tap shadow-sm transition-all flex items-center gap-1.5 border",
                        style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        onclick: move |_| {
                            if has_epub && has_pdf {
                                state.set(MorphState::FormatSelect);
                            } else if has_pdf {
                                start_download("pdf".to_string());
                            } else {
                                start_download("epub".to_string());
                            }
                        },
                        svg {
                            class: "w-3.5 h-3.5",
                            width: "14",
                            height: "14",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                            polyline { points: "7 10 12 15 17 10" }
                            line { x1: "12", y1: "15", x2: "12", y2: "3" }
                        }
                        span { "Download" }
                    }
                },
                MorphState::FormatSelect => rsx! {
                    div {
                        class: "flex items-center gap-1 p-1 rounded-full border shadow-md animate-spring-pop",
                        style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",
                        button {
                            class: "px-2.5 py-1 rounded-full text-[11px] font-sans font-bold uppercase apple-tap transition-all",
                            style: "background-color: {theme.accent_color()}20; color: {theme.accent_color()};",
                            onclick: {
                                let mut dl = start_download.clone();
                                move |_| dl("epub".to_string())
                            },
                            "EPUB"
                        }
                        button {
                            class: "px-2.5 py-1 rounded-full text-[11px] font-sans font-bold uppercase apple-tap transition-all",
                            style: "background-color: #ef444420; color: #ef4444;",
                            onclick: {
                                let mut dl = start_download.clone();
                                move |_| dl("pdf".to_string())
                            },
                            "PDF"
                        }
                        button {
                            class: "px-1.5 py-1 text-xs opacity-40 hover:opacity-100 transition-opacity",
                            style: "color: {theme.text_secondary()};",
                            onclick: move |_| state.set(MorphState::Idle),
                            "✕"
                        }
                    }
                },
                MorphState::Downloading { format } => rsx! {
                    div {
                        class: "relative overflow-hidden px-4 py-1.5 rounded-full text-xs font-sans font-semibold border shadow-inner min-w-[125px] flex items-center justify-center animate-pulse-glow",
                        style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        div {
                            class: "absolute inset-y-0 left-0 transition-all duration-200",
                            style: "width: {progress * 100.0}%; background-color: {theme.accent_color()}35;",
                        }
                        span {
                            class: "relative z-10 flex items-center gap-1.5",
                            div {
                                class: "w-2.5 h-2.5 border-2 border-t-transparent rounded-full animate-spin",
                                style: "border-color: {theme.accent_color()}; border-top-color: transparent;",
                            }
                            "{format.to_uppercase()} {((progress * 100.0) as u32)}%"
                        }
                    }
                },
                MorphState::Success => rsx! {
                    div {
                        class: "px-3.5 py-1.5 rounded-full text-xs font-sans font-semibold shadow-sm flex items-center gap-1.5 border animate-spring-pop text-emerald-700 dark:text-emerald-300 bg-emerald-50 dark:bg-emerald-950/40 border-emerald-500/30",
                        svg {
                            class: "w-3.5 h-3.5",
                            width: "14",
                            height: "14",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "20 6 9 17 4 12" }
                        }
                        span { "Tersimpan" }
                    }
                },
                MorphState::Failed(err) => rsx! {
                    button {
                        class: "px-3.5 py-1.5 rounded-full text-xs font-sans font-semibold apple-tap shadow-sm flex items-center gap-1.5 border animate-shake text-rose-700 dark:text-rose-300 bg-rose-50 dark:bg-rose-950/40 border-rose-500/30",
                        title: "{err}",
                        onclick: move |_| state.set(MorphState::Idle),
                        span { "✕ Gagal • Coba Lagi" }
                    }
                },
            }
        }
    }
}
