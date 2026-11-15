use crate::db::models::Book;
use rusqlite::{params, Connection, Result};

pub fn get_all_books(conn: &Connection) -> Result<Vec<Book>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, author, file_path, format, cover_image, 
                total_chapters, current_chapter, progress_percent, last_read_at, created_at 
         FROM books ORDER BY last_read_at DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Book {
            id: row.get(0)?,
            title: row.get(1)?,
            author: row.get(2)?,
            file_path: row.get(3)?,
            format: row.get(4)?,
            cover_image: row.get(5)?,
            total_chapters: row.get(6)?,
            current_chapter: row.get(7)?,
            progress_percent: row.get(8)?,
            last_read_at: row.get(9)?,
            created_at: row.get(10)?,
        })
    })?;

    let mut books = Vec::new();
    for b in rows {
        books.push(b?);
    }
    Ok(books)
}

pub fn insert_or_update_book(conn: &Connection, book: &Book) -> Result<()> {
    conn.execute(
        "INSERT INTO books (id, title, author, file_path, format, cover_image, 
                            total_chapters, current_chapter, progress_percent, last_read_at, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
         ON CONFLICT(id) DO UPDATE SET
            title = excluded.title,
            author = excluded.author,
            cover_image = excluded.cover_image,
            total_chapters = excluded.total_chapters,
            current_chapter = excluded.current_chapter,
            progress_percent = excluded.progress_percent,
            last_read_at = excluded.last_read_at",
        params![
            book.id,
            book.title,
            book.author,
            book.file_path,
            book.format,
            book.cover_image,
            book.total_chapters,
            book.current_chapter,
            book.progress_percent,
            book.last_read_at,
            book.created_at,
        ],
    )?;
    Ok(())
}

pub fn update_progress(conn: &Connection, book_id: &str, chapter: usize, progress: f32) -> Result<()> {
    let now = chrono::Utc::now().timestamp();
    conn.execute(
        "UPDATE books SET current_chapter = ?1, progress_percent = ?2, last_read_at = ?3 WHERE id = ?4",
        params![chapter, progress, now, book_id],
    )?;
    Ok(())
}

pub fn delete_book(conn: &Connection, book_id: &str) -> Result<()> {
    conn.execute("DELETE FROM bookmarks WHERE book_id = ?1", params![book_id])?;
    conn.execute("DELETE FROM reading_sessions WHERE book_id = ?1", params![book_id])?;
    conn.execute("DELETE FROM books WHERE id = ?1", params![book_id])?;
    Ok(())
}
