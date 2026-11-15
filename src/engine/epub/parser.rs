use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

use super::images::{embed_images_into_html, normalize_zip_path, read_zip_entry_as_base64};
use super::models::{Chapter, EpubBook, TocItem};
use crate::engine::sanitizer::sanitize_html;

pub fn parse_epub<P: AsRef<Path>>(path: P) -> Result<EpubBook, String> {
    let file = File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Invalid ZIP/EPUB: {}", e))?;

    let opf_path = get_opf_path(&mut archive)?;
    let opf_dir = Path::new(&opf_path).parent().unwrap_or(Path::new("")).to_str().unwrap_or("").to_string();

    let opf_content = read_zip_entry_to_string(&mut archive, &opf_path)?;
    let (title, author, cover_id) = parse_opf_metadata(&opf_content);
    let manifest = parse_opf_manifest(&opf_content);
    let spine = parse_opf_spine(&opf_content);

    let cover_image = extract_cover_image(&mut archive, &opf_dir, &manifest, cover_id.as_deref());

    let mut chapters = Vec::new();
    let mut toc = Vec::new();

    for (idx, item_id) in spine.iter().enumerate() {
        if let Some((href, _)) = manifest.get(item_id) {
            let full_href = normalize_zip_path(&opf_dir, href);
            let chapter_dir = Path::new(&full_href).parent().unwrap_or(Path::new("")).to_str().unwrap_or("").to_string();

            if let Ok(raw_html) = read_zip_entry_to_string(&mut archive, &full_href) {
                // 1. Embed images directly as Base64 Data URIs
                let html_with_images = embed_images_into_html(&raw_html, &chapter_dir, &mut archive);
                // 2. Sanitize scripts, iframes while preserving safe typography & images
                let sanitized = sanitize_html(&html_with_images);
                let chapter_title = extract_title_or_heading(&raw_html, idx + 1);

                toc.push(TocItem {
                    title: chapter_title.clone(),
                    chapter_index: idx,
                });

                chapters.push(Chapter {
                    index: idx,
                    title: chapter_title,
                    html_content: sanitized,
                });
            }
        }
    }

    Ok(EpubBook {
        title,
        author,
        cover_image,
        toc,
        chapters,
    })
}

fn get_opf_path(archive: &mut ZipArchive<File>) -> Result<String, String> {
    let container_xml = read_zip_entry_to_string(archive, "META-INF/container.xml")?;
    for line in container_xml.lines() {
        if line.contains("full-path=") {
            if let Some(start) = line.find("full-path=\"") {
                let rest = &line[start + 11..];
                if let Some(end) = rest.find('\"') {
                    return Ok(rest[..end].to_string());
                }
            }
        }
    }
    Err("Could not locate rootfile OPF path in container.xml".to_string())
}

fn parse_opf_metadata(opf: &str) -> (String, String, Option<String>) {
    let mut title = "Untitled Book".to_string();
    let mut author = "Unknown Author".to_string();
    let mut cover_id = None;

    for line in opf.lines() {
        if (line.contains("<dc:title") || line.contains("<title")) && line.contains("</") {
            if let (Some(s), Some(e)) = (line.find('>'), line.rfind('<')) {
                if s < e { title = line[s + 1..e].trim().to_string(); }
            }
        }
        if (line.contains("<dc:creator") || line.contains("<creator")) && line.contains("</") {
            if let (Some(s), Some(e)) = (line.find('>'), line.rfind('<')) {
                if s < e { author = line[s + 1..e].trim().to_string(); }
            }
        }
        if line.contains("name=\"cover\"") && line.contains("content=\"") {
            if let Some(idx) = line.find("content=\"") {
                let rest = &line[idx + 9..];
                if let Some(end) = rest.find('\"') {
                    cover_id = Some(rest[..end].to_string());
                }
            }
        }
    }
    (title, author, cover_id)
}

fn parse_opf_manifest(opf: &str) -> HashMap<String, (String, String)> {
    let mut manifest = HashMap::new();
    for line in opf.lines() {
        if line.contains("<item ") {
            let id = extract_attr(line, "id");
            let href = extract_attr(line, "href");
            let media_type = extract_attr(line, "media-type");
            if let (Some(id_val), Some(href_val), Some(media_val)) = (id, href, media_type) {
                manifest.insert(id_val, (href_val, media_val));
            }
        }
    }
    manifest
}

fn parse_opf_spine(opf: &str) -> Vec<String> {
    let mut spine = Vec::new();
    for line in opf.lines() {
        if line.contains("<itemref ") {
            if let Some(idref) = extract_attr(line, "idref") {
                spine.push(idref);
            }
        }
    }
    spine
}

fn extract_cover_image(
    archive: &mut ZipArchive<File>,
    opf_dir: &str,
    manifest: &HashMap<String, (String, String)>,
    cover_id: Option<&str>,
) -> Option<String> {
    if let Some(cid) = cover_id {
        if let Some((href, media_type)) = manifest.get(cid) {
            let full_href = normalize_zip_path(opf_dir, href);
            return read_zip_entry_as_base64(archive, &full_href, media_type);
        }
    }
    for (id, (href, media_type)) in manifest {
        if (id.to_lowercase().contains("cover") || href.to_lowercase().contains("cover"))
            && media_type.starts_with("image/") {
            let full_href = normalize_zip_path(opf_dir, href);
            if let Some(img) = read_zip_entry_as_base64(archive, &full_href, media_type) {
                return Some(img);
            }
        }
    }
    None
}

fn extract_attr(line: &str, attr_name: &str) -> Option<String> {
    let target = format!("{}=\"", attr_name);
    if let Some(start) = line.find(&target) {
        let rest = &line[start + target.len()..];
        if let Some(end) = rest.find('\"') {
            return Some(rest[..end].to_string());
        }
    }
    None
}

fn read_zip_entry_to_string(archive: &mut ZipArchive<File>, entry_name: &str) -> Result<String, String> {
    let mut entry = archive.by_name(entry_name).map_err(|e| format!("Zip entry '{}' tidak ditemukan: {}", entry_name, e))?;
    let mut buffer = Vec::new();
    entry.read_to_end(&mut buffer).map_err(|e| format!("Gagal membaca '{}': {}", entry_name, e))?;
    Ok(String::from_utf8_lossy(&buffer).to_string())
}

fn extract_title_or_heading(html: &str, default_index: usize) -> String {
    for tag in ["<h1", "<h2", "<h3", "<title"] {
        if let Some(start) = html.find(tag) {
            let rest = &html[start..];
            if let Some(close_tag) = rest.find('>') {
                let inner = &rest[close_tag + 1..];
                if let Some(end_tag) = inner.find('<') {
                    let heading = inner[..end_tag].trim();
                    if !heading.is_empty() {
                        return heading.to_string();
                    }
                }
            }
        }
    }
    format!("Chapter {}", default_index)
}
