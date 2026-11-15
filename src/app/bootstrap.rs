use crate::db::models::Book;
use crate::db::Database;
use std::sync::Arc;

pub fn bootstrap_library(db: &Arc<Database>) -> Vec<Book> {
    let mut existing = db.get_all_books().unwrap_or_default();
    let app_dir = Database::get_app_dir();
    let starters = crate::engine::ensure_starter_library(&app_dir);

    if existing.is_empty() {
        for b in &starters {
            let _ = db.insert_or_update_book(b);
        }
        starters
    } else {
        for b in &mut existing {
            if b.cover_image.is_none() {
                if let Some(s) = starters.iter().find(|s| {
                    s.id == b.id || s.title.to_lowercase() == b.title.to_lowercase()
                }) {
                    b.cover_image = s.cover_image.clone();
                    let _ = db.insert_or_update_book(b);
                }
            }
        }
        existing
    }
}
