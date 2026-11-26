#[derive(Debug, Clone, PartialEq)]
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

pub fn extract_string(val: Option<serde_json::Value>) -> Option<String> {
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

pub fn detect_formats(val: Option<&serde_json::Value>) -> (bool, bool) {
    let mut has_epub = false;
    let mut has_pdf = false;
    if let Some(v) = val {
        if let Some(s) = v.as_str() {
            let lower = s.to_lowercase();
            if lower.contains("epub") { has_epub = true; }
            if lower.contains("pdf") { has_pdf = true; }
        } else if let Some(arr) = v.as_array() {
            for item in arr {
                if let Some(s) = item.as_str() {
                    let lower = s.to_lowercase();
                    if lower.contains("epub") { has_epub = true; }
                    if lower.contains("pdf") { has_pdf = true; }
                }
            }
        }
    }
    (has_epub, has_pdf)
}
