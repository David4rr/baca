use std::io::Write;
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;
use crate::db::models::Book;
use crate::engine::starter_covers::generate_editorial_cover;

pub fn ensure_starter_library(app_dir: &Path) -> Vec<Book> {
    let starter_dir = app_dir.join("starter_library");
    let _ = std::fs::create_dir_all(&starter_dir);
    let now = chrono::Utc::now().timestamp();

    let p1 = starter_dir.join("psychology_of_money.epub");
    let cover_1 = generate_editorial_cover("The Psychology\nof Money", "MORGAN HOUSEL", "Timeless lessons on wealth & greed", "psychology");

    let p2 = starter_dir.join("atomic_habits.epub");
    let cover_2 = generate_editorial_cover("Atomic Habits", "JAMES CLEAR", "An Easy & Proven Way to Build Good Habits", "atomic");

    let p3 = starter_dir.join("sapiens.epub");
    let cover_3 = generate_editorial_cover("Sapiens", "YUVAL NOAH HARARI", "A Brief History of Humankind", "sapiens");

    let p4 = starter_dir.join("midnight_library.epub");
    let cover_4 = generate_editorial_cover("The Midnight\nLibrary", "MATT HAIG", "Between life and death there is a library", "midnight");

    vec![
        Book {
            id: "starter-psychology".to_string(),
            title: "The Psychology of Money".to_string(),
            author: "Morgan Housel".to_string(),
            file_path: p1.to_string_lossy().to_string(),
            format: "epub".to_string(),
            cover_image: Some(cover_1),
            total_chapters: 6,
            current_chapter: 4,
            progress_percent: 0.65,
            last_read_at: now,
            created_at: now - 3600,
        },
        Book {
            id: "starter-atomic".to_string(),
            title: "Atomic Habits".to_string(),
            author: "James Clear".to_string(),
            file_path: p2.to_string_lossy().to_string(),
            format: "epub".to_string(),
            cover_image: Some(cover_2),
            total_chapters: 3,
            current_chapter: 1,
            progress_percent: 0.38,
            last_read_at: now - 86400,
            created_at: now - 86400 * 2,
        },
        Book {
            id: "starter-sapiens".to_string(),
            title: "Sapiens".to_string(),
            author: "Yuval Noah Harari".to_string(),
            file_path: p3.to_string_lossy().to_string(),
            format: "epub".to_string(),
            cover_image: Some(cover_3),
            total_chapters: 3,
            current_chapter: 2,
            progress_percent: 1.0,
            last_read_at: now - 86400 * 3,
            created_at: now - 86400 * 5,
        },
        Book {
            id: "starter-midnight".to_string(),
            title: "The Midnight Library".to_string(),
            author: "Matt Haig".to_string(),
            file_path: p4.to_string_lossy().to_string(),
            format: "epub".to_string(),
            cover_image: Some(cover_4),
            total_chapters: 3,
            current_chapter: 0,
            progress_percent: 0.27,
            last_read_at: now - 86400 * 4,
            created_at: now - 86400 * 6,
        },
    ]
}

pub fn create_minimal_epub(path: &Path, title: &str, author: &str, chapters: &[(&str, &str)]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut zip = ZipWriter::new(file);
    let opt = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    zip.start_file("mimetype", opt).map_err(|e| e.to_string())?;
    zip.write_all(b"application/epub+zip").map_err(|e| e.to_string())?;

    zip.start_file("META-INF/container.xml", opt).map_err(|e| e.to_string())?;
    zip.write_all(br#"<?xml version="1.0"?><container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container"><rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles></container>"#).map_err(|e| e.to_string())?;

    let mut manifest = String::new();
    let mut spine = String::new();
    for (i, _) in chapters.iter().enumerate() {
        manifest.push_str(&format!(r#"<item id="ch{}" href="ch{}.xhtml" media-type="application/xhtml+xml"/>"#, i, i));
        spine.push_str(&format!(r#"<itemref idref="ch{}"/>"#, i));
    }

    let opf = format!(
        r#"<?xml version="1.0" encoding="utf-8"?><package version="2.0" xmlns="http://www.idpf.org/2007/opf" unique-identifier="BookId"><metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:title>{}</dc:title><dc:creator>{}</dc:creator></metadata><manifest>{}</manifest><spine>{}</spine></package>"#,
        title, author, manifest, spine
    );

    zip.start_file("OEBPS/content.opf", opt).map_err(|e| e.to_string())?;
    zip.write_all(opf.as_bytes()).map_err(|e| e.to_string())?;

    for (i, (ch_title, ch_body)) in chapters.iter().enumerate() {
        let xhtml = format!(r#"<!DOCTYPE html><html><head><title>{}</title></head><body><h1>{}</h1>{}</body></html>"#, ch_title, ch_title, ch_body);
        zip.start_file(format!("OEBPS/ch{}.xhtml", i), opt).map_err(|e| e.to_string())?;
        zip.write_all(xhtml.as_bytes()).map_err(|e| e.to_string())?;
    }

    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}
