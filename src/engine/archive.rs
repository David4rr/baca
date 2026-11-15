use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchiveBook {
    pub identifier: String,
    pub title: String,
    pub creator: Option<String>,
    pub downloads: Option<u64>,
    pub description: Option<String>,
}

impl ArchiveBook {
    pub fn thumbnail_url(&self) -> String {
        format!("https://archive.org/services/img/{}", self.identifier)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ArchiveSearchResponse {
    response: ArchiveSearchInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ArchiveSearchInner {
    #[serde(rename = "numFound")]
    num_found: u64,
    docs: Vec<ArchiveBookRaw>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ArchiveBookRaw {
    identifier: String,
    title: Option<serde_json::Value>,
    creator: Option<serde_json::Value>,
    downloads: Option<u64>,
    description: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ArchiveMetadata {
    files: Option<Vec<ArchiveFile>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ArchiveFile {
    name: Option<String>,
    format: Option<String>,
    size: Option<String>,
}

/// Helper to parse string or array of strings from Archive.org JSON
fn extract_string(val: Option<serde_json::Value>) -> Option<String> {
    match val {
        Some(serde_json::Value::String(s)) => Some(s),
        Some(serde_json::Value::Array(arr)) => {
            let mut parts = Vec::new();
            for item in arr {
                if let serde_json::Value::String(s) = item {
                    parts.push(s);
                }
            }
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
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?;

    let clean_query = query.trim();
    let q = if clean_query.is_empty() {
        "mediatype:texts".to_string()
    } else {
        format!("{}+mediatype:texts", clean_query)
    };

    let url = format!(
        "https://archive.org/advancedsearch.php?q={}&fl[]=identifier,title,creator,downloads,description&sort[]=downloads+desc&rows=20&page={}&output=json",
        urlencoding::encode(&q),
        page
    );

    let resp = client
        .get(&url)
        .header("User-Agent", "BacaReader/3.0.0")
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Archive.org returned status {}", resp.status()));
    }

    let search_res: ArchiveSearchResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let books = search_res
        .response
        .docs
        .into_iter()
        .map(|raw| {
            let title = extract_string(raw.title).unwrap_or_else(|| raw.identifier.clone());
            let creator = extract_string(raw.creator);
            let description = extract_string(raw.description);

            ArchiveBook {
                identifier: raw.identifier,
                title,
                creator,
                downloads: raw.downloads,
                description,
            }
        })
        .collect();

    Ok(books)
}

pub async fn get_archive_epub_file(identifier: &str) -> Result<Option<String>, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?;

    let url = format!("https://archive.org/metadata/{}", identifier);
    let resp = client
        .get(&url)
        .header("User-Agent", "BacaReader/3.0.0")
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
        // Look for .epub files
        for file in files {
            if let Some(name) = file.name {
                if name.to_lowercase().ends_with(".epub") {
                    return Ok(Some(name));
                }
            }
        }
    }

    Ok(None)
}

pub async fn download_archive_book(
    identifier: &str,
    filename: &str,
    destination: &Path,
) -> Result<(), String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| e.to_string())?;

    let file_url = format!(
        "https://archive.org/download/{}/{}",
        identifier,
        urlencoding::encode(filename)
    );

    let resp = client
        .get(&file_url)
        .header("User-Agent", "BacaReader/3.0.0")
        .send()
        .await
        .map_err(|e| format!("Failed to download file: {}", e))?;

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
