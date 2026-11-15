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
            let plain_text = strip_tags(&ch.html_content);
            let plain_lower = plain_text.to_lowercase();

            let mut start_pos = 0;
            while let Some(found_idx) = plain_lower[start_pos..].find(&q_lower) {
                let actual_idx = start_pos + found_idx;

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
        // Frosted Glass Backdrop
        div {
            class: "fixed inset-0 z-50 bg-black/40 backdrop-blur-sm animate-fade-in flex items-center justify-center p-4",
            onclick: move |_| on_close.call(()),

            // Apple Center Dialog
            div {
                class: "w-full max-w-lg rounded-[28px] border shadow-2xl flex flex-col overflow-hidden max-h-[82vh] animate-slide-up",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()}; box-shadow: {theme.card_shadow()};",
                onclick: |e| e.stop_propagation(),

                // Header with Search Input
                div {
                    class: "p-4 sm:p-5 border-b flex items-center gap-3",
                    style: "border-color: {theme.border_color()};",

                    div {
                        class: "w-8 h-8 rounded-full flex items-center justify-center opacity-50",
                        style: "width: 32px; height: 32px; min-width: 32px; min-height: 32px;",
                        svg {
                            class: "w-4 h-4",
                            width: "16",
                            height: "16",
                            style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.2",
                            circle { cx: "11", cy: "11", r: "8" }
                            line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                        }
                    }

                    input {
                        r#type: "text",
                        class: "flex-1 bg-transparent border-none text-base font-sans outline-none placeholder:opacity-40",
                        placeholder: "Cari kata atau kalimat di buku...",
                        value: "{query}",
                        autofocus: true,
                        oninput: move |e| do_search(e.value()),
                    }

                    button {
                        class: "w-8 h-8 rounded-full flex items-center justify-center apple-tap opacity-60 hover:opacity-100 transition-all",
                        style: "width: 32px; height: 32px; min-width: 32px; min-height: 32px; background-color: {theme.surface_subtle()}; color: {theme.text_primary()};",
                        onclick: move |_| on_close.call(()),
                        svg {
                            class: "w-4 h-4",
                            width: "16",
                            height: "16",
                            style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            line { x1: "18", y1: "6", x2: "6", y2: "18" }
                            line { x1: "6", y1: "6", x2: "18", y2: "18" }
                        }
                    }
                }

                // Subheader Counter
                div {
                    class: "px-6 py-2.5 text-[11px] font-sans font-medium uppercase tracking-wider opacity-50 border-b flex justify-between",
                    style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",
                    span {
                        if query().len() < 2 {
                            "Ketik minimal 2 huruf untuk mencari"
                        } else {
                            "{hits.read().len()} Hasil Ditemukan"
                        }
                    }
                }

                // Results List
                div {
                    class: "flex-1 overflow-y-auto p-4 space-y-2",
                    if hits.read().is_empty() && query().len() >= 2 {
                        div {
                            class: "py-16 text-center opacity-40 text-xs font-sans",
                            "Tidak ada teks yang cocok dalam buku ini."
                        }
                    } else {
                        for hit in hits.read().iter() {
                            {
                                let ch_idx = hit.chapter_index;
                                let term = hit.matched_term.clone();
                                rsx! {
                                    div {
                                        class: "p-3.5 rounded-2xl border apple-tap cursor-pointer transition-all shadow-sm",
                                        style: "border-color: {theme.border_color()}; background-color: {theme.surface_color()};",
                                        onclick: move |_| {
                                            on_select_match.call((ch_idx, term.clone()));
                                            on_close.call(());
                                        },
                                        div {
                                            class: "text-[11px] font-sans font-semibold uppercase tracking-wider mb-1.5 opacity-60",
                                            "{hit.chapter_title}"
                                        }
                                        p {
                                            class: "font-serif text-[13px] leading-relaxed line-clamp-2",
                                            span { class: "opacity-70", "...{hit.snippet_before}" }
                                            span {
                                                class: "font-semibold px-1 py-0.5 rounded mx-0.5",
                                                style: "background-color: #FEF08A; color: #1C1917;",
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
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;

    for c in html.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
            out.push(' ');
        } else if !in_tag {
            out.push(c);
        }
    }

    out.split_whitespace().collect::<Vec<_>>().join(" ")
}
