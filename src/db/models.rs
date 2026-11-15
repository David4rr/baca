use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub file_path: String,
    pub format: String, // "epub" or "pdf"
    pub cover_image: Option<String>, // Base64 or local path
    pub total_chapters: usize,
    pub current_chapter: usize,
    pub progress_percent: f32,
    pub last_read_at: i64,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub book_id: String,
    pub chapter_index: usize,
    pub chapter_title: String,
    pub text_snippet: String,
    pub note: Option<String>,
    pub color_tag: String, // "teal", "amber", "violet", "rose"
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSettings {
    pub theme: String, // "amoled", "warm", "slate", "light"
    pub font_family: String, // "serif", "sans", "mono"
    pub font_size: u32, // 14 to 32
    pub line_height: f32, // 1.4 to 2.2
    pub margin_horizontal: u32, // 16 to 80
    pub gemini_api_key: Option<String>,
    pub gemini_model: Option<String>,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            theme: "amoled".to_string(),
            font_family: "serif".to_string(),
            font_size: 18,
            line_height: 1.7,
            margin_horizontal: 32,
            gemini_api_key: None,
            gemini_model: Some("gemini-2.5-flash".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadingSession {
    pub id: String,
    pub book_id: String,
    pub duration_seconds: u32,
    pub timestamp: i64,
    pub date_str: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyReadingStats {
    pub date_str: String,
    pub minutes: u32,
    pub day_label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackupData {
    pub version: String,
    pub exported_at: i64,
    pub books: Vec<Book>,
    pub bookmarks: Vec<Bookmark>,
    pub settings: UserSettings,
    pub sessions: Vec<ReadingSession>,
}
