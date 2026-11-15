use crate::engine::starter_covers::generate_editorial_cover;
use crate::engine::{Chapter, EpubBook, TocItem};

pub fn get_starter_book_epub(path_or_id: &str) -> Option<EpubBook> {
    let lower = path_or_id.to_lowercase();
    if lower.contains("psychology") {
        let chapters = vec![
            ("No One's Crazy", "<p>Your personal experiences with money make up maybe 0.00000001% of what's happened in the world, but maybe 80% of how you think the world works.</p><p>Someone who grew up during the Great Depression thinks about risk and reward in ways that someone who grew up during the tech boom cannot fathom.</p>"),
            ("Luck & Risk", "<p>Nothing is as good or as bad as it looks. Luck and risk are both the reality that every outcome in life is guided by forces other than individual effort alone.</p><p>Be careful who you praise and admire. Assume that not all success is due to hard work, and not all poverty is due to laziness.</p>"),
            ("Never Enough", "<p>When rich people do crazy things. Modern capitalism is a pro at two things: generating wealth and generating envy.</p><p>The hardest financial skill is getting the goalpost to stop moving. If expectations rise with results, there is no logic in striving for more.</p>"),
            ("Confounding Compounding", "<p>More than 99% of Warren Buffett's wealth came after his 50th birthday. 97% came after he turned 65.</p><p>His skill is investing, but his secret is time. Compounding works best when you give an asset years and decades to grow.</p>"),
            ("The Wealth Equation", "<p><mark style=\"background-color: #FEF08A; padding: 2px 6px; border-radius: 4px; color: #1C1917;\">Wealth is not about how much you make, but how much you keep, and how hard it works for you.</mark></p><p>Wealth is the nice cars not purchased. The watches not worn. Wealth is financial options not yet converted into stuff.</p>"),
            ("Freedom", "<p>The highest form of wealth is the ability to wake up every morning and say, 'I can do whatever I want today.' More than your salary, control over your time is the highest dividend money pays.</p>"),
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
            cover_image: Some(generate_editorial_cover("The Psychology\nof Money", "MORGAN HOUSEL", "Timeless lessons on wealth & greed", "psychology")),
            toc: toc_vec,
            chapters: ch_vec,
        });
    }

    if lower.contains("atomic") {
        let chapters = vec![
            ("The Surprising Power of Atomic Habits", "<p>Changes that seem small and unimportant at first will compound into remarkable results if you're willing to stick with them for years.</p><p>Habits are the compound interest of self-improvement. Getting 1 percent better each day counts for a lot in the long run.</p>"),
            ("How Your Habits Shape Your Identity", "<p>Your habits are how you embody your identity. When you make your bed every day, you embody the identity of an organized person.</p><p>The real reason habits matter is not that they can get you better results, but that they can change your beliefs about yourself.</p>"),
            ("The Four Laws of Behavior Change", "<p>The process of building a habit can be divided into four simple steps: cue, craving, response, and reward.</p><p>To build a habit: Make it obvious, make it attractive, make it easy, and make it satisfying.</p>"),
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
            cover_image: Some(generate_editorial_cover("Atomic Habits", "JAMES CLEAR", "An Easy & Proven Way to Build Good Habits", "atomic")),
            toc: toc_vec,
            chapters: ch_vec,
        });
    }

    if lower.contains("sapiens") {
        let chapters = vec![
            ("An Animal of No Significance", "<p>One hundred thousand years ago, at least six different species of humans inhabited Planet Earth.</p><p>There was nothing special about them. Nobody had any inkling that their descendants would one day split the atom or fly to the moon.</p>"),
            ("The Tree of Knowledge", "<p>The Cognitive Revolution started roughly 70,000 years ago. Fiction has enabled us not merely to imagine things, but to do so collectively.</p><p>We can weave common myths such as the biblical creation story, nationalism, and limited liability corporations.</p>"),
            ("A Day in the Life of Adam and Eve", "<p>To understand our nature, history, and psychology, we need to get inside the heads of our hunter-gatherer ancestors.</p><p>They possessed intimate knowledge of their surroundings, birds, trees, and seasonal rhythms.</p>"),
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
            cover_image: Some(generate_editorial_cover("Sapiens", "YUVAL NOAH HARARI", "A Brief History of Humankind", "sapiens")),
            toc: toc_vec,
            chapters: ch_vec,
        });
    }

    if lower.contains("midnight") {
        let chapters = vec![
            ("The Conversation", "<p>Nineteen years before she decided to die, Nora Seed sat in the warmth of the small library at Hazeldene School.</p><p>'It's hard to predict,' said Mrs. Elm. 'You could be anything you want to be, Nora. Just look at all these possibilities.'</p>"),
            ("The Library", "<p>Between life and death there is a library, and within that library, the shelves go on forever. Every book provides a chance to try another life you could have lived.</p>"),
            ("The Book of Regrets", "<p>Every life contains millions of decisions, some big, some small. But every time one decision is taken over another, the outcomes differ.</p><p>The Book of Regrets was heavier than the others. It recorded every regret she had ever felt.</p>"),
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
            cover_image: Some(generate_editorial_cover("The Midnight\nLibrary", "MATT HAIG", "Between life and death there is a library", "midnight")),
            toc: toc_vec,
            chapters: ch_vec,
        });
    }

    None
}
