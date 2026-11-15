pub mod backup;
pub mod bookmarks;
pub mod books;
pub mod models;
pub mod schema;
pub mod settings;
pub mod stats;

use models::{BackupData, Book, Bookmark, DailyReadingStats, ReadingSession, UserSettings};
use rusqlite::{Connection, Result};
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
                    eprintln!("[Baca DB] create_tables failed: {:?}, using memory fallback", e);
                    return Self::memory();
                }
                Ok(db)
            }
            Err(e) => {
                eprintln!("[Baca DB] Connection failed: {:?}, using memory fallback", e);
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
        schema::create_tables(&self.conn)
    }

    // Books delegation
    pub fn get_all_books(&self) -> Result<Vec<Book>> {
        books::get_all_books(&self.conn)
    }

    pub fn insert_or_update_book(&self, book: &Book) -> Result<()> {
        books::insert_or_update_book(&self.conn, book)
    }

    pub fn update_progress(&self, book_id: &str, chapter: usize, progress: f32) -> Result<()> {
        books::update_progress(&self.conn, book_id, chapter, progress)
    }

    pub fn delete_book(&self, book_id: &str) -> Result<()> {
        books::delete_book(&self.conn, book_id)
    }

    // Bookmarks delegation
    pub fn get_bookmarks(&self, book_id: &str) -> Result<Vec<Bookmark>> {
        bookmarks::get_bookmarks(&self.conn, book_id)
    }

    pub fn get_all_bookmarks(&self) -> Result<Vec<Bookmark>> {
        bookmarks::get_all_bookmarks(&self.conn)
    }

    pub fn add_bookmark(&self, mark: &Bookmark) -> Result<()> {
        bookmarks::add_bookmark(&self.conn, mark)
    }

    pub fn delete_bookmark(&self, id: &str) -> Result<()> {
        bookmarks::delete_bookmark(&self.conn, id)
    }

    // Stats delegation
    pub fn record_reading_session(&self, book_id: &str, duration_seconds: u32) -> Result<()> {
        stats::record_reading_session(&self.conn, book_id, duration_seconds)
    }

    pub fn get_7_day_stats(&self) -> Result<Vec<DailyReadingStats>> {
        stats::get_7_day_stats(&self.conn)
    }

    pub fn get_all_reading_sessions(&self) -> Result<Vec<ReadingSession>> {
        stats::get_all_reading_sessions(&self.conn)
    }

    // Settings delegation
    pub fn get_settings(&self) -> Result<UserSettings> {
        settings::get_settings(&self.conn)
    }

    pub fn save_settings(&self, s: &UserSettings) -> Result<()> {
        settings::save_settings(&self.conn, s)
    }

    // Backup delegation
    pub fn export_backup(&self) -> Result<BackupData> {
        backup::export_backup(&self.conn)
    }

    pub fn restore_backup(&self, data: BackupData) -> Result<()> {
        backup::restore_backup(&self.conn, data)
    }
}
