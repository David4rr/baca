pub mod download;
pub mod models;
pub mod search;

pub use download::{
    download_archive_book, download_archive_book_with_progress, download_format_file,
    get_archive_epub_file, get_archive_format_file,
};
pub use models::{detect_formats, extract_string, ArchiveBook};
pub use search::search_archive;
