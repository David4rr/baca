use rusqlite::{Connection, Result};

pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS books (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            file_path TEXT NOT NULL,
            format TEXT NOT NULL,
            cover_image TEXT,
            total_chapters INTEGER NOT NULL,
            current_chapter INTEGER NOT NULL DEFAULT 0,
            progress_percent REAL NOT NULL DEFAULT 0.0,
            last_read_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS bookmarks (
            id TEXT PRIMARY KEY,
            book_id TEXT NOT NULL,
            chapter_index INTEGER NOT NULL,
            chapter_title TEXT NOT NULL,
            text_snippet TEXT NOT NULL,
            note TEXT,
            color_tag TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            theme TEXT NOT NULL,
            font_family TEXT NOT NULL,
            font_size INTEGER NOT NULL,
            line_height REAL NOT NULL,
            margin_horizontal INTEGER NOT NULL,
            gemini_api_key TEXT,
            gemini_model TEXT
        );

        CREATE TABLE IF NOT EXISTS reading_sessions (
            id TEXT PRIMARY KEY,
            book_id TEXT NOT NULL,
            duration_seconds INTEGER NOT NULL,
            timestamp INTEGER NOT NULL,
            date_str TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_books_last_read ON books(last_read_at);
        CREATE INDEX IF NOT EXISTS idx_sessions_date ON reading_sessions(date_str);
        CREATE INDEX IF NOT EXISTS idx_sessions_book ON reading_sessions(book_id);
        CREATE INDEX IF NOT EXISTS idx_bookmarks_book ON bookmarks(book_id, chapter_index);",
    )?;

    migrate(conn)?;
    Ok(())
}

fn migrate(conn: &Connection) -> Result<()> {
    let version: i64 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;

    if version < 2 {
        add_column_if_missing(conn, "settings", "gemini_api_key", "ALTER TABLE settings ADD COLUMN gemini_api_key TEXT")?;
        add_column_if_missing(conn, "settings", "gemini_model", "ALTER TABLE settings ADD COLUMN gemini_model TEXT")?;
        conn.execute_batch("PRAGMA user_version = 2")?;
    }

    Ok(())
}

fn add_column_if_missing(conn: &Connection, table: &str, column: &str, ddl: &str) -> Result<()> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({table})"))?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?;
        if name == column {
            return Ok(());
        }
    }
    conn.execute(ddl, [])?;
    Ok(())
}
