use dioxus::prelude::*;
use baca::engine::archive::{get_archive_format_file, search_archive};
use baca::state::{AppState, View};
use baca::db::models::UserSettings;

#[tokio::test]
async fn test_format_detection_and_query() {
    let results = search_archive("pride and prejudice", 1).await;
    assert!(results.is_ok(), "Search should succeed");
    let books = results.unwrap();
    assert!(!books.is_empty(), "Should find books for 'pride and prejudice'");

    let first = &books[0];
    assert!(first.has_epub || first.has_pdf, "Should detect at least one valid format (EPUB or PDF)");

    // Test format file resolver
    let epub_file = get_archive_format_file(&first.identifier, "epub").await;
    assert!(epub_file.is_ok(), "Resolving EPUB format should not error");
}

#[test]
fn test_app_state_navigation_and_modals() {
    let mut dom = VirtualDom::new(|| {
        let settings = UserSettings::default();
        let mut state = AppState::new(vec![], settings);

        // Initial view is Shelf
        assert_eq!(*state.current_view.read(), View::Shelf);

        // Navigation to Stats and Settings full pages
        *state.current_view.write() = View::Stats;
        assert_eq!(*state.current_view.read(), View::Stats);

        *state.current_view.write() = View::Settings;
        assert_eq!(*state.current_view.read(), View::Settings);

        // Toast message handling
        assert!(state.toast_message.read().is_none());
        *state.toast_message.write() = Some("Test Toast".to_string());
        assert_eq!(state.toast_message.read().as_deref(), Some("Test Toast"));

        // Format chooser modal state
        assert!(state.format_modal_book.read().is_none());

        // Bookmarks, in-book keyword, and delete confirm state
        assert_eq!(state.bookmarks.read().len(), 0);
        assert!(state.inbook_keyword.read().is_none());
        assert!(state.delete_confirm_book.read().is_none());

        rsx! {}
    });
    dom.rebuild_in_place();
}
