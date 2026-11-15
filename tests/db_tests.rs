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

    // 5. Reading Sessions & 7-Day Stats
    db.record_reading_session("book-123", 120).expect("Should record reading session");
    db.record_reading_session("book-123", 60).expect("Should record second session");
    let stats = db.get_7_day_stats().expect("Should get 7-day stats");
    assert_eq!(stats.len(), 7);
    let today_stats = stats.last().expect("Today stats exist");
    assert_eq!(today_stats.minutes, 3); // (120+60+30)/60 = 3

    // 6. Settings with Gemini API configuration
    let mut settings = db.get_settings().expect("Should get default settings");
    assert_eq!(settings.theme, "amoled");

    settings.theme = "warm".to_string();
    settings.font_size = 22;
    settings.gemini_api_key = Some("test-api-key-123".to_string());
    settings.gemini_model = Some("gemini-2.5-flash".to_string());
    db.save_settings(&settings).expect("Should save settings");

    let reloaded = db.get_settings().expect("Should reload settings");
    assert_eq!(reloaded.theme, "warm");
    assert_eq!(reloaded.font_size, 22);
    assert_eq!(reloaded.gemini_api_key.as_deref(), Some("test-api-key-123"));

    // 7. Backup and Restore
    let backup_data = db.export_backup().expect("Should export backup data");
    assert_eq!(backup_data.books.len(), 1);
    assert_eq!(backup_data.bookmarks.len(), 1);
    assert_eq!(backup_data.sessions.len(), 2);

    // Test restoring into fresh DB
    let fresh_db = Database::memory().expect("Fresh in-memory DB");
    fresh_db.restore_backup(backup_data).expect("Should restore backup");
    let restored_books = fresh_db.get_all_books().expect("Should query restored books");
    assert_eq!(restored_books.len(), 1);
    assert_eq!(restored_books[0].id, "book-123");

    // 8. Delete bookmark and book
    db.delete_bookmark("mark-1").expect("Should delete bookmark");
    let marks = db.get_bookmarks("book-123").expect("Should get bookmarks");
    assert_eq!(marks.len(), 0);

    db.delete_book("book-123").expect("Should delete book");
    let books = db.get_all_books().expect("Should query books");
    assert_eq!(books.len(), 0);
}
