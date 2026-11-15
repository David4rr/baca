use std::fs::File;
use std::io::Read;
use zip::ZipArchive;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub fn normalize_zip_path(base_dir: &str, relative_path: &str) -> String {
    let combined = if base_dir.is_empty() {
        relative_path.to_string()
    } else {
        format!("{}/{}", base_dir.trim_end_matches('/'), relative_path.trim_start_matches('/'))
    };

    let mut segments = Vec::new();
    for part in combined.split('/') {
        if part.is_empty() || part == "." {
            continue;
        } else if part == ".." {
            segments.pop();
        } else {
            segments.push(part);
        }
    }
    segments.join("/")
}

pub fn read_zip_entry_as_base64(archive: &mut ZipArchive<File>, entry_name: &str, media_type: &str) -> Option<String> {
    if let Ok(mut entry) = archive.by_name(entry_name) {
        let mut buffer = Vec::new();
        if entry.read_to_end(&mut buffer).is_ok() {
            let b64 = BASE64.encode(&buffer);
            return Some(format!("data:{};base64,{}", media_type, b64));
        }
    }
    // Case-insensitive fallback
    let target_lower = entry_name.to_lowercase();
    let count = archive.len();
    for i in 0..count {
        if let Ok(mut entry) = archive.by_index(i) {
            if entry.name().to_lowercase() == target_lower {
                let mut buffer = Vec::new();
                if entry.read_to_end(&mut buffer).is_ok() {
                    let b64 = BASE64.encode(&buffer);
                    return Some(format!("data:{};base64,{}", media_type, b64));
                }
            }
        }
    }
    None
}

pub fn guess_image_mime(path: &str) -> &'static str {
    let lower = path.to_lowercase();
    if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else if lower.ends_with(".svg") {
        "image/svg+xml"
    } else {
        "image/jpeg"
    }
}

pub fn embed_images_into_html(
    html: &str,
    chapter_dir: &str,
    archive: &mut ZipArchive<File>,
) -> String {
    let mut result = String::with_capacity(html.len());
    let mut cursor = 0;

    while let Some(tag_start) = html[cursor..].find("<img") {
        let actual_start = cursor + tag_start;
        result.push_str(&html[cursor..actual_start]);

        if let Some(tag_end) = html[actual_start..].find('>') {
            let full_tag = &html[actual_start..actual_start + tag_end + 1];
            let modified = replace_src_with_base64(full_tag, chapter_dir, archive);
            result.push_str(&modified);
            cursor = actual_start + tag_end + 1;
        } else {
            result.push_str(&html[actual_start..]);
            cursor = html.len();
            break;
        }
    }
    result.push_str(&html[cursor..]);
    result
}

fn replace_src_with_base64(img_tag: &str, chapter_dir: &str, archive: &mut ZipArchive<File>) -> String {
    for attr in ["src=\"", "src='", "xlink:href=\"", "href=\""] {
        if let Some(pos) = img_tag.find(attr) {
            let quote = if attr.contains('\'') { '\'' } else { '\"' };
            let val_start = pos + attr.len();
            if let Some(val_len) = img_tag[val_start..].find(quote) {
                let src_val = &img_tag[val_start..val_start + val_len];
                if src_val.starts_with("data:") || src_val.starts_with("http") {
                    return img_tag.to_string();
                }
                let zip_img_path = normalize_zip_path(chapter_dir, src_val);
                let mime = guess_image_mime(&zip_img_path);
                if let Some(b64_uri) = read_zip_entry_as_base64(archive, &zip_img_path, mime) {
                    let before = &img_tag[..val_start];
                    let after = &img_tag[val_start + val_len..];
                    return format!("{before}{b64_uri}{after}");
                }
            }
        }
    }
    img_tag.to_string()
}
