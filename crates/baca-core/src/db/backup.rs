use crate::db::models::BackupData;
use crate::db::{bookmarks, books, settings, stats};
use rusqlite::{params, Connection, Result};

pub fn export_backup(conn: &Connection) -> Result<BackupData> {
    Ok(BackupData {
        version: env!("CARGO_PKG_VERSION").to_string(),
        exported_at: chrono::Utc::now().timestamp(),
        books: books::get_all_books(conn)?,
        bookmarks: bookmarks::get_all_bookmarks(conn)?,
        settings: {
            let mut s = settings::get_settings(conn)?;
            // Never export the Gemini credential.
            s.gemini_api_key = None;
            s
        },
        sessions: stats::get_all_reading_sessions(conn)?,
    })
}

/// Restores a backup atomically: either every row lands or nothing changes.
pub fn restore_backup(conn: &Connection, data: BackupData) -> Result<()> {
    let tx = conn.unchecked_transaction()?;

    for b in &data.books {
        books::insert_or_update_book(&tx, b)?;
    }
    for bm in &data.bookmarks {
        tx.execute(
            "INSERT OR IGNORE INTO bookmarks
             (id, book_id, chapter_index, chapter_title, text_snippet, note, color_tag, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                bm.id,
                bm.book_id,
                bm.chapter_index,
                bm.chapter_title,
                bm.text_snippet,
                bm.note,
                bm.color_tag,
                bm.created_at
            ],
        )?;
    }
    for s in &data.sessions {
        tx.execute(
            "INSERT OR IGNORE INTO reading_sessions (id, book_id, duration_seconds, timestamp, date_str)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![s.id, s.book_id, s.duration_seconds, s.timestamp, s.date_str],
        )?;
    }
    settings::save_settings(&tx, &data.settings)?;

    tx.commit()?;
    Ok(())
}
