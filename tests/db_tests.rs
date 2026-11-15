use baca::db::models::{Book, Bookmark};
use baca::db::Database;

#[test]
fn test_database_crud_operations() {
    let db = Database::memory().expect("In-memory DB should initialize cleanly");

    // 1. Initial state should be empty
    let books = db.get_all_books().expect("Should query books");
    assert_eq!(books.len(), 0);

    // 2. Insert book
    let test_book = Book {
        id: "book-123".to_string(),
        title: "The Rust Journey".to_string(),
        author: "Ferris".to_string(),
        file_path: "/tmp/rust.epub".to_string(),
        format: "epub".to_string(),
        cover_image: Some("data:image/png;base64,...".to_string()),
        total_chapters: 12,
        current_chapter: 0,
        progress_percent: 0.0,
        last_read_at: 1000,
        created_at: 1000,
    };
    db.insert_or_update_book(&test_book).expect("Should insert book");

    let books = db.get_all_books().expect("Should query books");
    assert_eq!(books.len(), 1);
    assert_eq!(books[0].title, "The Rust Journey");

    // 3. Update reading progress
    db.update_progress("book-123", 4, 0.33).expect("Should update progress");
    let books = db.get_all_books().expect("Should query books");
    assert_eq!(books[0].current_chapter, 4);
    assert!((books[0].progress_percent - 0.33).abs() < 0.001);

    // 4. Bookmarks
    let mark = Bookmark {
        id: "mark-1".to_string(),
        book_id: "book-123".to_string(),
        chapter_index: 4,
        chapter_title: "Chapter 4: Lifetimes".to_string(),
        text_snippet: "Borrow checker guarantees memory safety.".to_string(),
        note: Some("Crucial concept".to_string()),
        color_tag: "teal".to_string(),
        created_at: 1500,
    };
    db.add_bookmark(&mark).expect("Should add bookmark");

    let marks = db.get_bookmarks("book-123").expect("Should get bookmarks");
    assert_eq!(marks.len(), 1);
    assert_eq!(marks[0].text_snippet, "Borrow checker guarantees memory safety.");

    db.delete_bookmark("mark-1").expect("Should delete bookmark");
    let marks = db.get_bookmarks("book-123").expect("Should get bookmarks");
    assert_eq!(marks.len(), 0);

    // 5. Settings
    let mut settings = db.get_settings().expect("Should get default settings");
    assert_eq!(settings.theme, "amoled");

    settings.theme = "warm".to_string();
    settings.font_size = 22;
    db.save_settings(&settings).expect("Should save settings");

    let reloaded = db.get_settings().expect("Should reload settings");
    assert_eq!(reloaded.theme, "warm");
    assert_eq!(reloaded.font_size, 22);

    // 6. Delete book
    db.delete_book("book-123").expect("Should delete book");
    let books = db.get_all_books().expect("Should query books");
    assert_eq!(books.len(), 0);
}
