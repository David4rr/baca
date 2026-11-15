use std::io::Write;
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;
use crate::db::models::Book;
use crate::engine::{Chapter, EpubBook, TocItem};

pub fn get_starter_book_epub(path_or_id: &str) -> Option<EpubBook> {
    let lower = path_or_id.to_lowercase();
    if lower.contains("psychology") {
        let chapters_data = vec![
            ("No One's Crazy", "<p>Your personal experiences with money make up maybe 0.00000001% of what's happened in the world, but maybe 80% of how you think the world works.</p>"),
            ("Luck & Risk", "<p>Nothing is as good or as bad as it looks. Luck and risk are both the reality that every outcome in life is guided by forces other than individual effort alone.</p>"),
            ("Never Enough", "<p>When rich people do crazy things. There is no reason to risk what you have and need for what you don't have and don't need.</p>"),
            ("Confounding Compounding", "<p>More than 99% of Warren Buffett's wealth came after his 50th birthday. His skill is investing, but his secret is time.</p>"),
            ("The Wealth Equation", r#"<p><mark style="background-color: #FEF08A; padding: 2px 6px; border-radius: 4px; color: #1C1917;">Wealth is not about how much you make, but how much you keep, and how hard it works for you.</mark></p>
<p>Most people overestimate their earning power, and underestimate the power of compounding and patience.</p>
<p>The first rule of wealth: don't lose money. The second rule: don't forget the first rule.</p>
<p>Doing well with money has a little to do with how smart you are and a lot to do with how you behave. And behavior is hard to teach, even to really smart people.</p>"#),
            ("Freedom", "<p>The highest form of wealth is the ability to wake up every morning and say, 'I can do whatever I want today.'</p>"),
            ("Man in the Car Paradox", "<p>No one is impressed with your possessions as much as you are.</p>"),
            ("Save Money", "<p>You can build wealth without a high income, but you have no chance of building wealth without a high savings rate.</p>"),
            ("Reasonable > Rational", "<p>Do not aim to be coldly rational when making financial decisions. Aim to be pretty reasonable.</p>"),
            ("Surprise!", "<p>Things that have never happened before happen all the time.</p>"),
            ("Room for Error", "<p>The most important part of every plan is planning on your plan not going according to plan.</p>"),
            ("You'll Change", "<p>Long-term planning is harder than it seems because people's goals and desires change over time.</p>"),
        ];

        let mut chapters = Vec::new();
        let mut toc = Vec::new();
        for (i, (t, h)) in chapters_data.into_iter().enumerate() {
            toc.push(TocItem { title: t.to_string(), chapter_index: i });
            chapters.push(Chapter { index: i, title: t.to_string(), html_content: h.to_string() });
        }
        return Some(EpubBook {
            title: "The Psychology of Money".to_string(),
            author: "Morgan Housel".to_string(),
            cover_image: None,
            toc,
            chapters,
        });
    }

    if lower.contains("atomic") {
        let chapters_data = vec![
            ("The Surprising Power of Atomic Habits", "<p>Changes that seem small and unimportant at first will compound into remarkable results if you're willing to stick with them for years.</p>"),
            ("How Your Habits Shape Your Identity", "<p>Your habits are how you embody your identity. When you make your bed every day, you embody the identity of an organized person.</p>"),
            ("The 4 Laws of Behavior Change", "<p>Make it obvious. Make it attractive. Make it easy. Make it satisfying.</p>"),
        ];
        let mut chapters = Vec::new();
        let mut toc = Vec::new();
        for (i, (t, h)) in chapters_data.into_iter().enumerate() {
            toc.push(TocItem { title: t.to_string(), chapter_index: i });
            chapters.push(Chapter { index: i, title: t.to_string(), html_content: h.to_string() });
        }
        return Some(EpubBook {
            title: "Atomic Habits".to_string(),
            author: "James Clear".to_string(),
            cover_image: None,
            toc,
            chapters,
        });
    }

    if lower.contains("sapiens") {
        let chapters_data = vec![
            ("An Animal of No Significance", "<p>About 14 billion years ago, matter, energy, time and space came into being in what is known as the Big Bang.</p>"),
            ("The Tree of Knowledge", "<p>The Cognitive Revolution is the point when history declared its independence from biology.</p>"),
            ("A Day in the Life of Adam and Eve", "<p>To understand human nature, history and psychology, we must get into the minds of our hunter-gatherer ancestors.</p>"),
        ];
        let mut chapters = Vec::new();
        let mut toc = Vec::new();
        for (i, (t, h)) in chapters_data.into_iter().enumerate() {
            toc.push(TocItem { title: t.to_string(), chapter_index: i });
            chapters.push(Chapter { index: i, title: t.to_string(), html_content: h.to_string() });
        }
        return Some(EpubBook {
            title: "Sapiens".to_string(),
            author: "Yuval Noah Harari".to_string(),
            cover_image: None,
            toc,
            chapters,
        });
    }

    if lower.contains("midnight") {
        let chapters_data = vec![
            ("A Conversation About Rain", "<p>Nineteen years before she decided to die, Nora Seed sat in the warmth of the small library at Hazeldene School.</p>"),
            ("The Midnight Library", "<p>Between life and death there is a library, and within that library, the shelves go on for ever. Every book provides a chance to try another life you could have lived.</p>"),
            ("The Many Lives of Nora Seed", "<p>It is easy to mourn the lives we aren't living. Easy to wish we'd developed other talents, said other yeses.</p>"),
        ];
        let mut chapters = Vec::new();
        let mut toc = Vec::new();
        for (i, (t, h)) in chapters_data.into_iter().enumerate() {
            toc.push(TocItem { title: t.to_string(), chapter_index: i });
            chapters.push(Chapter { index: i, title: t.to_string(), html_content: h.to_string() });
        }
        return Some(EpubBook {
            title: "The Midnight Library".to_string(),
            author: "Matt Haig".to_string(),
            cover_image: None,
            toc,
            chapters,
        });
    }

    None
}

pub fn create_epub_file(
    path: &Path,
    title: &str,
    author: &str,
    chapters: &[(&str, &str)], // (chapter_title, chapter_html)
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    // 1. mimetype
    zip.start_file("mimetype", options).map_err(|e| e.to_string())?;
    zip.write_all(b"application/epub+zip").map_err(|e| e.to_string())?;

    // 2. META-INF/container.xml
    zip.start_file("META-INF/container.xml", options).map_err(|e| e.to_string())?;
    zip.write_all(br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles>
    <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
  </rootfiles>
</container>"#).map_err(|e| e.to_string())?;

    // 3. OEBPS/content.opf
    let mut manifest_items = String::new();
    let mut spine_items = String::new();

    for (i, _) in chapters.iter().enumerate() {
        manifest_items.push_str(&format!(
            r#"<item id="ch{}" href="ch{}.xhtml" media-type="application/xhtml+xml"/>"#,
            i, i
        ));
        spine_items.push_str(&format!(r#"<itemref idref="ch{}"/>"#, i));
    }

    let opf_content = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<package version="2.0" xmlns="http://www.idpf.org/2007/opf" unique-identifier="BookId">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
    <dc:title>{}</dc:title>
    <dc:creator>{}</dc:creator>
  </metadata>
  <manifest>
    {}
  </manifest>
  <spine>
    {}
  </spine>
</package>"#,
        title, author, manifest_items, spine_items
    );

    zip.start_file("OEBPS/content.opf", options).map_err(|e| e.to_string())?;
    zip.write_all(opf_content.as_bytes()).map_err(|e| e.to_string())?;

    // 4. Chapters
    for (i, (ch_title, ch_body)) in chapters.iter().enumerate() {
        let ch_xhtml = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE html>
<html>
  <head><title>{}</title></head>
  <body>
    <h1>{}</h1>
    {}
  </body>
</html>"#,
            ch_title, ch_title, ch_body
        );

        zip.start_file(format!("OEBPS/ch{}.xhtml", i), options).map_err(|e| e.to_string())?;
        zip.write_all(ch_xhtml.as_bytes()).map_err(|e| e.to_string())?;
    }

    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn ensure_starter_library(app_dir: &Path) -> Vec<Book> {
    let starter_dir = app_dir.join("starter_library");
    let _ = std::fs::create_dir_all(&starter_dir);
    let now = chrono::Utc::now().timestamp();

    // 1. The Psychology of Money (Master Reference Hero)
    let p1 = starter_dir.join("psychology_of_money.epub");
    if !p1.exists() {
        let chapters = vec![
            ("No One's Crazy", "<p>Your personal experiences with money make up maybe 0.00000001% of what's happened in the world, but maybe 80% of how you think the world works.</p>"),
            ("Luck & Risk", "<p>Nothing is as good or as bad as it looks. Luck and risk are both the reality that every outcome in life is guided by forces other than individual effort alone.</p>"),
            ("Never Enough", "<p>When rich people do crazy things. There is no reason to risk what you have and need for what you don't have and don't need.</p>"),
            ("Confounding Compounding", "<p>More than 99% of Warren Buffett's wealth came after his 50th birthday. His skill is investing, but his secret is time.</p>"),
            ("The Wealth Equation", r#"<p><mark style="background-color: #FEF08A; padding: 2px 6px; border-radius: 4px; color: #1C1917;">Wealth is not about how much you make, but how much you keep, and how hard it works for you.</mark></p>
<p>Most people overestimate their earning power, and underestimate the power of compounding and patience.</p>
<p>The first rule of wealth: don't lose money. The second rule: don't forget the first rule.</p>
<p>Doing well with money has a little to do with how smart you are and a lot to do with how you behave. And behavior is hard to teach, even to really smart people.</p>"#),
            ("Freedom", "<p>The highest form of wealth is the ability to wake up every morning and say, 'I can do whatever I want today.'</p>"),
            ("Man in the Car Paradox", "<p>No one is impressed with your possessions as much as you are.</p>"),
            ("Save Money", "<p>You can build wealth without a high income, but you have no chance of building wealth without a high savings rate.</p>"),
            ("Reasonable > Rational", "<p>Do not aim to be coldly rational when making financial decisions. Aim to be pretty reasonable.</p>"),
            ("Surprise!", "<p>Things that have never happened before happen all the time.</p>"),
            ("Room for Error", "<p>The most important part of every plan is planning on your plan not going according to plan.</p>"),
            ("You'll Change", "<p>Long-term planning is harder than it seems because people's goals and desires change over time.</p>"),
        ];
        let _ = create_epub_file(&p1, "The Psychology of Money", "Morgan Housel", &chapters);
    }

    // 2. Atomic Habits (Recent 1)
    let p2 = starter_dir.join("atomic_habits.epub");
    if !p2.exists() {
        let chapters = vec![
            ("The Surprising Power of Atomic Habits", "<p>Changes that seem small and unimportant at first will compound into remarkable results if you're willing to stick with them for years.</p>"),
            ("How Your Habits Shape Your Identity", "<p>Your habits are how you embody your identity. When you make your bed every day, you embody the identity of an organized person.</p>"),
            ("The 4 Laws of Behavior Change", "<p>Make it obvious. Make it attractive. Make it easy. Make it satisfying.</p>"),
        ];
        let _ = create_epub_file(&p2, "Atomic Habits", "James Clear", &chapters);
    }

    // 3. Sapiens (Recent 2)
    let p3 = starter_dir.join("sapiens.epub");
    if !p3.exists() {
        let chapters = vec![
            ("An Animal of No Significance", "<p>About 14 billion years ago, matter, energy, time and space came into being in what is known as the Big Bang.</p>"),
            ("The Tree of Knowledge", "<p>The Cognitive Revolution is the point when history declared its independence from biology.</p>"),
            ("A Day in the Life of Adam and Eve", "<p>To understand human nature, history and psychology, we must get into the minds of our hunter-gatherer ancestors.</p>"),
        ];
        let _ = create_epub_file(&p3, "Sapiens", "Yuval Noah Harari", &chapters);
    }

    // 4. The Midnight Library (Recent 3)
    let p4 = starter_dir.join("midnight_library.epub");
    if !p4.exists() {
        let chapters = vec![
            ("A Conversation About Rain", "<p>Nineteen years before she decided to die, Nora Seed sat in the warmth of the small library at Hazeldene School.</p>"),
            ("The Midnight Library", "<p>Between life and death there is a library, and within that library, the shelves go on for ever. Every book provides a chance to try another life you could have lived.</p>"),
            ("The Many Lives of Nora Seed", "<p>It is easy to mourn the lives we aren't living. Easy to wish we'd developed other talents, said other yeses.</p>"),
        ];
        let _ = create_epub_file(&p4, "The Midnight Library", "Matt Haig", &chapters);
    }

    vec![
        Book {
            id: "starter-psychology".to_string(),
            title: "The Psychology of Money".to_string(),
            author: "Morgan Housel".to_string(),
            file_path: p1.to_string_lossy().to_string(),
            format: "epub".to_string(),
            cover_image: None,
            total_chapters: 12,
            current_chapter: 4, // Chapter 5
            progress_percent: 0.65,
            last_read_at: now,
            created_at: now - 3600,
        },
        Book {
            id: "starter-atomic".to_string(),
            title: "Atomic Habits".to_string(),
            author: "James Clear".to_string(),
            file_path: p2.to_string_lossy().to_string(),
            format: "epub".to_string(),
            cover_image: None,
            total_chapters: 3,
            current_chapter: 1,
            progress_percent: 0.38,
            last_read_at: now - 86400,
            created_at: now - 86400 * 2,
        },
        Book {
            id: "starter-sapiens".to_string(),
            title: "Sapiens".to_string(),
            author: "Yuval Noah Harari".to_string(),
            file_path: p3.to_string_lossy().to_string(),
            format: "epub".to_string(),
            cover_image: None,
            total_chapters: 3,
            current_chapter: 3,
            progress_percent: 1.0,
            last_read_at: now - 86400 * 3,
            created_at: now - 86400 * 5,
        },
        Book {
            id: "starter-midnight".to_string(),
            title: "The Midnight Library".to_string(),
            author: "Matt Haig".to_string(),
            file_path: p4.to_string_lossy().to_string(),
            format: "epub".to_string(),
            cover_image: None,
            total_chapters: 3,
            current_chapter: 0,
            progress_percent: 0.27,
            last_read_at: now - 86400 * 4,
            created_at: now - 86400 * 6,
        },
    ]
}
