use serde::Deserialize;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

const MAX_DOWNLOAD_BYTES: usize = 512 * 1024 * 1024;

fn is_allowed_download_host(url: &str) -> bool {
    let Some(rest) = url.strip_prefix("https://") else {
        return false;
    };
    let host = rest.split(['/', '?', '#']).next().unwrap_or("");
    host == "archive.org"
        || host.ends_with(".archive.org")
        || host == "gutenberg.org"
        || host.ends_with(".gutenberg.org")
}

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
        return if is_allowed_download_host(identifier) {
            Ok(Some(identifier.to_string()))
        } else {
            Err("Sumber unduhan tidak diizinkan".to_string())
        };
    }

    if let Some(rest) = identifier.strip_prefix("gutenberg:") {
        let (epub_direct, pdf_direct) = rest.split_once('|').unwrap_or((rest, ""));
        if is_pdf {
            if !pdf_direct.is_empty() {
                if !is_allowed_download_host(pdf_direct) {
                    return Err("Sumber unduhan tidak diizinkan".to_string());
                }
                return Ok(Some(pdf_direct.to_string()));
            } else {
                return Ok(None);
            }
        } else if !epub_direct.is_empty() {
            if !is_allowed_download_host(epub_direct) {
                return Err("Sumber unduhan tidak diizinkan".to_string());
            }
            return Ok(Some(epub_direct.to_string()));
        } else {
            return Ok(None);
        }
    }

    if !identifier.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-')) {
        return Err("Identifier buku tidak valid".to_string());
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
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            if attempt.previous().len() >= 10 {
                attempt.stop()
            } else if is_allowed_download_host(attempt.url().as_str()) {
                attempt.follow()
            } else {
                attempt.stop()
            }
        }))
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| e.to_string())?;

    let file_url = if filename.starts_with("http://") || filename.starts_with("https://") {
        if !is_allowed_download_host(filename) {
            return Err("Sumber unduhan tidak diizinkan".to_string());
        }
        filename.to_string()
    } else {
        if !identifier.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-')) {
            return Err("Identifier buku tidak valid".to_string());
        }
        let encoded_name = filename
            .trim_start_matches('/')
            .replace(' ', "%20")
            .replace('#', "%23")
            .replace('?', "%3F");
        format!("https://archive.org/download/{}/{}", identifier, encoded_name)
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
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Gagal menyiapkan folder unduhan: {e}"))?;
    }

    let total_size = resp.content_length().unwrap_or(0);
    if total_size as usize > MAX_DOWNLOAD_BYTES {
        return Err("Berkas unduhan terlalu besar".to_string());
    }

    let part_path = destination.with_extension("part");
    let mut file = std::fs::File::create(&part_path)
        .map_err(|e| format!("Gagal membuat berkas penyimpanan: {}", e))?;

    let mut downloaded: usize = 0;

    loop {
        match resp.chunk().await {
            Ok(Some(chunk)) => {
                downloaded += chunk.len();
                if downloaded > MAX_DOWNLOAD_BYTES {
                    drop(file);
                    let _ = std::fs::remove_file(&part_path);
                    return Err("Berkas unduhan melebihi batas ukuran".to_string());
                }
                file.write_all(&chunk)
                    .map_err(|e| format!("Gagal menulis data ke disk: {}", e))?;
                if total_size > 0 {
                    let pct = (downloaded as f32 / total_size as f32).clamp(0.0, 1.0);
                    on_progress(pct);
                } else {
                    let est_pct = ((downloaded as f32 / (downloaded as f32 + 500_000.0)) * 0.9).min(0.95);
                    on_progress(est_pct);
                }
            }
            Ok(None) => break,
            Err(e) => {
                drop(file);
                let _ = std::fs::remove_file(&part_path);
                return Err(format!("Gagal membaca paket data: {e}"));
            }
        }
    }

    file.flush()
        .map_err(|e| format!("Gagal memfinalisasi unduhan: {e}"))?;
    drop(file);
    std::fs::rename(&part_path, destination)
        .map_err(|e| format!("Gagal menyimpan berkas unduhan: {e}"))?;
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

    let app_dir = crate::db::Database::get_app_dir()?;
    let books_dir = app_dir.join("books");
    std::fs::create_dir_all(&books_dir)
        .map_err(|e| format!("Gagal menyiapkan folder buku: {e}"))?;
    let clean_id = id.replace(|c: char| !c.is_alphanumeric(), "_");
    let ext = if fmt == "pdf" { "pdf" } else { "epub" };
    let dest_path = books_dir.join(format!("{}-{}.{}", clean_id, &uuid::Uuid::new_v4().to_string()[..8], ext));

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
    } else {
        let parsed = match crate::engine::parse_epub(&dest_path) {
            Ok(p) if !p.chapters.is_empty() => p,
            Ok(_) => {
                let _ = std::fs::remove_file(&dest_path);
                return Err("Berkas EPUB tidak memiliki bab yang bisa dibaca".to_string());
            }
            Err(e) => {
                let _ = std::fs::remove_file(&dest_path);
                return Err(format!("Berkas EPUB tidak valid: {e}"));
            }
        };
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
    };

    Ok(book)
}
