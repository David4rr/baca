/// Case-insensitive HTML text highlighter that preserves tags and attributes.
pub fn highlight_html(html: &str, keyword: &str) -> String {
    let lower_html = html.to_lowercase();
    let lower_kw = keyword.to_lowercase();
    let kw_len = keyword.len();
    if kw_len == 0 { return html.to_string(); }

    let mut result = String::with_capacity(html.len() + 256);
    let mut last_idx = 0;

    for (start_byte, _) in lower_html.match_indices(&lower_kw) {
        if start_byte < last_idx { continue; }
        let preceding = &html[..start_byte];
        let in_tag = preceding.rfind('<').map_or(false, |open| {
            preceding.rfind('>').map_or(true, |close| open > close)
        });
        if in_tag { continue; }

        result.push_str(&html[last_idx..start_byte]);
        result.push_str("<mark class=\"bg-amber-300 text-stone-900 rounded px-1 font-semibold\">");
        result.push_str(&html[start_byte..start_byte + kw_len]);
        result.push_str("</mark>");
        last_idx = start_byte + kw_len;
    }
    result.push_str(&html[last_idx..]);
    result
}
