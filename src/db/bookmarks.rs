use crate::db::models::Bookmark;
use rusqlite::{params, Connection, Result};

pub fn get_bookmarks(conn: &Connection, book_id: &str) -> Result<Vec<Bookmark>> {
    let mut stmt = conn.prepare(
        "SELECT id, book_id, chapter_index, chapter_title, text_snippet, note, color_tag, created_at 
         FROM bookmarks WHERE book_id = ?1 ORDER BY chapter_index ASC, created_at ASC",
    )?;

    let rows = stmt.query_map(params![book_id], |row| {
        Ok(Bookmark {
            id: row.get(0)?,
            book_id: row.get(1)?,
            chapter_index: row.get(2)?,
            chapter_title: row.get(3)?,
            text_snippet: row.get(4)?,
            note: row.get(5)?,
            color_tag: row.get(6)?,
            created_at: row.get(7)?,
        })
    })?;

    let mut marks = Vec::new();
    for m in rows {
        marks.push(m?);
    }
    Ok(marks)
}

pub fn get_all_bookmarks(conn: &Connection) -> Result<Vec<Bookmark>> {
    let mut stmt = conn.prepare(
        "SELECT id, book_id, chapter_index, chapter_title, text_snippet, note, color_tag, created_at 
         FROM bookmarks ORDER BY created_at DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Bookmark {
            id: row.get(0)?,
            book_id: row.get(1)?,
            chapter_index: row.get(2)?,
            chapter_title: row.get(3)?,
            text_snippet: row.get(4)?,
            note: row.get(5)?,
            color_tag: row.get(6)?,
            created_at: row.get(7)?,
        })
    })?;

    let mut marks = Vec::new();
    for m in rows {
        marks.push(m?);
    }
    Ok(marks)
}

pub fn add_bookmark(conn: &Connection, mark: &Bookmark) -> Result<()> {
    conn.execute(
        "INSERT INTO bookmarks (id, book_id, chapter_index, chapter_title, text_snippet, note, color_tag, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            mark.id,
            mark.book_id,
            mark.chapter_index,
            mark.chapter_title,
            mark.text_snippet,
            mark.note,
            mark.color_tag,
            mark.created_at,
        ],
    )?;
    Ok(())
}

pub fn delete_bookmark(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM bookmarks WHERE id = ?1", params![id])?;
    Ok(())
}
