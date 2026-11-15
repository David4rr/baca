use crate::db::models::Book;
use crate::db::Database;
use crate::engine::archive::{download_format_file, ArchiveBook};

pub async fn download_format_to_library(
    db: &Database,
    archive_book: &ArchiveBook,
    format_str: &str,
) -> Result<Book, String> {
    let book = download_format_file(archive_book, format_str, |_| {}).await?;
    db.insert_or_update_book(&book)
        .map_err(|e| format!("Gagal menyimpan metadata buku: {}", e))?;
    Ok(book)
}
