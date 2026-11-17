use crate::db::models::Book;
use crate::db::Database;
use crate::engine::parse_epub;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::Path;

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

fn hash_file(path: &Path) -> Result<String, String> {
    let mut file = std::fs::File::open(path).map_err(|e| format!("Gagal membaca berkas: {e}"))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer).map_err(|e| format!("Gagal membaca berkas: {e}"))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(hex::encode(hasher.finalize()))
}

/// Imports an e-book into private storage. The content hash is the book id, so
/// importing the same file twice is a no-op instead of a duplicate row.
pub fn import_local_file(
    db: &Database,
    file_name: &str,
    raw_bytes: Option<&[u8]>,
    source_path: Option<&Path>,
) -> Result<Book, String> {
    let app_dir = Database::get_app_dir().map_err(|e| format!("Gagal menyiapkan penyimpanan: {e}"))?;
    let books_dir = app_dir.join("books");
    std::fs::create_dir_all(&books_dir).map_err(|e| format!("Gagal menyiapkan folder buku: {e}"))?;

    let is_pdf = file_name.to_lowercase().ends_with(".pdf");
    let ext = if is_pdf { "pdf" } else { "epub" };

    let digest = if let Some(bytes) = raw_bytes {
        hash_bytes(bytes)
    } else if let Some(src) = source_path {
        hash_file(src)?
    } else {
        return Err("Tidak ada data berkas yang diberikan".to_string());
    };

    let dest_path = books_dir.join(format!("{digest}.{ext}"));

    if let Ok(existing) = db.get_all_books() {
        if let Some(book) = existing.into_iter().find(|b| b.id == digest) {
            return Ok(book);
        }
    }

    let write_result = if let Some(bytes) = raw_bytes {
        std::fs::write(&dest_path, bytes).map_err(|e| format!("Gagal menyimpan berkas: {e}"))
    } else if let Some(src) = source_path {
        std::fs::copy(src, &dest_path)
            .map(|_| ())
            .map_err(|e| format!("Gagal menyalin berkas: {e}"))
    } else {
        Err("Tidak ada data berkas yang diberikan".to_string())
    };
    write_result?;

    let now = chrono::Utc::now().timestamp();
    let book = if is_pdf {
        Book {
            id: digest,
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
            Ok(parsed) if !parsed.chapters.is_empty() => Book {
                id: digest,
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
            Ok(_) => {
                let _ = std::fs::remove_file(&dest_path);
                return Err("EPUB tidak memiliki bab yang bisa dibaca".to_string());
            }
            Err(e) => {
                let _ = std::fs::remove_file(&dest_path);
                return Err(format!("Gagal membaca EPUB: {e}"));
            }
        }
    };

    db.insert_or_update_book(&book)
        .map_err(|e| format!("Gagal menyimpan metadata buku: {e}"))?;

    Ok(book)
}
