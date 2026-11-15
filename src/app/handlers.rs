use dioxus::prelude::*;
use crate::db::models::{BackupData, Book, Bookmark, UserSettings};
use crate::db::Database;
use crate::engine::archive::ArchiveBook;
use crate::engine::parse_epub;
use crate::state::{AppState, View};
use crate::theme::ThemeMode;
use std::sync::Arc;
use super::download_handler::download_format_to_library;

pub fn open_book_reader(db: Arc<Database>, mut state: AppState, book: Book) {
    let path = book.file_path.clone();
    let book_clone = book.clone();
    let book_id = book.id.clone();
    let now = chrono::Utc::now().timestamp();

    // Preload bookmarks for this book
    let bms = db.get_bookmarks(&book_id).unwrap_or_default();
    *state.bookmarks.write() = bms;

    // Update last_read_at in background
    let db_touch = db.clone();
    let id_touch = book_id.clone();
    let ch_touch = book.current_chapter;
    let pct_touch = book.progress_percent;
    dioxus::prelude::spawn(async move {
        let _ = db_touch.update_progress(&id_touch, ch_touch, pct_touch);
    });

    dioxus::prelude::spawn(async move {
        if book_clone.format.to_lowercase() == "pdf" {
            *state.current_chapter.write() = book_clone.current_chapter;
            *state.current_view.write() = View::Reader { book: book_clone, epub: None };
            return;
        }
        let res = crate::engine::get_starter_book_epub(&path)
            .or_else(|| crate::engine::get_starter_book_epub(&book_clone.title))
            .map(Ok)
            .unwrap_or_else(|| parse_epub(&path));

        match res {
            Ok(parsed) => {
                *state.current_chapter.write() = book_clone.current_chapter;
                let mut updated_book = book_clone;
                updated_book.last_read_at = now;
                *state.current_view.write() = View::Reader { book: updated_book, epub: Some(parsed) };
            }
            Err(e) => *state.toast_message.write() = Some(format!("Gagal membuka buku: {}", e)),
        }
    });
}

pub fn delete_book_from_shelf(db: Arc<Database>, mut state: AppState, book_id: String) {
    let path_opt = state.books.read().iter().find(|b| b.id == book_id).map(|b| b.file_path.clone());
    dioxus::prelude::spawn(async move {
        if db.delete_book(&book_id).is_ok() {
            if let Some(path) = path_opt {
                let p = std::path::Path::new(&path);
                if p.exists() && path.contains("books") {
                    let _ = std::fs::remove_file(p);
                }
            }
            state.books.write().retain(|b| b.id != book_id);
            *state.toast_message.write() = Some("Buku dihapus dari rak".to_string());
        }
    });
}

pub fn update_reading_progress(db: Arc<Database>, mut state: AppState, new_chapter: usize) {
    *state.current_chapter.write() = new_chapter;
    if let View::Reader { book, epub } = state.current_view.read().clone() {
        let total = epub.as_ref().map(|e| e.chapters.len()).unwrap_or(1);
        let pct = if total > 0 { (new_chapter as f32) / (total as f32) } else { 0.0 };
        let id = book.id.clone();
        dioxus::prelude::spawn(async move {
            let _ = db.update_progress(&id, new_chapter, pct);
        });
    }
}

pub fn record_reading_time(db: Arc<Database>, book_id: String, duration_seconds: u32) {
    if duration_seconds < 5 { return; }
    dioxus::prelude::spawn(async move {
        let _ = db.record_reading_session(&book_id, duration_seconds);
    });
}

