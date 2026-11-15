pub mod models;

use models::{BackupData, Book, Bookmark, DailyReadingStats, ReadingSession, UserSettings};
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

    pub fn get_app_dir() -> PathBuf {
        #[cfg(target_os = "android")]
        {
            let pkg = std::fs::read_to_string("/proc/self/cmdline")
                .map(|s| s.trim_matches('\0').trim().to_string())
                .unwrap_or_default();

            let mut candidates = Vec::new();
            if !pkg.is_empty() {
                candidates.push(format!("/data/data/{}/files", pkg));
                candidates.push(format!("/data/user/0/{}/files", pkg));
            }
            candidates.push("/data/data/com.example.Baca/files".to_string());
            candidates.push("/data/data/com.baca.reader/files".to_string());
            candidates.push("/sdcard/Download".to_string());

            for dir_str in candidates {
                let dir = PathBuf::from(&dir_str);
                if std::fs::create_dir_all(&dir).is_ok() {
                    return dir;
                }
            }
            PathBuf::from(".")
        }
        #[cfg(not(target_os = "android"))]
        {
            let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
            path.push("baca_reader");
            let _ = std::fs::create_dir_all(&path);
            path
        }
    }

    fn get_db_path() -> PathBuf {
        let mut path = Self::get_app_dir();
        path.push("library.db");
        path
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
            );"
        )?;

        // Safe migrations for newly added columns if table previously existed
        let _ = self.conn.execute("ALTER TABLE settings ADD COLUMN gemini_api_key TEXT", []);
        let _ = self.conn.execute("ALTER TABLE settings ADD COLUMN gemini_model TEXT", []);

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
        self.conn.execute("DELETE FROM bookmarks WHERE book_id = ?1", params![book_id])?;
        self.conn.execute("DELETE FROM reading_sessions WHERE book_id = ?1", params![book_id])?;
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

    pub fn get_all_bookmarks(&self) -> Result<Vec<Bookmark>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, book_id, chapter_index, chapter_title, text_snippet, note, color_tag, created_at 
             FROM bookmarks ORDER BY created_at DESC"
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

    pub fn record_reading_session(&self, book_id: &str, duration_seconds: u32) -> Result<()> {
        let now = chrono::Utc::now();
        let date_str = now.format("%Y-%m-%d").to_string();
        let session_id = uuid::Uuid::new_v4().to_string();

        self.conn.execute(
            "INSERT INTO reading_sessions (id, book_id, duration_seconds, timestamp, date_str)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![session_id, book_id, duration_seconds, now.timestamp(), date_str],
        )?;
        Ok(())
    }

    pub fn get_7_day_stats(&self) -> Result<Vec<DailyReadingStats>> {
        let now = chrono::Utc::now();
        let mut stats = Vec::new();

        for i in (0..7).rev() {
            let day = now - chrono::Duration::days(i);
            let date_str = day.format("%Y-%m-%d").to_string();
            let day_name = match day.format("%a").to_string().as_str() {
                "Mon" => "Sen",
                "Tue" => "Sel",
                "Wed" => "Rab",
                "Thu" => "Kam",
                "Fri" => "Jum",
                "Sat" => "Sab",
                _ => "Min",
            };

            let mut stmt = self.conn.prepare(
                "SELECT COALESCE(SUM(duration_seconds), 0) FROM reading_sessions WHERE date_str = ?1"
            )?;
            let total_seconds: u32 = stmt.query_row(params![date_str], |row| row.get(0))?;
            let minutes = (total_seconds + 30) / 60;

            stats.push(DailyReadingStats {
                date_str,
                minutes,
                day_label: day_name.to_string(),
            });
        }

        Ok(stats)
    }

    pub fn get_all_reading_sessions(&self) -> Result<Vec<ReadingSession>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, book_id, duration_seconds, timestamp, date_str FROM reading_sessions ORDER BY timestamp DESC"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(ReadingSession {
                id: row.get(0)?,
                book_id: row.get(1)?,
                duration_seconds: row.get(2)?,
                timestamp: row.get(3)?,
                date_str: row.get(4)?,
            })
        })?;

        let mut sessions = Vec::new();
        for s in rows {
            sessions.push(s?);
        }
        Ok(sessions)
    }

    pub fn get_settings(&self) -> Result<UserSettings> {
        let mut stmt = self.conn.prepare(
            "SELECT theme, font_family, font_size, line_height, margin_horizontal, gemini_api_key, gemini_model 
             FROM settings WHERE id = 1"
        )?;

        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            Ok(UserSettings {
                theme: row.get(0)?,
                font_family: row.get(1)?,
                font_size: row.get(2)?,
                line_height: row.get(3)?,
                margin_horizontal: row.get(4)?,
                gemini_api_key: row.get(5)?,
                gemini_model: row.get(6)?,
            })
        } else {
            let default_settings = UserSettings::default();
            self.save_settings(&default_settings)?;
            Ok(default_settings)
        }
    }

    pub fn save_settings(&self, s: &UserSettings) -> Result<()> {
        self.conn.execute(
            "INSERT INTO settings (id, theme, font_family, font_size, line_height, margin_horizontal, gemini_api_key, gemini_model)
             VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET
                theme = excluded.theme,
                font_family = excluded.font_family,
                font_size = excluded.font_size,
                line_height = excluded.line_height,
                margin_horizontal = excluded.margin_horizontal,
                gemini_api_key = excluded.gemini_api_key,
                gemini_model = excluded.gemini_model",
            params![
                s.theme,
                s.font_family,
                s.font_size,
                s.line_height,
                s.margin_horizontal,
                s.gemini_api_key,
                s.gemini_model,
            ],
        )?;
        Ok(())
    }

    pub fn export_backup(&self) -> Result<BackupData> {
        Ok(BackupData {
            version: "3.0.0".to_string(),
            exported_at: chrono::Utc::now().timestamp(),
            books: self.get_all_books()?,
            bookmarks: self.get_all_bookmarks()?,
            settings: self.get_settings()?,
            sessions: self.get_all_reading_sessions()?,
        })
    }

    pub fn restore_backup(&self, data: BackupData) -> Result<()> {
        for b in &data.books {
            let _ = self.insert_or_update_book(b);
        }
        for bm in &data.bookmarks {
            let _ = self.add_bookmark(bm);
        }
        for s in &data.sessions {
            let _ = self.conn.execute(
                "INSERT OR IGNORE INTO reading_sessions (id, book_id, duration_seconds, timestamp, date_str)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![s.id, s.book_id, s.duration_seconds, s.timestamp, s.date_str],
            );
        }
        let _ = self.save_settings(&data.settings);
        Ok(())
    }
}
