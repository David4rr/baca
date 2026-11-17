use crate::db::models::{BackupData, Book, Bookmark, UserSettings};
use crate::db::Database;
use crate::engine::parse_epub;
use crate::state::{AppState, View};
use crate::theme::ThemeMode;
use dioxus::prelude::*;
use std::path::Path;
use std::sync::Arc;

pub fn open_book_reader(db: Arc<Database>, mut state: AppState, book: Book) {
    let book_id = book.id.clone();
    let path = book.file_path.clone();
    let now = chrono::Utc::now().timestamp();

    *state.bookmarks.write() = db.get_bookmarks(&book_id).unwrap_or_default();

    if let Some(b) = state.books.write().iter_mut().find(|b| b.id == book_id) {
        b.last_read_at = now;
    }

    // Touch last_read_at without rewriting the (possibly stale) chapter snapshot.
    let db_touch = db.clone();
    dioxus::prelude::spawn(async move {
        let _ = db_touch.touch_last_read(&book_id);
    });

    dioxus::prelude::spawn(async move {
        if book.format.eq_ignore_ascii_case("pdf") {
            *state.current_chapter.write() = book.current_chapter;
            *state.current_view.write() = View::Reader { book, epub: None };
            return;
        }

        let path_for_task = path.clone();
        let parsed = tokio::task::spawn_blocking(move || parse_epub(&path_for_task)).await;
        match parsed {
            Ok(Ok(parsed)) => {
                *state.current_chapter.write() = book.current_chapter.min(parsed.chapters.len().saturating_sub(1));
                let mut updated_book = book;
                updated_book.last_read_at = now;
                *state.current_view.write() = View::Reader {
                    book: updated_book,
                    epub: Some(Arc::new(parsed)),
                };
            }
            Ok(Err(e)) => *state.toast_message.write() = Some(format!("Gagal membuka buku: {e}")),
            Err(e) => *state.toast_message.write() = Some(format!("Gagal membuka buku: {e}")),
        }
    });
}

pub fn save_downloaded_book(db: Arc<Database>, mut state: AppState, book: Book) {
    match db.insert_or_update_book(&book) {
        Ok(()) => {
            state.books.write().retain(|b| b.id != book.id);
            state.books.write().insert(0, book);
        }
        Err(e) => *state.toast_message.write() = Some(format!("Gagal menyimpan buku: {e}")),
    }
}

pub fn delete_book_from_shelf(db: Arc<Database>, mut state: AppState, book_id: String) {
    let path_opt = state
        .books
        .read()
        .iter()
        .find(|b| b.id == book_id)
        .map(|b| b.file_path.clone());

    dioxus::prelude::spawn(async move {
        match db.delete_book(&book_id) {
            Ok(()) => {
                if let Some(path) = path_opt {
                    remove_library_file(&path);
                }
                state.books.write().retain(|b| b.id != book_id);
                *state.toast_message.write() = Some("Buku dihapus dari rak".to_string());
            }
            Err(e) => *state.toast_message.write() = Some(format!("Gagal menghapus buku: {e}")),
        }
    });
}

/// Deletes a file only if it really lives inside this app's private books dir.
fn remove_library_file(path: &str) {
    let Ok(app_dir) = Database::get_app_dir() else { return };
    let books_dir = app_dir.join("books");
    let Ok(canonical_books) = books_dir.canonicalize() else { return };
    let Ok(canonical_file) = Path::new(path).canonicalize() else { return };
    if canonical_file.starts_with(&canonical_books) {
        let _ = std::fs::remove_file(canonical_file);
    }
}

pub fn update_reading_progress(db: Arc<Database>, mut state: AppState, chapter: usize, page_fraction: f32) {
    *state.current_chapter.write() = chapter;

    let (book_id, total) = match state.current_view.read().clone() {
        View::Reader { book, epub } => {
            let total = epub.as_ref().map(|e| e.chapters.len()).unwrap_or(1).max(1);
            (book.id, total)
        }
        _ => return,
    };

    let pct = compute_progress(chapter, page_fraction, total);

    if let Some(b) = state.books.write().iter_mut().find(|b| b.id == book_id) {
        b.current_chapter = chapter;
        b.progress_percent = pct;
    }

    dioxus::prelude::spawn(async move {
        let _ = db.update_progress(&book_id, chapter, pct);
    });
}

pub(crate) fn compute_progress(chapter: usize, page_fraction: f32, total: usize) -> f32 {
    let total = total.max(1);
    (((chapter as f32) + page_fraction.clamp(0.0, 1.0)) / total as f32).clamp(0.0, 1.0)
}

pub fn update_pdf_progress(db: Arc<Database>, mut state: AppState, progress: f32) {
    let pct = progress.clamp(0.0, 1.0);
    if let View::Reader { book, .. } = state.current_view.read().clone() {
        if let Some(b) = state.books.write().iter_mut().find(|b| b.id == book.id) {
            b.progress_percent = pct;
        }
        dioxus::prelude::spawn(async move {
            let _ = db.update_progress(&book.id, 0, pct);
        });
    }
}

