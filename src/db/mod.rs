pub mod models;

use models::{Book, Bookmark, UserSettings};
use rusqlite::{params, Connection, Result};
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn init() -> Result<Self> {
        let db_path = Self::get_db_path();
        if let Some(parent) = db_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        match Connection::open(&db_path) {
            Ok(conn) => {
                let db = Self { conn };
                if let Err(e) = db.create_tables() {
                    eprintln!("[Baca DB] create_tables failed: {:?}, falling back to memory DB", e);
                    return Self::memory();
                }
                Ok(db)
            }
            Err(e) => {
                eprintln!("[Baca DB] Connection::open failed for {:?}: {:?}, falling back to memory DB", db_path, e);
                Self::memory()
            }
        }
    }

    pub fn memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.create_tables()?;
        Ok(db)
    }

    fn get_db_path() -> PathBuf {
        #[cfg(target_os = "android")]
        {
            let pkg = std::fs::read_to_string("/proc/self/cmdline")
                .map(|s| s.trim_matches('\0').trim().to_string())
                .unwrap_or_default();

            let mut candidates = Vec::new();
            if !pkg.is_empty() {
                candidates.push(format!("/data/data/{}/files", pkg));
                candidates.push(format!("/data/user/0/{}/files", pkg));
                candidates.push(format!("/data/data/{}/databases", pkg));
            }
            candidates.push("/data/data/com.example.Baca/files".to_string());
            candidates.push("/data/data/com.baca.reader/files".to_string());
            candidates.push("/sdcard/Download".to_string());

            for dir_str in candidates {
                let dir = PathBuf::from(&dir_str);
                if std::fs::create_dir_all(&dir).is_ok() {
                    let db_file = dir.join("library.db");
                    if std::fs::OpenOptions::new().create(true).write(true).open(&db_file).is_ok() {
                        return db_file;
                    }
                }
            }
            PathBuf::from("library.db")
        }
        #[cfg(not(target_os = "android"))]
        {
            let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
            path.push("baca_reader");
            let _ = std::fs::create_dir_all(&path);
            path.push("library.db");
            path
        }
    }

    fn create_tables(&self) -> Result<()> {
        self.conn.execute_batch(
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
                margin_horizontal INTEGER NOT NULL
            );"
        )?;
        Ok(())
    }

    pub fn get_all_books(&self) -> Result<Vec<Book>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, author, file_path, format, cover_image, 
                    total_chapters, current_chapter, progress_percent, last_read_at, created_at 
             FROM books ORDER BY last_read_at DESC"
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

    pub fn insert_or_update_book(&self, book: &Book) -> Result<()> {
        self.conn.execute(
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

    pub fn update_progress(&self, book_id: &str, chapter: usize, progress: f32) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        self.conn.execute(
            "UPDATE books SET current_chapter = ?1, progress_percent = ?2, last_read_at = ?3 WHERE id = ?4",
            params![chapter, progress, now, book_id],
        )?;
        Ok(())
    }

    pub fn delete_book(&self, book_id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM books WHERE id = ?1", params![book_id])?;
        Ok(())
    }

    pub fn get_bookmarks(&self, book_id: &str) -> Result<Vec<Bookmark>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, book_id, chapter_index, chapter_title, text_snippet, note, color_tag, created_at 
             FROM bookmarks WHERE book_id = ?1 ORDER BY chapter_index ASC, created_at ASC"
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

    pub fn add_bookmark(&self, mark: &Bookmark) -> Result<()> {
        self.conn.execute(
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

    pub fn delete_bookmark(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM bookmarks WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_settings(&self) -> Result<UserSettings> {
        let mut stmt = self.conn.prepare(
            "SELECT theme, font_family, font_size, line_height, margin_horizontal FROM settings WHERE id = 1"
        )?;

        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            Ok(UserSettings {
                theme: row.get(0)?,
                font_family: row.get(1)?,
                font_size: row.get(2)?,
                line_height: row.get(3)?,
                margin_horizontal: row.get(4)?,
            })
        } else {
            let default_settings = UserSettings::default();
            self.save_settings(&default_settings)?;
            Ok(default_settings)
        }
    }

    pub fn save_settings(&self, s: &UserSettings) -> Result<()> {
        self.conn.execute(
            "INSERT INTO settings (id, theme, font_family, font_size, line_height, margin_horizontal)
             VALUES (1, ?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(id) DO UPDATE SET
                theme = excluded.theme,
                font_family = excluded.font_family,
                font_size = excluded.font_size,
                line_height = excluded.line_height,
                margin_horizontal = excluded.margin_horizontal",
            params![s.theme, s.font_family, s.font_size, s.line_height, s.margin_horizontal],
        )?;
        Ok(())
    }
}
