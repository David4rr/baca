use super::models::{detect_formats, extract_string, ArchiveBook};
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;

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
    subjects: Option<Vec<String>>,
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
    format: Option<serde_json::Value>,
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
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) Baca/3.3.0")
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
                        let desc = b.summaries.and_then(|s| s.into_iter().next())
                            .or_else(|| b.subjects.as_ref().map(|s| s.iter().take(3).cloned().collect::<Vec<_>>().join(" • ")));
                        let id = format!(
                            "gutenberg:{}|{}",
                            epub_url.as_deref().unwrap_or(""),
                            pdf_url.as_deref().unwrap_or("")
                        );

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
        "https://archive.org/advancedsearch.php?q={}&fl[]=identifier,title,creator,downloads,description,format&sort[]=downloads+desc&rows=15&page={}&output=json",
        urlencoding::encode(&archive_query),
        page
    );

    if let Ok(resp) = client
        .get(&archive_url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) Baca/3.3.0")
        .send()
        .await
    {
        if resp.status().is_success() {
            if let Ok(search_res) = resp.json::<ArchiveSearchResponse>().await {
                for raw in search_res.response.docs {
                    let title = extract_string(raw.title).unwrap_or_else(|| raw.identifier.clone());
                    let creator = extract_string(raw.creator);
                    let description = extract_string(raw.description);
                    let (has_epub, has_pdf) = detect_formats(raw.format.as_ref());
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
                        has_epub,
                        has_pdf,
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
