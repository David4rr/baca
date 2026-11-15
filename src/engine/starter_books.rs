use std::io::Write;
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;
use crate::db::models::Book;
use crate::engine::starter_covers::generate_editorial_cover;
use crate::engine::{Chapter, EpubBook, TocItem};

pub fn get_starter_book_epub(path_or_id: &str) -> Option<EpubBook> {
    let lower = path_or_id.to_lowercase();
    if lower.contains("psychology") {
        let chapters = vec![
            ("No One's Crazy", r#"<p>Your personal experiences with money make up maybe 0.00000001% of what's happened in the world, but maybe 80% of how you think the world works. Therefore, equally smart people can disagree about how and why the economy moves.</p>
<p>Someone who grew up during the Great Depression thinks about risk and reward in ways that someone who grew up during the tech boom of the 1990s cannot fathom. Their view of money was formed in different worlds.</p>
<p>When you see someone make a financial decision that seems irrational to you, remember: no one is crazy. People from different generations, raised by different parents who earned different incomes in different parts of the world, learn vastly different lessons.</p>"#),
            ("Luck & Risk", r#"<p>Nothing is as good or as bad as it looks. Luck and risk are both the reality that every outcome in life is guided by forces other than individual effort alone.</p>
<p>They are siblings. They are both the realization that every outcome in life is guided by forces other than individual effort. They happen because the world is too complex to allow 100% of your actions to dictate 100% of your outcomes.</p>
<p>Be careful who you praise and admire. Be careful who you look down upon and wish to avoid becoming. Assume that not all success is due to hard work, and not all poverty is due to laziness.</p>"#),
            ("Never Enough", r#"<p>When rich people do crazy things. Modern capitalism is a pro at two things: generating wealth and generating envy. Perhaps they go hand in hand; wanting to surpass your peers can be the fuel of analytical effort.</p>
<p>The hardest financial skill is getting the goalpost to stop moving. If expectations rise with results, there is no logic in striving for more because you'll feel the same after putting in extra effort.</p>
<p>There is no reason to risk what you have and need for what you don't have and don't need. It's one of those things that is obvious, yet easily forgotten in the heat of ambition.</p>"#),
            ("Confounding Compounding", r#"<p>More than 99% of Warren Buffett's wealth came after his 50th birthday. 97% came after he turned 65. If he had retired at 60 like ordinary people, few would have ever heard of him.</p>
<p>His skill is investing, but his secret is time. Compounding works best when you give an asset years and decades to grow. It is like oak trees: a single year looks uneventful, but fifty years creates an empire.</p>
<p>Good investing isn't necessarily about earning the highest returns, because the highest returns tend to be one-off hits that can't be repeated. It's about earning pretty good returns that you can stick with and which can be repeated for the longest period of time.</p>"#),
            ("The Wealth Equation", r#"<p><mark style="background-color: #FEF08A; padding: 2px 6px; border-radius: 4px; color: #1C1917;">Wealth is not about how much you make, but how much you keep, and how hard it works for you.</mark></p>
<p>Most people overestimate their earning power, and underestimate the power of compounding and patience. Wealth is the nice cars not purchased. The watches not worn, the first-class upgrades declined. Wealth is financial options not yet converted into stuff.</p>
<p>The first rule of wealth: don't lose money. The second rule: don't forget the first rule. Doing well with money has a little to do with how smart you are and a lot to do with how you behave.</p>
<p>Spending money to show people how much money you have is the fastest way to have less money. True financial independence means having the freedom to wake up every morning and say, 'I can do whatever I want today.'</p>"#),
            ("Freedom", r#"<p>The highest form of wealth is the ability to wake up every morning and say, 'I can do whatever I want today.' More than your salary, more than the size of your house, more than the prestige of your job, control over your time is the highest dividend money pays.</p>
<p>Having a strong sense of controlling one's life is a more dependable predictor of positive feelings of wellbeing than any of the objective conditions of life we have considered.</p>
<p>Use money to gain control over your time, because not having control of your time is such a powerful and universal drag on happiness.</p>"#),
        ];

        let mut ch_vec = Vec::new();
        let mut toc_vec = Vec::new();
        for (i, (t, h)) in chapters.into_iter().enumerate() {
            toc_vec.push(TocItem { title: t.to_string(), chapter_index: i });
            ch_vec.push(Chapter { index: i, title: t.to_string(), html_content: h.to_string() });
        }
        return Some(EpubBook {
            title: "The Psychology of Money".to_string(),
            author: "Morgan Housel".to_string(),
            cover_image: Some(generate_editorial_cover(
                "The Psychology\nof Money",
                "MORGAN HOUSEL",
                "Timeless lessons on wealth & greed",
                "psychology",
            )),
            toc: toc_vec,
            chapters: ch_vec,
        });
    }

    if lower.contains("atomic") {
        let chapters = vec![
            ("The Surprising Power of Atomic Habits", r#"<p>Changes that seem small and unimportant at first will compound into remarkable results if you're willing to stick with them for years.</p>
<p>We all deal with setbacks, but in the long run, the quality of our lives often depends on the quality of our habits. With the same habits, you'll end up with the same results. But with better habits, anything is possible.</p>
<p>Habits are the compound interest of self-improvement. Getting 1 percent better each day counts for a lot in the long run.</p>"#),
            ("How Your Habits Shape Your Identity", r#"<p>Your habits are how you embody your identity. When you make your bed every day, you embody the identity of an organized person. When you write every day, you embody the identity of a creative person.</p>
<p>The real reason habits matter is not that they can get you better results, but that they can change your beliefs about yourself. You begin to trust your own promises to yourself.</p>"#),
            ("The 4 Laws of Behavior Change", r#"<p>If you want to build a habit, make it obvious, make it attractive, make it easy, and make it satisfying. Inversion for breaking bad habits: make it invisible, make it unattractive, make it difficult, and make it unsatisfying.</p>
<p>Environment is the invisible hand that shapes human behavior. We often choose products not because of what they are, but because of where they are.</p>"#),
        ];
        let mut ch_vec = Vec::new();
        let mut toc_vec = Vec::new();
        for (i, (t, h)) in chapters.into_iter().enumerate() {
            toc_vec.push(TocItem { title: t.to_string(), chapter_index: i });
            ch_vec.push(Chapter { index: i, title: t.to_string(), html_content: h.to_string() });
        }
        return Some(EpubBook {
            title: "Atomic Habits".to_string(),
            author: "James Clear".to_string(),
            cover_image: Some(generate_editorial_cover(
                "Atomic Habits",
                "JAMES CLEAR",
                "An Easy & Proven Way to Build Good Habits",
                "atomic",
            )),
            toc: toc_vec,
            chapters: ch_vec,
        });
    }

    if lower.contains("sapiens") {
        let chapters = vec![
            ("An Animal of No Significance", r#"<p>About 14 billion years ago, matter, energy, time and space came into being in what is known as the Big Bang. The story of these fundamental features of our universe is called physics.</p>
<p>About 300,000 years later, matter and energy started to coalesce into complex structures called atoms, which then combined into molecules. The story of atoms, molecules and their interactions is called chemistry.</p>
<p>About 3.8 billion years ago, on a planet called Earth, certain molecules combined to form particularly large and intricate structures called organisms. The story of organisms is called biology.</p>"#),
            ("The Tree of Knowledge", r#"<p>The Cognitive Revolution is the point when history declared its independence from biology. From this point onwards, historical narratives replace biological theories as our primary means of explaining the development of Homo sapiens.</p>
<p>Fiction has enabled us not merely to imagine things, but to do so collectively. We can weave common myths such as the biblical creation story, the Dreamtime myths of Indigenous Australians, and the nationalist myths of modern states.</p>"#),
            ("A Day in the Life of Adam and Eve", r#"<p>To understand human nature, history and psychology, we must get into the minds of our hunter-gatherer ancestors. For nearly all of our species' existence, humans lived as foragers.</p>
<p>The foraging lifestyle varied significantly from one region to another and from one season to the next, but on the whole, foragers enjoyed a more comfortable and rewarding lifestyle than most peasants, shepherds, and modern office workers who came after them.</p>"#),
        ];
        let mut ch_vec = Vec::new();
        let mut toc_vec = Vec::new();
        for (i, (t, h)) in chapters.into_iter().enumerate() {
            toc_vec.push(TocItem { title: t.to_string(), chapter_index: i });
            ch_vec.push(Chapter { index: i, title: t.to_string(), html_content: h.to_string() });
        }
        return Some(EpubBook {
            title: "Sapiens".to_string(),
            author: "Yuval Noah Harari".to_string(),
            cover_image: Some(generate_editorial_cover(
                "Sapiens",
                "YUVAL NOAH HARARI",
                "A Brief History of Humankind",
                "sapiens",
            )),
            toc: toc_vec,
            chapters: ch_vec,
        });
    }

    if lower.contains("midnight") {
        let chapters = vec![
            ("A Conversation About Rain", r#"<p>Nineteen years before she decided to die, Nora Seed sat in the warmth of the small library at Hazeldene School in the town of Bedford. She was staring at a chessboard on a low table.</p>
<p>'Nora dear, it's natural to worry about your future,' said the librarian, Mrs Elm, her eyes twinkling like frosted glass under the amber lamps.</p>
<p>'I don't just worry about my future,' Nora said. 'I worry about everyone's future. What if nothing ever gets better?' Mrs Elm moved a knight across the board with a gentle click. 'Every move changes the game, Nora. Always.'</p>"#),
            ("The Midnight Library", r#"<p>Between life and death there is a library, and within that library, the shelves go on for ever. Every book provides a chance to try another life you could have lived. To see how things would be different if you had made other choices.</p>
<p>'Would you have done anything different, if you had the chance to undo your regrets?' Mrs Elm asked softly, standing beside a shelf stacked with deep green volumes that stretched into the cosmos.</p>"#),
            ("The Many Lives of Nora Seed", r#"<p>It is easy to mourn the lives we aren't living. Easy to wish we'd developed other talents, said other yeses, worked harder, loved deeper, kept promises we abandoned.</p>
<p>It is not the lives we didn't live that weigh upon us, Nora realized. It is the weight of believing we failed the only one we have. You don't have to understand life. You just have to live it.</p>"#),
        ];
        let mut ch_vec = Vec::new();
        let mut toc_vec = Vec::new();
        for (i, (t, h)) in chapters.into_iter().enumerate() {
            toc_vec.push(TocItem { title: t.to_string(), chapter_index: i });
            ch_vec.push(Chapter { index: i, title: t.to_string(), html_content: h.to_string() });
        }
        return Some(EpubBook {
            title: "The Midnight Library".to_string(),
            author: "Matt Haig".to_string(),
            cover_image: Some(generate_editorial_cover(
                "The Midnight\nLibrary",
                "MATT HAIG",
                "Between life and death there is a library",
                "midnight",
            )),
            toc: toc_vec,
            chapters: ch_vec,
        });
    }

    None
}

pub fn create_epub_file(
    path: &Path,
    title: &str,
    author: &str,
    chapters: &[(&str, &str)],
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    zip.start_file("mimetype", options).map_err(|e| e.to_string())?;
    zip.write_all(b"application/epub+zip").map_err(|e| e.to_string())?;

    zip.start_file("META-INF/container.xml", options).map_err(|e| e.to_string())?;
    zip.write_all(br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles>
    <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
  </rootfiles>
</container>"#).map_err(|e| e.to_string())?;

    let mut manifest_items = String::new();
    let mut spine_items = String::new();
    for (i, _) in chapters.iter().enumerate() {
        manifest_items.push_str(&format!(
            r#"<item id="ch{}" href="ch{}.xhtml" media-type="application/xhtml+xml"/>"#,
            i, i
        ));
        spine_items.push_str(&format!(r#"<itemref idref="ch{}"/>"#, i));
    }

    let opf = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<package version="2.0" xmlns="http://www.idpf.org/2007/opf" unique-identifier="BookId">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
    <dc:title>{}</dc:title>
    <dc:creator>{}</dc:creator>
  </metadata>
  <manifest>{}</manifest>
  <spine>{}</spine>
</package>"#,
        title, author, manifest_items, spine_items
    );

    zip.start_file("OEBPS/content.opf", options).map_err(|e| e.to_string())?;
    zip.write_all(opf.as_bytes()).map_err(|e| e.to_string())?;

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

    let p1 = starter_dir.join("psychology_of_money.epub");
    let cover_1 = generate_editorial_cover(
        "The Psychology\nof Money",
        "MORGAN HOUSEL",
        "Timeless lessons on wealth & greed",
        "psychology",
    );

    let p2 = starter_dir.join("atomic_habits.epub");
    let cover_2 = generate_editorial_cover(
        "Atomic Habits",
        "JAMES CLEAR",
        "An Easy & Proven Way to Build Good Habits",
        "atomic",
    );

    let p3 = starter_dir.join("sapiens.epub");
    let cover_3 = generate_editorial_cover(
        "Sapiens",
        "YUVAL NOAH HARARI",
        "A Brief History of Humankind",
        "sapiens",
    );

    let p4 = starter_dir.join("midnight_library.epub");
    let cover_4 = generate_editorial_cover(
        "The Midnight\nLibrary",
        "MATT HAIG",
        "Between life and death there is a library",
        "midnight",
    );

    vec![
        Book {
            id: "starter-psychology".to_string(),
            title: "The Psychology of Money".to_string(),
            author: "Morgan Housel".to_string(),
            file_path: p1.to_string_lossy().to_string(),
            format: "epub".to_string(),
            cover_image: Some(cover_1),
            total_chapters: 6,
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
            cover_image: Some(cover_2),
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
            cover_image: Some(cover_3),
            total_chapters: 3,
            current_chapter: 2,
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
            cover_image: Some(cover_4),
            total_chapters: 3,
            current_chapter: 0,
            progress_percent: 0.27,
            last_read_at: now - 86400 * 4,
            created_at: now - 86400 * 6,
        },
    ]
}
