use crate::db::models::BackupData;
use crate::db::{books, bookmarks, settings, stats};
use rusqlite::{params, Connection, Result};

pub fn export_backup(conn: &Connection) -> Result<BackupData> {
    Ok(BackupData {
        version: "3.0.0".to_string(),
        exported_at: chrono::Utc::now().timestamp(),
        books: books::get_all_books(conn)?,
        bookmarks: bookmarks::get_all_bookmarks(conn)?,
        settings: settings::get_settings(conn)?,
        sessions: stats::get_all_reading_sessions(conn)?,
    })
}

pub fn restore_backup(conn: &Connection, data: BackupData) -> Result<()> {
    for b in &data.books {
        let _ = books::insert_or_update_book(conn, b);
    }
    for bm in &data.bookmarks {
        let _ = bookmarks::add_bookmark(conn, bm);
    }
    for s in &data.sessions {
        let _ = conn.execute(
            "INSERT OR IGNORE INTO reading_sessions (id, book_id, duration_seconds, timestamp, date_str)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![s.id, s.book_id, s.duration_seconds, s.timestamp, s.date_str],
        );
    }
    let _ = settings::save_settings(conn, &data.settings);
    Ok(())
}
