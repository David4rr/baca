use dioxus::prelude::*;
use crate::db::models::{Book, UserSettings};
use crate::engine::{ArchiveBook, EpubBook};
use crate::theme::ThemeMode;

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Shelf,
    Explore,
    Stats,
    Settings,
    Reader {
        book: Book,
        epub: Option<EpubBook>,
    },
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub current_view: Signal<View>,
    pub books: Signal<Vec<Book>>,
    pub search_query: Signal<String>,
    pub active_filter: Signal<String>,
    pub theme: Signal<ThemeMode>,
    pub settings: Signal<UserSettings>,
    pub current_chapter: Signal<usize>,
    pub is_toc_open: Signal<bool>,
    pub is_typography_open: Signal<bool>,
    pub is_inbook_search_open: Signal<bool>,
    pub is_ai_open: Signal<bool>,
    pub toast_message: Signal<Option<String>>,
    pub last_back_press: Signal<i64>,
    pub format_modal_book: Signal<Option<ArchiveBook>>,
    pub ai_term: Signal<String>,
    pub inbook_keyword: Signal<Option<String>>,
}

impl AppState {
    pub fn new(initial_books: Vec<Book>, initial_settings: UserSettings) -> Self {
        let initial_theme = ThemeMode::from_str(&initial_settings.theme);
        Self {
            current_view: Signal::new(View::Shelf),
            books: Signal::new(initial_books),
            search_query: Signal::new(String::new()),
            active_filter: Signal::new("all".to_string()),
            theme: Signal::new(initial_theme),
            settings: Signal::new(initial_settings),
            current_chapter: Signal::new(0),
            is_toc_open: Signal::new(false),
            is_typography_open: Signal::new(false),
            is_inbook_search_open: Signal::new(false),
            is_ai_open: Signal::new(false),
            toast_message: Signal::new(None),
            last_back_press: Signal::new(0),
            format_modal_book: Signal::new(None),
            ai_term: Signal::new(String::new()),
            inbook_keyword: Signal::new(None),
        }
    }
}
