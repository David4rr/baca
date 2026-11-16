use serde::Deserialize;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize)]
struct ArchiveMetadata {
    files: Option<Vec<ArchiveFile>>,
}

#[derive(Debug, Clone, Deserialize)]
struct ArchiveFile {
    name: Option<String>,
}

pub async fn get_archive_format_file(identifier: &str, format_kind: &str) -> Result<Option<String>, String> {
    let is_pdf = format_kind.to_lowercase() == "pdf";

    if identifier.starts_with("http://") || identifier.starts_with("https://") {
        return Ok(Some(identifier.to_string()));
    }

    if let Some(rest) = identifier.strip_prefix("gutenberg:") {
        let (epub_direct, pdf_direct) = rest.split_once('|').unwrap_or((rest, ""));
        if is_pdf {
            if !pdf_direct.is_empty() {
                return Ok(Some(pdf_direct.to_string()));
            } else {
                return Ok(None);
            }
        } else if !epub_direct.is_empty() {
            return Ok(Some(epub_direct.to_string()));
        } else {
            return Ok(None);
        }
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(12))
        .build()
        .map_err(|e| e.to_string())?;

    let url = format!("https://archive.org/metadata/{}", identifier);
    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) Baca/3.3.0")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch metadata: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Metadata request failed: {}", resp.status()));
    }

    let meta: ArchiveMetadata = resp
        .json()
        .await
        .map_err(|e| format!("JSON parse error: {}", e))?;

    if let Some(files) = meta.files {
        for file in files {
            if let Some(name) = file.name {
                let lower = name.to_lowercase();
                if is_pdf {
                    if lower.ends_with(".pdf") && !lower.contains("_thumb") {
                        return Ok(Some(name));
                    }
                } else if lower.ends_with(".epub") {
                    return Ok(Some(name));
                }
            }
        }
    }

    Ok(None)
}

pub async fn get_archive_epub_file(identifier: &str) -> Result<Option<String>, String> {
    get_archive_format_file(identifier, "epub").await
}

pub async fn download_archive_book_with_progress<F>(
    identifier: &str,
    filename: &str,
    destination: &Path,
    on_progress: F,
) -> Result<(), String>
where
    F: Fn(f32) + Send + 'static,
{
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(15))
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| e.to_string())?;

    let file_url = if filename.starts_with("http://") || filename.starts_with("https://") {
        filename.to_string()
    } else {
        format!(
            "https://archive.org/download/{}/{}",
            identifier,
            filename.trim_start_matches('/')
        )
    };

    let mut resp = client
        .get(&file_url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Baca/3.3.0",
        )
        .send()
        .await
        .map_err(|e| format!("Koneksi jaringan gagal: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Unduhan gagal dengan status: {}", resp.status()));
    }

    if let Some(parent) = destination.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let total_size = resp.content_length().unwrap_or(0);
    let mut file = std::fs::File::create(destination)
        .map_err(|e| format!("Gagal membuat berkas penyimpanan: {}", e))?;

    let mut downloaded: usize = 0;

    while let Some(chunk) = resp
        .chunk()
        .await
        .map_err(|e| format!("Gagal membaca paket data: {}", e))?
    {
        file.write_all(&chunk)
            .map_err(|e| format!("Gagal menulis data ke disk: {}", e))?;
        downloaded += chunk.len();
        if total_size > 0 {
            let pct = (downloaded as f32 / total_size as f32).clamp(0.0, 1.0);
            on_progress(pct);
        } else {
            let est_pct = ((downloaded as f32 / (downloaded as f32 + 500_000.0)) * 0.9).min(0.95);
            on_progress(est_pct);
        }
    }

    let _ = file.flush();
    on_progress(1.0);

    Ok(())
}

pub async fn download_archive_book(
    identifier: &str,
    filename: &str,
    destination: &Path,
) -> Result<(), String> {
    download_archive_book_with_progress(identifier, filename, destination, |_| {}).await
}

pub async fn download_format_file<F>(
    archive_book: &super::models::ArchiveBook,
    format_str: &str,
    on_progress: F,
) -> Result<crate::db::models::Book, String>
where
    F: Fn(f32) + Send + 'static,
{
    let id = &archive_book.identifier;
    let fmt = format_str.to_lowercase();
    let filename = get_archive_format_file(id, &fmt)
        .await?
        .ok_or_else(|| format!("Format {} tidak tersedia", fmt.to_uppercase()))?;

    let app_dir = crate::db::Database::get_app_dir();
    let books_dir = app_dir.join("books");
    let _ = std::fs::create_dir_all(&books_dir);
    let clean_id = id.replace(|c: char| !c.is_alphanumeric(), "_");
    let ext = if fmt == "pdf" { "pdf" } else { "epub" };
    let dest_path = books_dir.join(format!("{}.{}", clean_id, ext));

    download_archive_book_with_progress(id, &filename, &dest_path, on_progress).await?;

    let now = chrono::Utc::now().timestamp();
    let book = if fmt == "pdf" {
        crate::db::models::Book {
            id: uuid::Uuid::new_v4().to_string(),
            title: archive_book.title.clone(),
            author: archive_book.author_display(),
            file_path: dest_path.to_string_lossy().to_string(),
            format: "pdf".to_string(),
            cover_image: Some(archive_book.thumbnail_url()),
            total_chapters: 1,
            current_chapter: 0,
            progress_percent: 0.0,
            last_read_at: 0,
            created_at: now,
        }
    } else if let Ok(parsed) = crate::engine::parse_epub(&dest_path) {
        crate::db::models::Book {
            id: uuid::Uuid::new_v4().to_string(),
            title: if !parsed.title.is_empty() { parsed.title } else { archive_book.title.clone() },
            author: if !parsed.author.is_empty() { parsed.author } else { archive_book.author_display() },
            file_path: dest_path.to_string_lossy().to_string(),
            format: "epub".to_string(),
            cover_image: parsed.cover_image.or_else(|| Some(archive_book.thumbnail_url())),
            total_chapters: parsed.chapters.len(),
            current_chapter: 0,
            progress_percent: 0.0,
            last_read_at: 0,
            created_at: now,
        }
    } else {
        crate::db::models::Book {
            id: uuid::Uuid::new_v4().to_string(),
            title: archive_book.title.clone(),
            author: archive_book.author_display(),
            file_path: dest_path.to_string_lossy().to_string(),
            format: "epub".to_string(),
            cover_image: Some(archive_book.thumbnail_url()),
            total_chapters: 1,
            current_chapter: 0,
            progress_percent: 0.0,
            last_read_at: 0,
            created_at: now,
        }
    };

    Ok(book)
}