pub fn toggle_bookmark(
    db: Arc<Database>,
    mut state: AppState,
    book_id: String,
    chapter_index: usize,
    chapter_title: String,
    snippet: String,
) {
    let existing = state.bookmarks.read().iter()
        .find(|b| b.book_id == book_id && b.chapter_index == chapter_index)
        .cloned();

    if let Some(bm) = existing {
        let id = bm.id.clone();
        dioxus::prelude::spawn(async move {
            if db.delete_bookmark(&id).is_ok() {
                state.bookmarks.write().retain(|b| b.id != id);
                *state.toast_message.write() = Some("Markah buku dihapus".to_string());
            }
        });
    } else {
        let mark = Bookmark {
            id: uuid::Uuid::new_v4().to_string(),
            book_id,
            chapter_index,
            chapter_title,
            text_snippet: snippet,
            note: None,
            color_tag: "amber".to_string(),
            created_at: chrono::Utc::now().timestamp(),
        };
        let mark_clone = mark.clone();
        dioxus::prelude::spawn(async move {
            if db.add_bookmark(&mark_clone).is_ok() {
                state.bookmarks.write().push(mark_clone);
                *state.toast_message.write() = Some("Markah buku disimpan".to_string());
            }
        });
    }
}

pub fn delete_bookmark_by_id(db: Arc<Database>, mut state: AppState, bookmark_id: String) {
    dioxus::prelude::spawn(async move {
        if db.delete_bookmark(&bookmark_id).is_ok() {
            state.bookmarks.write().retain(|b| b.id != bookmark_id);
            *state.toast_message.write() = Some("Markah dihapus".to_string());
        }
    });
}

pub fn save_user_settings(db: Arc<Database>, mut state: AppState, new_settings: UserSettings) {
    *state.theme.write() = ThemeMode::from_str(&new_settings.theme);
    *state.settings.write() = new_settings.clone();
    dioxus::prelude::spawn(async move {
        let _ = db.save_settings(&new_settings);
    });
}

pub fn download_online_book(db: Arc<Database>, mut state: AppState, archive_book: ArchiveBook, format_str: String) {
    *state.toast_message.write() = Some(format!("Mengunduh {} ({})...", archive_book.title, format_str.to_uppercase()));
    dioxus::prelude::spawn(async move {
        match download_format_to_library(&db, &archive_book, &format_str).await {
            Ok(book) => {
                state.books.write().insert(0, book);
                *state.toast_message.write() = Some("Buku berhasil diunduh dan tersimpan".to_string());
            }
            Err(e) => *state.toast_message.write() = Some(e),
        }
    });
}

pub fn export_library_backup(db: Arc<Database>, mut state: AppState) {
    if let Ok(data) = db.export_backup() {
        if let Ok(json) = serde_json::to_string_pretty(&data) {
            let mut saved_dest = String::new();
            let app_dest = Database::get_app_dir().join("baca_backup.json");
            if std::fs::write(&app_dest, &json).is_ok() {
                saved_dest = app_dest.display().to_string();
            }
            for dl_root in ["/sdcard/Download", "/storage/emulated/0/Download"] {
                let dl_dest = std::path::Path::new(dl_root).join("baca_backup.json");
                if std::fs::write(&dl_dest, &json).is_ok() {
                    saved_dest = dl_dest.display().to_string();
                    break;
                }
            }
            let msg = if !saved_dest.is_empty() {
                format!("Cadangan disimpan ke {}", saved_dest)
            } else {
                "Cadangan berhasil diekspor".to_string()
            };
            *state.toast_message.write() = Some(msg);
        }
    }
}

pub fn restore_library_backup(db: Arc<Database>, mut state: AppState, json_text: String) {
    if let Ok(data) = serde_json::from_str::<BackupData>(&json_text) {
        let _ = db.restore_backup(data);
        if let Ok(books) = db.get_all_books() { *state.books.write() = books; }
        if let Ok(s) = db.get_settings() {
            *state.theme.write() = ThemeMode::from_str(&s.theme);
            *state.settings.write() = s;
        }
        *state.toast_message.write() = Some("Cadangan berhasil dipulihkan".to_string());
    } else {
        *state.toast_message.write() = Some("Format JSON cadangan tidak valid".to_string());
    }
}
