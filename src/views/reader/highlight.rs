/// Case-insensitive (ASCII) highlighter that never slices inside a UTF-8
/// codepoint and never rewrites text inside HTML tags or attributes.
pub fn highlight_html(html: &str, keyword: &str) -> String {
    let keyword = keyword.trim();
    if keyword.is_empty() || html.len() < keyword.len() {
        return html.to_string();
    }

    let bytes = html.as_bytes();
    let needle = keyword.as_bytes();
    let mut result = String::with_capacity(html.len() + 64);
    let mut last = 0;
    let mut i = 0;
    let mut in_tag = false;

    while i < html.len() {
        let ch = html[i..].chars().next().unwrap();

        if ch == '<' {
            in_tag = true;
            i += 1;
            continue;
        }
        if ch == '>' {
            in_tag = false;
            i += 1;
            continue;
        }

        if !in_tag
            && i + needle.len() <= bytes.len()
            && bytes[i..i + needle.len()].eq_ignore_ascii_case(needle)
        {
            result.push_str(&html[last..i]);
            result.push_str("<mark class=\"baca-hit\">");
            result.push_str(&html[i..i + needle.len()]);
            result.push_str("</mark>");
            i += needle.len();
            last = i;
            continue;
        }

        i += ch.len_utf8();
    }

    result.push_str(&html[last..]);
    result
}

#[cfg(test)]
mod tests {
    use super::highlight_html;

    #[test]
    fn handles_non_ascii_without_panicking() {
        for html in [
            "<p>İSTANBUL ıstanbul</p>",
            "<p>ẞtraße STRASSE</p>",
            "<p>日本語のテキスト</p>",
        ] {
            let _ = highlight_html(html, "istanbul");
            let _ = highlight_html(html, "日本");
            let _ = highlight_html(html, "ß");
        }
    }

    #[test]
    fn does_not_highlight_tags() {
        let out = highlight_html("<p class=\"title\">title</p>", "title");
        assert!(out.contains("class=\"title\""));
        assert_eq!(out.matches("<mark").count(), 1);
    }
}
