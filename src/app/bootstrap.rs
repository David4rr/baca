use crate::db::models::Book;
use crate::db::Database;
use std::sync::Arc;

/// Loads the library. Only rows created by the retired demo seeder are removed;
/// user books are never touched, regardless of title.
pub fn bootstrap_library(db: &Arc<Database>) -> Vec<Book> {
    let existing = db.get_all_books().unwrap_or_default();

    let mut user_books = Vec::new();
    for b in existing {
        if b.id.starts_with("starter-book-") {
            let _ = db.delete_book(&b.id);
        } else {
            user_books.push(b);
        }
    }

    user_books
}
