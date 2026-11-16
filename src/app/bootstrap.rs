use crate::db::models::Book;
use crate::db::Database;
use std::sync::Arc;

pub fn bootstrap_library(db: &Arc<Database>) -> Vec<Book> {
    let existing = db.get_all_books().unwrap_or_default();
    
    // Purge unwanted starter books from previously seeded libraries
    let starter_titles = [
        "atomic habits",
        "the psychology of money",
        "sapiens",
        "the midnight library",
    ];

    let mut user_books = Vec::new();
    for b in existing {
        let is_starter = b.id.starts_with("starter-book-") 
            || starter_titles.iter().any(|t| b.title.to_lowercase().contains(t));
        if is_starter {
            let _ = db.delete_book(&b.id);
        } else {
            user_books.push(b);
        }
    }

    user_books
}
