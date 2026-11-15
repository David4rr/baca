use dioxus::prelude::*;
use crate::engine::Chapter;
use crate::theme::ThemeMode;

#[derive(Debug, Clone, PartialEq)]
pub struct SearchHit {
    pub chapter_index: usize,
    pub chapter_title: String,
    pub snippet_before: String,
    pub matched_term: String,
    pub snippet_after: String,
}

#[component]
pub fn InBookSearchModal(
    chapters: Vec<Chapter>,
    theme: ThemeMode,
    is_open: bool,
    on_close: EventHandler<()>,
    on_select_match: EventHandler<(usize, String)>,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    let mut query = use_signal(|| "".to_string());
    let mut hits = use_signal(|| Vec::<SearchHit>::new());

    let mut do_search = move |q_str: String| {
        let q = q_str.trim().to_string();
        query.set(q.clone());

        if q.len() < 2 {
            hits.set(Vec::new());
            return;
        }

        let q_lower = q.to_lowercase();
        let mut results = Vec::new();

        for (idx, ch) in chapters.iter().enumerate() {
            // Extract raw text by stripping HTML tags
            let plain_text = strip_tags(&ch.html_content);
            let plain_lower = plain_text.to_lowercase();

            let mut start_pos = 0;
            while let Some(found_idx) = plain_lower[start_pos..].find(&q_lower) {
                let actual_idx = start_pos + found_idx;

                // Extract context window
                let before_start = actual_idx.saturating_sub(40);
                let snippet_before = plain_text[before_start..actual_idx].trim_start().to_string();
                let matched_term = plain_text[actual_idx..actual_idx + q.len()].to_string();
                let after_end = (actual_idx + q.len() + 60).min(plain_text.len());
                let snippet_after = plain_text[actual_idx + q.len()..after_end].trim_end().to_string();

                results.push(SearchHit {
                    chapter_index: idx,
                    chapter_title: ch.title.clone(),
                    snippet_before,
                    matched_term,
                    snippet_after,
                });

                if results.len() >= 50 {
                    break;
                }

                start_pos = actual_idx + q.len();
            }

            if results.len() >= 50 {
                break;
            }
        }

        hits.set(results);
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-fade-in",
            onclick: move |_| on_close.call(()),

            div {
                class: "w-full max-w-xl max-h-[85vh] rounded-3xl border shadow-2xl flex flex-col overflow-hidden",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                onclick: move |e| e.stop_propagation(),

                // Search Header
                div {
                    class: "p-4 border-b flex items-center gap-3",
                    style: "border-color: {theme.border_color()};",
                    span { class: "text-lg opacity-50", "🔍" }
                    input {
                        r#type: "text",
                        class: "flex-1 bg-transparent border-none text-base outline-none font-medium",
                        placeholder: "Cari kata atau kalimat di dalam buku...",
                        value: "{query}",
                        autofocus: true,
                        oninput: move |e| do_search(e.value()),
                    }
                    button {
                        class: "p-1.5 rounded-full hover:bg-white/10 opacity-60 hover:opacity-100 text-sm font-bold",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                // Results Counter
                div {
                    class: "px-5 py-2 text-[11px] font-bold uppercase tracking-wider opacity-50 border-b flex justify-between",
                    style: "border-color: {theme.border_color()};",
                    span {
                        if query().len() < 2 {
                            "Ketik minimal 2 karakter"
                        } else {
                            "{hits.read().len()} Hasil Ditemukan"
                        }
                    }
                }

                // Results List
                div {
                    class: "flex-1 overflow-y-auto p-3 space-y-2 divide-y divide-white/5",
                    if hits.read().is_empty() && query().len() >= 2 {
                        div {
                            class: "py-12 text-center opacity-40 text-sm",
                            "Tidak ada teks yang cocok dalam buku ini."
                        }
                    } else {
                        for hit in hits.read().iter() {
                            {
                                let ch_idx = hit.chapter_index;
                                let term = hit.matched_term.clone();
                                rsx! {
                                    div {
                                        class: "p-3 rounded-2xl hover:bg-white/5 cursor-pointer transition-colors text-left group",
                                        onclick: move |_| {
                                            on_select_match.call((ch_idx, term.clone()));
                                            on_close.call(());
                                        },
                                        div {
                                            class: "text-[11px] font-bold uppercase tracking-wider mb-1 opacity-60 group-hover:opacity-90",
                                            style: "color: {theme.accent_color()};",
                                            "{hit.chapter_title}"
                                        }
                                        p {
                                            class: "text-xs leading-relaxed line-clamp-2",
                                            span { class: "opacity-70", "...{hit.snippet_before}" }
                                            span {
                                                class: "font-bold px-1 py-0.5 rounded mx-0.5",
                                                style: format!("background-color: {}; color: #000;", theme.accent_color()),
                                                "{hit.matched_term}"
                                            }
                                            span { class: "opacity-70", "{hit.snippet_after}..." }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn strip_tags(html: &str) -> String {
    let mut output = String::with_capacity(html.len());
    let mut in_tag = false;

    for ch in html.chars() {
        if ch == '<' {
            in_tag = true;
        } else if ch == '>' {
            in_tag = false;
            output.push(' ');
        } else if !in_tag {
            output.push(ch);
        }
    }
    output
}
