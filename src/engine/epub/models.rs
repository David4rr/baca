#[derive(Debug, Clone, PartialEq)]
pub struct TocItem {
    pub title: String,
    pub chapter_index: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chapter {
    pub index: usize,
    pub title: String,
    pub html_content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EpubBook {
    pub title: String,
    pub author: String,
    pub cover_image: Option<String>,
    pub toc: Vec<TocItem>,
    pub chapters: Vec<Chapter>,
}
