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
    if !is_open { return rsx! {}; }

    let mut query = use_signal(String::new);
    let mut hits = use_signal(Vec::<SearchHit>::new);

    let mut do_search = move |q_str: String| {
        let q = q_str.trim().to_string();
        query.set(q.clone());
        if q.len() < 2 { hits.set(Vec::new()); return; }

        let q_lower = q.to_lowercase();
        let mut results = Vec::new();

        for (idx, ch) in chapters.iter().enumerate() {
            let plain = strip_tags(&ch.html_content);
            let plain_lower = plain.to_lowercase();
            let mut start_pos = 0;

            while let Some(found_idx) = plain_lower[start_pos..].find(&q_lower) {
                let actual = start_pos + found_idx;
                let b_start = actual.saturating_sub(40);
                results.push(SearchHit {
                    chapter_index: idx,
                    chapter_title: ch.title.clone(),
                    snippet_before: plain[b_start..actual].trim_start().to_string(),
                    matched_term: plain[actual..actual + q.len()].to_string(),
                    snippet_after: plain[actual + q.len()..(actual + q.len() + 60).min(plain.len())].trim_end().to_string(),
                });
                if results.len() >= 50 { break; }
                start_pos = actual + q.len();
            }
            if results.len() >= 50 { break; }
        }
        hits.set(results);
    };

    rsx! {
        div { class: "fixed inset-0 z-50 bg-black/40 backdrop-blur-sm animate-fade-in flex items-center justify-center p-4", onclick: move |_| on_close.call(()),
            div { class: "w-full max-w-lg rounded-[28px] border shadow-2xl flex flex-col overflow-hidden max-h-[82vh] animate-slide-up",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                onclick: |e| e.stop_propagation(),

                div { class: "p-4 sm:p-5 border-b flex items-center gap-3", style: "border-color: {theme.border_color()};",
                    input { r#type: "text", class: "flex-1 bg-transparent border-none text-base font-sans outline-none placeholder:opacity-40",
                        placeholder: "Cari kata atau kalimat di buku...", value: "{query}", autofocus: true, oninput: move |e| do_search(e.value())
                    }
                    button { class: "w-8 h-8 rounded-full flex items-center justify-center apple-tap opacity-60 hover:opacity-100", style: "background-color: {theme.surface_subtle()};", onclick: move |_| on_close.call(()), "✕" }
                }

                div { class: "px-6 py-2.5 text-[11px] font-sans font-medium uppercase tracking-wider opacity-50 border-b", style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",
                    if query().len() < 2 { "Ketik minimal 2 huruf untuk mencari" } else { "{hits.read().len()} Hasil Ditemukan" }
                }

                div { class: "flex-1 overflow-y-auto p-4 space-y-2",
                    if hits.read().is_empty() && query().len() >= 2 {
                        div { class: "py-16 text-center opacity-40 text-xs font-sans", "Tidak ada teks yang cocok dalam buku ini." }
                    } else {
                        for hit in hits.read().iter() {
                            {
                                let (ch_idx, term) = (hit.chapter_index, hit.matched_term.clone());
                                rsx! {
                                    div { class: "p-3.5 rounded-2xl border apple-tap cursor-pointer shadow-sm", style: "border-color: {theme.border_color()}; background-color: {theme.surface_color()};",
                                        onclick: move |_| { on_select_match.call((ch_idx, term.clone())); on_close.call(()); },
                                        div { class: "text-[11px] font-sans font-semibold uppercase tracking-wider mb-1 opacity-60", "{hit.chapter_title}" }
                                        p { class: "font-serif text-[13px] leading-relaxed line-clamp-2",
                                            span { class: "opacity-70", "...{hit.snippet_before}" }
                                            span { class: "font-semibold px-1 py-0.5 rounded mx-0.5 bg-amber-200 text-stone-900", "{hit.matched_term}" }
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
        if c == '<' { in_tag = true; }
        else if c == '>' { in_tag = false; out.push(' '); }
        else if !in_tag { out.push(c); }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}
