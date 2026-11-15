use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchiveBook {
    pub identifier: String,
    pub title: String,
    pub creator: Option<String>,
    pub downloads: Option<u64>,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub direct_epub_url: Option<String>,
    pub direct_pdf_url: Option<String>,
    pub has_epub: bool,
    pub has_pdf: bool,
}

impl ArchiveBook {
    pub fn thumbnail_url(&self) -> String {
        if let Some(cover) = &self.cover_url {
            if !cover.is_empty() {
                return cover.clone();
            }
        }
        format!("https://archive.org/services/img/{}", self.identifier)
    }

    pub fn author_display(&self) -> String {
        self.creator
            .clone()
            .unwrap_or_else(|| "Pengarang Tidak Dikenal".to_string())
    }
}

#[derive(Debug, Clone, Deserialize)]
struct GutendexResponse {
    results: Vec<GutendexBook>,
}

#[derive(Debug, Clone, Deserialize)]
struct GutendexBook {
    title: String,
    authors: Vec<GutendexAuthor>,
    formats: HashMap<String, String>,
    download_count: Option<u64>,
    summaries: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
struct GutendexAuthor {
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ArchiveSearchResponse {
    response: ArchiveSearchInner,
}

#[derive(Debug, Clone, Deserialize)]
struct ArchiveSearchInner {
    docs: Vec<ArchiveBookRaw>,
}

#[derive(Debug, Clone, Deserialize)]
struct ArchiveBookRaw {
    identifier: String,
    title: Option<serde_json::Value>,
    creator: Option<serde_json::Value>,
    downloads: Option<u64>,
    description: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
struct ArchiveMetadata {
    files: Option<Vec<ArchiveFile>>,
}

#[derive(Debug, Clone, Deserialize)]
struct ArchiveFile {
    name: Option<String>,
}

fn extract_string(val: Option<serde_json::Value>) -> Option<String> {
    match val {
        Some(serde_json::Value::String(s)) => Some(s),
        Some(serde_json::Value::Array(arr)) => {
            let parts: Vec<String> = arr
                .into_iter()
                .filter_map(|item| {
                    if let serde_json::Value::String(s) = item {
                        Some(s)
                    } else {
                        None
                    }
                })
                .collect();
            if parts.is_empty() {
                None
            } else {
                Some(parts.join(", "))
            }
        }
        _ => None,
    }
}

pub async fn search_archive(query: &str, page: u32) -> Result<Vec<ArchiveBook>, String> {
    let clean_query = query.trim();
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(12))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .map_err(|e| e.to_string())?;

    let mut books = Vec::new();

    // 1. Search Project Gutenberg via Gutendex
    let guten_query = if clean_query.is_empty() { "classics" } else { clean_query };
    let guten_url = format!(
        "https://gutendex.com/books/?search={}&page={}",
        urlencoding::encode(guten_query),
        page
    );

    if let Ok(resp) = client
        .get(&guten_url)
        .header("User-Agent", "BacaReader/3.2.0")
        .send()
        .await
    {
        if resp.status().is_success() {
            if let Ok(parsed) = resp.json::<GutendexResponse>().await {
                for b in parsed.results {
                    let epub_url = b.formats.get("application/epub+zip").cloned();
                    let pdf_url = b.formats.get("application/pdf").cloned();
                    let has_epub = epub_url.is_some();
                    let has_pdf = pdf_url.is_some();

                    if has_epub || has_pdf {
                        let cover_url = b.formats.get("image/jpeg").cloned();
                        let author = b.authors.first().map(|a| a.name.clone());
                        let desc = b.summaries.and_then(|s| s.into_iter().next());

                        let id = if let Some(e) = &epub_url {
                            format!("gutenberg:{}", e)
                        } else {
                            format!("gutenberg:{}", pdf_url.as_deref().unwrap_or_default())
                        };

                        books.push(ArchiveBook {
                            identifier: id,
                            title: b.title,
                            creator: author,
                            downloads: b.download_count,
                            description: desc,
                            cover_url,
                            direct_epub_url: epub_url,
                            direct_pdf_url: pdf_url,
                            has_epub,
                            has_pdf,
                        });
                    }
                }
            }
        }
    }

    // 2. Search Archive.org with EPUB and PDF filters
    let archive_query = if clean_query.is_empty() {
        "mediatype:(texts) AND (format:(EPUB) OR format:(PDF) OR format:(\"Text PDF\"))".to_string()
    } else {
        format!("({}) AND mediatype:(texts) AND (format:(EPUB) OR format:(PDF) OR format:(\"Text PDF\"))", clean_query)
    };

    let archive_url = format!(
        "https://archive.org/advancedsearch.php?q={}&fl[]=identifier,title,creator,downloads,description&sort[]=downloads+desc&rows=15&page={}&output=json",
        urlencoding::encode(&archive_query),
        page
    );

    if let Ok(resp) = client
        .get(&archive_url)
        .header("User-Agent", "BacaReader/3.2.0")
        .send()
        .await
    {
        if resp.status().is_success() {
            if let Ok(search_res) = resp.json::<ArchiveSearchResponse>().await {
                for raw in search_res.response.docs {
                    let title = extract_string(raw.title).unwrap_or_else(|| raw.identifier.clone());
                    let creator = extract_string(raw.creator);
                    let description = extract_string(raw.description);
                    let id = raw.identifier;

                    books.push(ArchiveBook {
                        cover_url: Some(format!("https://archive.org/services/img/{}", id)),
                        direct_epub_url: None,
                        direct_pdf_url: None,
                        identifier: id,
                        title,
                        creator,
                        downloads: raw.downloads,
                        description,
                        has_epub: true,
                        has_pdf: true,
                    });
                }
            }
        }
    }

    if books.is_empty() && !clean_query.is_empty() {
        return Err(format!("Tidak ada buku ditemukan untuk pencarian \"{}\".", clean_query));
    }

    Ok(books)
}

pub async fn get_archive_format_file(identifier: &str, format_kind: &str) -> Result<Option<String>, String> {
    let is_pdf = format_kind.to_lowercase() == "pdf";

    if identifier.starts_with("http://") || identifier.starts_with("https://") {
        return Ok(Some(identifier.to_string()));
    }
    if let Some(direct) = identifier.strip_prefix("gutenberg:") {
        if is_pdf {
            if direct.ends_with(".pdf") {
                return Ok(Some(direct.to_string()));
            }
        } else {
            return Ok(Some(direct.to_string()));
        }
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(12))
        .build()
        .map_err(|e| e.to_string())?;

    let url = format!("https://archive.org/metadata/{}", identifier);
    let resp = client
        .get(&url)
        .header("User-Agent", "BacaReader/3.2.0")
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
                if is_pdf && lower.ends_with(".pdf") {
                    return Ok(Some(name));
                } else if !is_pdf && lower.ends_with(".epub") {
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

pub async fn download_archive_book(
    identifier: &str,
    filename: &str,
    destination: &Path,
) -> Result<(), String> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| e.to_string())?;

    let file_url = if filename.starts_with("http://") || filename.starts_with("https://") {
        filename.to_string()
    } else {
        format!(
            "https://archive.org/download/{}/{}",
            identifier,
            urlencoding::encode(filename)
        )
    };

    let resp = client
        .get(&file_url)
        .header("User-Agent", "BacaReader/3.2.0")
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Download failed with status: {}", resp.status()));
    }

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("Failed to read file bytes: {}", e))?;

    if let Some(parent) = destination.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    std::fs::write(destination, &bytes).map_err(|e| format!("Failed to save file: {}", e))?;

    Ok(())
}
