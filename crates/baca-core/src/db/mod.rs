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
use std::time::Duration;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Opens the persistent library. Returns an error instead of silently
    /// degrading to an in-memory database, so the UI can show a real failure.
    pub fn init() -> std::result::Result<Self, String> {
        let db_path = Self::get_db_path()?;
        Self::open_path(&db_path)
    }

    pub fn open_path(db_path: &std::path::Path) -> std::result::Result<Self, String> {
        let conn = Connection::open(db_path)
            .map_err(|e| format!("Gagal membuka basis data: {e}"))?;
        conn.busy_timeout(Duration::from_secs(5))
            .map_err(|e| format!("Gagal mengatur basis data: {e}"))?;
        conn.pragma_update(None, "foreign_keys", "ON")
            .map_err(|e| format!("Gagal mengaktifkan integritas basis data: {e}"))?;
        let _ = conn.pragma_update(None, "journal_mode", "WAL");
        let db = Self { conn };
        db.create_tables()
            .map_err(|e| format!("Gagal menyiapkan skema basis data: {e}"))?;
        Ok(db)
    }

    /// In-memory database for tests only.
    pub fn memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        let db = Self { conn };
        db.create_tables()?;
        Ok(db)
    }

    pub fn get_app_dir() -> std::result::Result<PathBuf, String> {
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
            candidates.push("/data/data/com.baca.reader/files".to_string());

            for dir_str in candidates {
                let dir = PathBuf::from(&dir_str);
                if Self::is_writable(&dir) {
                    return Ok(dir);
                }
            }
            Err("Tidak ada direktori aplikasi yang bisa ditulis".to_string())
        }
        #[cfg(not(target_os = "android"))]
        {
            let base = dirs::data_local_dir()
                .ok_or_else(|| "Direktori data pengguna tidak ditemukan".to_string())?;
            let path = base.join("baca_reader");
            if Self::is_writable(&path) {
                Ok(path)
            } else {
                Err(format!("Direktori tidak bisa ditulis: {}", path.display()))
            }
        }
    }

    fn is_writable(path: &std::path::Path) -> bool {
        if std::fs::create_dir_all(path).is_err() {
            return false;
        }
        let probe = path.join(".baca_write_probe");
        if std::fs::write(&probe, b"ok").is_err() {
            return false;
        }
        let _ = std::fs::remove_file(&probe);
        true
    }

    fn get_db_path() -> std::result::Result<PathBuf, String> {
        Ok(Self::get_app_dir()?.join("library.db"))
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

    pub fn touch_last_read(&self, book_id: &str) -> Result<()> {
        books::touch_last_read(&self.conn, book_id)
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

    pub fn get_total_reading_minutes(&self) -> Result<u32> {
        stats::get_total_reading_minutes(&self.conn)
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
