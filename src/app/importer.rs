use crate::db::models::Book;
use crate::db::Database;
use crate::engine::parse_epub;
use std::path::Path;

/// Centralized, DRY function to import an e-book file (EPUB or PDF) into private local storage and DB.
pub fn import_local_file(db: &Database, file_name: &str, raw_bytes: Option<&[u8]>, source_path: Option<&Path>) -> Result<Book, String> {
    let app_dir = Database::get_app_dir();
    let books_dir = app_dir.join("books");
    let _ = std::fs::create_dir_all(&books_dir);

    let clean_id = uuid::Uuid::new_v4().to_string();
    let is_pdf = file_name.to_lowercase().ends_with(".pdf");
    let ext = if is_pdf { "pdf" } else { "epub" };
    let dest_path = books_dir.join(format!("{}.{}", clean_id, ext));

    if let Some(bytes) = raw_bytes {
        std::fs::write(&dest_path, bytes).map_err(|e| format!("Gagal menyimpan berkas: {}", e))?;
    } else if let Some(src) = source_path {
        std::fs::copy(src, &dest_path).map_err(|e| format!("Gagal menyalin berkas: {}", e))?;
    } else {
        return Err("Tidak ada data berkas yang diberikan".to_string());
    }

    let now = chrono::Utc::now().timestamp();
    let book = if is_pdf {
        Book {
            id: clean_id,
            title: file_name.trim_end_matches(".pdf").replace(['_', '-'], " "),
            author: "Buku Lokal".to_string(),
            file_path: dest_path.to_string_lossy().to_string(),
            format: "pdf".to_string(),
            cover_image: None,
            total_chapters: 1,
            current_chapter: 0,
            progress_percent: 0.0,
            last_read_at: now,
            created_at: now,
        }
    } else {
        match parse_epub(&dest_path) {
            Ok(parsed) => Book {
                id: clean_id,
                title: if !parsed.title.is_empty() { parsed.title } else { file_name.to_string() },
                author: if !parsed.author.is_empty() { parsed.author } else { "Buku Lokal".to_string() },
                file_path: dest_path.to_string_lossy().to_string(),
                format: "epub".to_string(),
                cover_image: parsed.cover_image,
                total_chapters: parsed.chapters.len(),
                current_chapter: 0,
                progress_percent: 0.0,
                last_read_at: now,
                created_at: now,
            },
            Err(e) => return Err(format!("Gagal membaca EPUB: {}", e)),
        }
    };

    db.insert_or_update_book(&book)
        .map_err(|e| format!("Gagal menyimpan metadata buku: {}", e))?;

    Ok(book)
}
