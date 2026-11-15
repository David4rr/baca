use dioxus::prelude::*;
use crate::db::models::{BackupData, Book, UserSettings};
use crate::db::Database;
use crate::engine::archive::ArchiveBook;
use crate::engine::parse_epub;
use crate::state::{AppState, View};
use crate::theme::ThemeMode;
use std::sync::Arc;
use super::download_handler::download_format_to_library;

pub fn open_book_reader(mut state: AppState, book: Book) {
    let path = book.file_path.clone();
    let book_clone = book.clone();
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
                *state.current_view.write() = View::Reader { book: book_clone, epub: Some(parsed) };
            }
            Err(e) => *state.toast_message.write() = Some(format!("Gagal membuka buku: {}", e)),
        }
    });
}

pub fn delete_book_from_shelf(db: Arc<Database>, mut state: AppState, book_id: String) {
    dioxus::prelude::spawn(async move {
        if db.delete_book(&book_id).is_ok() {
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
            let _ = db.record_reading_session(&id, 30);
        });
    }
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
            let dest = Database::get_app_dir().join("baca_backup.json");
            let _ = std::fs::write(&dest, json);
            *state.toast_message.write() = Some("Cadangan disimpan ke baca_backup.json".to_string());
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
