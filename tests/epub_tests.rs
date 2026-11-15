use baca::engine::parse_epub;
use std::io::Write;
use tempfile::NamedTempFile;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

#[test]
fn test_parse_valid_epub() {
    let mut temp = NamedTempFile::new().expect("Should create temp file");
    {
        let mut zip = ZipWriter::new(&mut temp);
        let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

        // 1. mimetype
        zip.start_file("mimetype", options).unwrap();
        zip.write_all(b"application/epub+zip").unwrap();

        // 2. META-INF/container.xml
        zip.start_file("META-INF/container.xml", options).unwrap();
        zip.write_all(br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles>
    <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
  </rootfiles>
</container>"#).unwrap();

        // 3. OEBPS/content.opf
        zip.start_file("OEBPS/content.opf", options).unwrap();
        zip.write_all(br#"<?xml version="1.0" encoding="utf-8"?>
<package version="2.0" xmlns="http://www.idpf.org/2007/opf" unique-identifier="BookId">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
    <dc:title>Test Book Title</dc:title>
    <dc:creator>Test Author</dc:creator>
  </metadata>
  <manifest>
    <item id="ch1" href="ch1.xhtml" media-type="application/xhtml+xml"/>
  </manifest>
  <spine>
    <itemref idref="ch1"/>
  </spine>
</package>"#).unwrap();

        // 4. OEBPS/ch1.xhtml
        zip.start_file("OEBPS/ch1.xhtml", options).unwrap();
        zip.write_all(br#"<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE html>
<html>
  <head><title>Chapter 1</title></head>
  <body>
    <h1>Chapter 1: The Beginning</h1>
    <p>It was a dark and stormy night in the world of systems programming.</p>
  </body>
</html>"#).unwrap();

        zip.finish().unwrap();
    }

    let parsed = parse_epub(temp.path()).expect("Should parse synthetic EPUB cleanly");
    assert_eq!(parsed.title, "Test Book Title");
    assert_eq!(parsed.author, "Test Author");
    assert_eq!(parsed.chapters.len(), 1);
    assert_eq!(parsed.chapters[0].title, "Chapter 1: The Beginning");
    assert!(parsed.chapters[0].html_content.contains("dark and stormy night"));
    assert_eq!(parsed.toc.len(), 1);
    assert_eq!(parsed.toc[0].title, "Chapter 1: The Beginning");
}
