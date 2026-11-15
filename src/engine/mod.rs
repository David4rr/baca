pub mod archive;
pub mod epub;
pub mod gemini;
pub mod sanitizer;
pub mod starter_books;
pub mod starter_covers;

pub use archive::{download_archive_book, get_archive_epub_file, search_archive, ArchiveBook};
pub use epub::{parse_epub, Chapter, EpubBook, TocItem};
pub use gemini::ask_gemini;
pub use sanitizer::sanitize_html;
pub use starter_books::{ensure_starter_library, get_starter_book_epub};
pub use starter_covers::generate_editorial_cover;
