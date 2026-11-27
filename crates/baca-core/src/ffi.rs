use std::path::Path;
use std::sync::Mutex;

use crate::blocks::{html_to_blocks, ReaderBlock};
use crate::db::models::{BackupData, Book, Bookmark, DailyReadingStats, UserSettings};
use crate::db::Database;
use crate::engine::{
    ask_gemini, check_latest_release, download_archive_book_with_progress,
    get_archive_format_file, parse_epub, search_archive, ArchiveBook,
};

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum BacaError {
    #[error("{reason}")]
    Message { reason: String },
}

impl From<String> for BacaError {
    fn from(reason: String) -> Self {
        Self::Message { reason }
    }
}

impl From<rusqlite::Error> for BacaError {
    fn from(err: rusqlite::Error) -> Self {
        Self::Message { reason: err.to_string() }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct NativeBook {
    pub id: String,
    pub title: String,
    pub author: String,
    pub file_path: String,
    pub format: String,
    pub cover_image: Option<String>,
    pub total_chapters: u32,
    pub current_chapter: u32,
    pub progress_percent: f32,
    pub last_read_at: i64,
    pub created_at: i64,
}

impl From<Book> for NativeBook {
    fn from(b: Book) -> Self {
        Self {
            id: b.id,
            title: b.title,
            author: b.author,
            file_path: b.file_path,
            format: b.format,
            cover_image: b.cover_image,
            total_chapters: b.total_chapters as u32,
            current_chapter: b.current_chapter as u32,
            progress_percent: b.progress_percent,
            last_read_at: b.last_read_at,
            created_at: b.created_at,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct NativeTocItem {
    pub title: String,
    pub chapter_index: u32,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct NativeChapter {
    pub index: u32,
    pub title: String,
    pub blocks: Vec<ReaderBlock>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct NativeEpubBook {
    pub title: String,
    pub author: String,
    pub toc: Vec<NativeTocItem>,
    pub chapters: Vec<NativeChapter>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct NativeBookmark {
    pub id: String,
    pub book_id: String,
    pub chapter_index: u32,
    pub chapter_title: String,
    pub text_snippet: String,
    pub created_at: i64,
}

impl From<Bookmark> for NativeBookmark {
    fn from(b: Bookmark) -> Self {
        Self {
            id: b.id,
            book_id: b.book_id,
            chapter_index: b.chapter_index as u32,
            chapter_title: b.chapter_title,
            text_snippet: b.text_snippet,
            created_at: b.created_at,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct NativeSettings {
    pub theme: String,
    pub font_family: String,
    pub font_size: u32,
    pub line_height: f32,
    pub margin_horizontal: u32,
    pub gemini_api_key: Option<String>,
    pub gemini_model: Option<String>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct NativeDailyStat {
    pub date_str: String,
    pub minutes: u32,
    pub day_label: String,
}

impl From<DailyReadingStats> for NativeDailyStat {
    fn from(s: DailyReadingStats) -> Self {
        Self {
            date_str: s.date_str,
            minutes: s.minutes,
            day_label: s.day_label,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct NativeSearchBook {
    pub identifier: String,
    pub title: String,
    pub creator: String,
    pub description: String,
    pub cover_url: String,
    pub epub_url: Option<String>,
    pub pdf_url: Option<String>,
}

impl From<ArchiveBook> for NativeSearchBook {
    fn from(r: ArchiveBook) -> Self {
        let creator = r.author_display();
        let cover_url = r.thumbnail_url();
        Self {
            identifier: r.identifier,
            title: r.title,
            creator,
            description: r.description.unwrap_or_default(),
            cover_url,
            epub_url: r.direct_epub_url,
            pdf_url: r.direct_pdf_url,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct NativeReleaseInfo {
    pub tag_name: String,
    pub version: String,
    pub name: String,
    pub body: String,
    pub html_url: String,
    pub apk_url: Option<String>,
}

#[derive(uniffi::Object)]
pub struct BacaCore {
    db: Mutex<Database>,
    books_dir: String,
}

#[uniffi::export]
impl BacaCore {
    #[uniffi::constructor]
    pub fn open(data_dir: String) -> Result<Self, BacaError> {
        let root = Path::new(&data_dir);
        let books_dir = root.join("books");
        std::fs::create_dir_all(&books_dir)
            .map_err(|e| BacaError::Message { reason: format!("Gagal membuat folder buku: {e}") })?;
        let db_path = root.join("library.db");
        let db = Database::open_path(&db_path)?;
        Ok(Self {
            db: Mutex::new(db),
            books_dir: books_dir.to_string_lossy().to_string(),
        })
    }

    pub fn get_all_books(&self) -> Result<Vec<NativeBook>, BacaError> {
        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        Ok(db.get_all_books()?.into_iter().map(NativeBook::from).collect())
    }

    pub fn import_bytes(&self, file_name: String, bytes: Vec<u8>) -> Result<NativeBook, BacaError> {
        let ext = Path::new(&file_name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if ext != "epub" && ext != "pdf" {
            return Err(BacaError::Message {
                reason: "Format harus .epub atau .pdf".to_string(),
            });
        }
        let id = uuid::Uuid::new_v4().to_string();
        let dest = Path::new(&self.books_dir).join(format!("{id}.{ext}"));
        std::fs::write(&dest, &bytes)
            .map_err(|e| BacaError::Message { reason: format!("Gagal menulis berkas: {e}") })?;
        let dest_str = dest.to_string_lossy().to_string();
        let now = chrono::Utc::now().timestamp();
        let fallback_title = Path::new(&file_name)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Buku Tanpa Judul")
            .to_string();

        let book = if ext == "epub" {
            let parsed = parse_epub(&dest_str).map_err(|e| {
                let _ = std::fs::remove_file(&dest);
                BacaError::Message { reason: format!("EPUB tidak valid: {e}") }
            })?;
            Book {
                id,
                title: if parsed.title.trim().is_empty() { fallback_title } else { parsed.title },
                author: parsed.author,
                file_path: dest_str,
                format: "epub".to_string(),
                cover_image: parsed.cover_image,
                total_chapters: parsed.chapters.len(),
                current_chapter: 0,
                progress_percent: 0.0,
                last_read_at: 0,
                created_at: now,
            }
        } else {
            Book {
                id,
                title: fallback_title,
                author: "Dokumen PDF".to_string(),
                file_path: dest_str,
                format: "pdf".to_string(),
                cover_image: None,
                total_chapters: 1,
                current_chapter: 0,
                progress_percent: 0.0,
                last_read_at: 0,
                created_at: now,
            }
        };

        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        db.insert_or_update_book(&book)?;
        Ok(NativeBook::from(book))
    }

    pub fn delete_book(&self, book_id: String, file_path: String) -> Result<(), BacaError> {
        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        db.delete_book(&book_id)?;
        let _ = std::fs::remove_file(&file_path);
        Ok(())
    }

    pub fn parse_epub_blocks(&self, file_path: String) -> Result<NativeEpubBook, BacaError> {
        let parsed = parse_epub(&file_path).map_err(|e| BacaError::Message { reason: e })?;
        Ok(NativeEpubBook {
            title: parsed.title,
            author: parsed.author,
            toc: parsed
                .toc
                .into_iter()
                .map(|t| NativeTocItem {
                    title: t.title,
                    chapter_index: t.chapter_index as u32,
                })
                .collect(),
            chapters: parsed
                .chapters
                .into_iter()
                .map(|ch| NativeChapter {
                    index: ch.index as u32,
                    title: ch.title,
                    blocks: html_to_blocks(&ch.html_content),
                })
                .collect(),
        })
    }

    pub fn touch_last_read(&self, book_id: String) -> Result<(), BacaError> {
        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        db.touch_last_read(&book_id)?;
        Ok(())
    }

    pub fn update_progress(&self, book_id: String, chapter: u32, progress: f32) -> Result<(), BacaError> {
        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        db.update_progress(&book_id, chapter as usize, progress.clamp(0.0, 1.0))?;
        Ok(())
    }

    pub fn record_reading_session(&self, book_id: String, duration_seconds: u32) -> Result<(), BacaError> {
        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        db.record_reading_session(&book_id, duration_seconds)?;
        Ok(())
    }

    pub fn get_7_day_stats(&self) -> Result<Vec<NativeDailyStat>, BacaError> {
        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        Ok(db.get_7_day_stats()?.into_iter().map(NativeDailyStat::from).collect())
    }

    pub fn get_bookmarks(&self, book_id: String) -> Result<Vec<NativeBookmark>, BacaError> {
        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        Ok(db.get_bookmarks(&book_id)?.into_iter().map(NativeBookmark::from).collect())
    }

    pub fn toggle_bookmark(
        &self,
        book_id: String,
        chapter_index: u32,
        chapter_title: String,
        snippet: String,
    ) -> Result<bool, BacaError> {
        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        let existing = db
            .get_bookmarks(&book_id)?
            .into_iter()
            .find(|b| b.chapter_index == chapter_index as usize);
        if let Some(bm) = existing {
            db.delete_bookmark(&bm.id)?;
            Ok(false)
        } else {
            let mark = Bookmark {
                id: uuid::Uuid::new_v4().to_string(),
                book_id,
                chapter_index: chapter_index as usize,
                chapter_title,
                text_snippet: snippet,
                note: None,
                color_tag: "amber".to_string(),
                created_at: chrono::Utc::now().timestamp(),
            };
            db.add_bookmark(&mark)?;
            Ok(true)
        }
    }

    pub fn get_settings(&self) -> Result<NativeSettings, BacaError> {
        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        let s = db.get_settings()?;
        Ok(NativeSettings {
            theme: s.theme,
            font_family: s.font_family,
            font_size: s.font_size,
            line_height: s.line_height,
            margin_horizontal: s.margin_horizontal,
            gemini_api_key: s.gemini_api_key,
            gemini_model: s.gemini_model,
        })
    }

    pub fn save_settings(&self, settings: NativeSettings) -> Result<(), BacaError> {
        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        db.save_settings(&UserSettings {
            theme: settings.theme,
            font_family: settings.font_family,
            font_size: settings.font_size,
            line_height: settings.line_height,
            margin_horizontal: settings.margin_horizontal,
            gemini_api_key: settings.gemini_api_key,
            gemini_model: settings.gemini_model,
        })?;
        Ok(())
    }

    pub fn search_online(&self, query: String, page: u32) -> Result<Vec<NativeSearchBook>, BacaError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| BacaError::Message { reason: e.to_string() })?;
        let results = rt
            .block_on(search_archive(&query, page))
            .map_err(|e| BacaError::Message { reason: e })?;
        Ok(results.into_iter().map(NativeSearchBook::from).collect())
    }

    pub fn delete_bookmark(&self, bookmark_id: String) -> Result<(), BacaError> {
        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        db.delete_bookmark(&bookmark_id)?;
        Ok(())
    }

    pub fn get_total_reading_minutes(&self) -> Result<u32, BacaError> {
        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        Ok(db.get_total_reading_minutes()?)
    }

    pub fn download_online_book(
        &self,
        identifier: String,
        title: String,
        author: String,
        format: String,
        direct_url: Option<String>,
    ) -> Result<NativeBook, BacaError> {
        let fmt = format.to_lowercase();
        let ext = if fmt == "pdf" { "pdf" } else { "epub" };
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| BacaError::Message { reason: e.to_string() })?;

        let target = rt
            .block_on(async {
                if let Some(u) = direct_url.filter(|s| !s.trim().is_empty()) {
                    Ok(u)
                } else {
                    get_archive_format_file(&identifier, ext)
                        .await?
                        .ok_or_else(|| format!("Format {} tidak tersedia", ext.to_uppercase()))
                }
            })
            .map_err(|e| BacaError::Message { reason: e })?;

        let id = uuid::Uuid::new_v4().to_string();
        let dest = Path::new(&self.books_dir).join(format!("{id}.{ext}"));
        rt.block_on(download_archive_book_with_progress(&identifier, &target, &dest, |_| {}))
            .map_err(|e| {
                let _ = std::fs::remove_file(&dest);
                BacaError::Message { reason: e }
            })?;

        let dest_str = dest.to_string_lossy().to_string();
        let now = chrono::Utc::now().timestamp();
        let book = if ext == "epub" {
            let parsed = parse_epub(&dest_str).map_err(|e| {
                let _ = std::fs::remove_file(&dest);
                BacaError::Message { reason: format!("EPUB tidak valid: {e}") }
            })?;
            Book {
                id,
                title: if parsed.title.trim().is_empty() { title } else { parsed.title },
                author: if parsed.author.trim().is_empty() { author } else { parsed.author },
                file_path: dest_str,
                format: "epub".to_string(),
                cover_image: parsed.cover_image,
                total_chapters: parsed.chapters.len(),
                current_chapter: 0,
                progress_percent: 0.0,
                last_read_at: 0,
                created_at: now,
            }
        } else {
            Book {
                id,
                title,
                author,
                file_path: dest_str,
                format: "pdf".to_string(),
                cover_image: None,
                total_chapters: 1,
                current_chapter: 0,
                progress_percent: 0.0,
                last_read_at: 0,
                created_at: now,
            }
        };

        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        db.insert_or_update_book(&book)?;
        Ok(NativeBook::from(book))
    }

    pub fn ask_ai(&self, prompt: String, api_key: String, model: String) -> Result<String, BacaError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| BacaError::Message { reason: e.to_string() })?;
        rt.block_on(ask_gemini(&api_key, &model, &prompt))
            .map_err(|e| BacaError::Message { reason: e })
    }

    pub fn export_backup_json(&self) -> Result<String, BacaError> {
        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        let backup = db.export_backup()?;
        serde_json::to_string_pretty(&backup)
            .map_err(|e| BacaError::Message { reason: format!("Gagal serialisasi backup: {e}") })
    }

    pub fn import_backup_json(&self, json: String) -> Result<u32, BacaError> {
        let data: BackupData = serde_json::from_str(&json)
            .map_err(|e| BacaError::Message { reason: format!("Format backup JSON tidak valid: {e}") })?;
        let count = data.books.len() as u32;
        let db = self.db.lock().map_err(|e| BacaError::Message { reason: e.to_string() })?;
        db.restore_backup(data)?;
        Ok(count)
    }

    pub fn check_for_update(&self, current_version: String) -> Result<Option<NativeReleaseInfo>, BacaError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| BacaError::Message { reason: e.to_string() })?;
        let rel = rt
            .block_on(check_latest_release(&current_version))
            .map_err(|e| BacaError::Message { reason: e })?;
        Ok(rel.map(|r| NativeReleaseInfo {
            tag_name: r.tag_name,
            version: r.version,
            name: r.name,
            body: r.body,
            html_url: r.html_url,
            apk_url: r.apk_url,
        }))
    }
}
