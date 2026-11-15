pub mod archive;
pub mod epub;
pub mod gemini;
pub mod sanitizer;
pub mod starter_books;
pub mod starter_covers;
pub mod updater;

pub use archive::{download_archive_book, download_archive_book_with_progress, get_archive_epub_file, get_archive_format_file, search_archive, ArchiveBook};
pub use epub::{parse_epub, Chapter, EpubBook, TocItem};
pub use gemini::ask_gemini;
pub use sanitizer::sanitize_html;
pub use starter_books::{ensure_starter_library, get_starter_book_epub};
pub use starter_covers::generate_editorial_cover;
pub use updater::{check_latest_release, download_update_package, is_newer_version, ReleaseInfo};
