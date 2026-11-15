use baca::engine::archive::search_archive;

#[tokio::test]
async fn test_online_search_finds_books() {
    let result = search_archive("sherlock", 1).await;
    assert!(result.is_ok(), "Online book search must succeed");
    let books = result.unwrap();
    assert!(!books.is_empty(), "Should find at least one book for 'sherlock'");
    let first = &books[0];
    assert!(!first.title.is_empty(), "Book title must not be empty");
    assert!(!first.identifier.is_empty(), "Book identifier must not be empty");
}