pub fn record_reading_time(db: Arc<Database>, book_id: String, duration_seconds: u32) {
    if duration_seconds < 5 {
        return;
    }
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
    let db_lookup = db.clone();
    let lookup_id = book_id.clone();
    dioxus::prelude::spawn(async move {
        let existing = db_lookup
            .get_bookmarks(&lookup_id)
            .unwrap_or_default()
            .into_iter()
            .find(|b| b.chapter_index == chapter_index);

        if let Some(bm) = existing {
            let id = bm.id.clone();
            if db_lookup.delete_bookmark(&id).is_ok() {
                state.bookmarks.write().retain(|b| b.id != id);
                *state.toast_message.write() = Some("Markah buku dihapus".to_string());
            }
        } else {
            let mark = Bookmark {
                id: uuid::Uuid::new_v4().to_string(),
                book_id: lookup_id,
                chapter_index,
                chapter_title,
                text_snippet: snippet,
                note: None,
                color_tag: "amber".to_string(),
                created_at: chrono::Utc::now().timestamp(),
            };
            let mark_clone = mark.clone();
            if db_lookup.add_bookmark(&mark_clone).is_ok() {
                state.bookmarks.write().push(mark_clone);
                *state.toast_message.write() = Some("Markah buku disimpan".to_string());
            }
        }
    });
}

pub fn delete_bookmark_by_id(db: Arc<Database>, mut state: AppState, bookmark_id: String) {
    dioxus::prelude::spawn(async move {
        match db.delete_bookmark(&bookmark_id) {
            Ok(()) => {
                state.bookmarks.write().retain(|b| b.id != bookmark_id);
                *state.toast_message.write() = Some("Markah dihapus".to_string());
            }
            Err(e) => *state.toast_message.write() = Some(format!("Gagal menghapus markah: {e}")),
        }
    });
}

pub fn save_user_settings(db: Arc<Database>, mut state: AppState, new_settings: UserSettings) {
    let saved = new_settings.clone();
    match db.save_settings(&saved) {
        Ok(()) => {
            *state.theme.write() = ThemeMode::from_name(&saved.theme);
            *state.settings.write() = saved;
        }
        Err(e) => *state.toast_message.write() = Some(format!("Gagal menyimpan pengaturan: {e}")),
    }
}

pub fn export_library_backup(db: Arc<Database>, mut state: AppState) {
    let data = match db.export_backup() {
        Ok(d) => d,
        Err(e) => {
            *state.toast_message.write() = Some(format!("Gagal mengekspor cadangan: {e}"));
            return;
        }
    };
    let json = match serde_json::to_string_pretty(&data) {
        Ok(j) => j,
        Err(e) => {
            *state.toast_message.write() = Some(format!("Gagal menyusun cadangan: {e}"));
            return;
        }
    };
    let dest = match write_backup_file(&json) {
        Ok(p) => p,
        Err(e) => {
            *state.toast_message.write() = Some(e);
            return;
        }
    };
    *state.toast_message.write() = Some(format!("Cadangan disimpan ke {}", dest.display()));
}

fn write_backup_file(json: &str) -> Result<std::path::PathBuf, String> {
    let app_dir = Database::get_app_dir()?;
    let tmp = app_dir.join("baca_backup.json.tmp");
    std::fs::write(&tmp, json).map_err(|e| format!("Gagal menulis cadangan: {e}"))?;

    #[cfg(target_os = "android")]
    {
        let shared = std::path::Path::new("/sdcard/Download/baca_backup.json");
        if std::fs::rename(&tmp, shared).is_ok() {
            return Ok(shared.to_path_buf());
        }
        // Scoped storage blocks shared Downloads on newer Android; keep the
        // backup in the app's private writable directory instead.
        let private = app_dir.join("baca_backup.json");
        std::fs::rename(&tmp, &private)
            .map_err(|e| format!("Gagal menyimpan cadangan: {e}"))?;
        Ok(private)
    }
    #[cfg(not(target_os = "android"))]
    {
        let dest = app_dir.join("baca_backup.json");
        std::fs::rename(&tmp, &dest).map_err(|e| format!("Gagal menyimpan cadangan: {e}"))?;
        Ok(dest)
    }
}

pub fn restore_library_backup(db: Arc<Database>, mut state: AppState, json_text: String) {
    let data = match serde_json::from_str::<BackupData>(&json_text) {
        Ok(d) => d,
        Err(e) => {
            *state.toast_message.write() = Some(format!("Format JSON cadangan tidak valid: {e}"));
            return;
        }
    };

    if let Err(e) = db.restore_backup(data) {
        *state.toast_message.write() = Some(format!("Pemulihan gagal, tidak ada data yang diubah: {e}"));
        return;
    }

    match db.get_all_books() {
        Ok(books) => *state.books.write() = books,
        Err(e) => *state.toast_message.write() = Some(format!("Cadangan dipulihkan, gagal memuat rak: {e}")),
    }
    if let Ok(s) = db.get_settings() {
        *state.theme.write() = ThemeMode::from_name(&s.theme);
        *state.settings.write() = s;
    }
    *state.toast_message.write() = Some("Cadangan berhasil dipulihkan".to_string());
}


#[cfg(test)]
mod tests {
    use super::compute_progress;

    #[test]
    fn last_page_of_last_chapter_is_complete() {
        assert_eq!(compute_progress(9, 1.0, 10), 1.0);
        assert_eq!(compute_progress(0, 0.5, 10), 0.05);
        assert_eq!(compute_progress(0, 0.0, 0), 0.0);
        assert_eq!(compute_progress(5, 2.0, 10), 0.6);
    }
}
