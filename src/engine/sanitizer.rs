use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use ammonia::Builder;

/// Allowlist HTML sanitizer for untrusted EPUB chapter content.
///
/// Only typographic markup survives. All attributes are dropped except a
/// narrow set: event handlers, `style`, `id`, remote `src`/`href`, and
/// `data:`/`javascript:` links can never reach the WebView.
pub fn sanitize_html(html: &str) -> String {
    let html = &strip_self_closing_blocks(html);
    let tags: HashSet<&str> = [
        "a", "p", "div", "span", "br", "hr", "h1", "h2", "h3", "h4", "h5", "h6", "em", "strong",
        "b", "i", "u", "s", "small", "sub", "sup", "mark", "blockquote", "pre", "code", "ul", "ol",
        "li", "dl", "dt", "dd", "table", "thead", "tbody", "tfoot", "tr", "th", "td", "figure",
        "figcaption", "img", "cite", "q", "abbr", "ruby", "rt", "rp",
    ]
    .into_iter()
    .collect();

    let mut tag_attributes: HashMap<&str, HashSet<&str>> = HashMap::new();
    tag_attributes.insert("img", ["src", "alt", "width", "height"].into_iter().collect());
    tag_attributes.insert("a", ["title"].into_iter().collect());
    tag_attributes.insert("td", ["colspan", "rowspan"].into_iter().collect());
    tag_attributes.insert("th", ["colspan", "rowspan"].into_iter().collect());
    tag_attributes.insert("ol", ["start"].into_iter().collect());

    let mut builder = Builder::default();
    builder
        .tags(tags)
        .tag_attributes(tag_attributes)
        .generic_attributes(["class"].into_iter().collect())
        .url_schemes(["data"].into_iter().collect())
        .url_relative(ammonia::UrlRelative::Deny)
        .link_rel(None)
        .attribute_filter(|element, attribute, value| -> Option<Cow<'_, str>> {
            // Epub links are dead ends in a native reader: drop destinations.
            if element == "a" && attribute == "href" {
                return None;
            }
            // Only archive-local images (already inlined as raster data URIs).
            if element == "img" && attribute == "src" {
                let lower = value.to_ascii_lowercase();
                let safe = lower.starts_with("data:image/png;base64,")
                    || lower.starts_with("data:image/jpeg;base64,")
                    || lower.starts_with("data:image/gif;base64,")
                    || lower.starts_with("data:image/webp;base64,");
                return if safe { Some(Cow::Borrowed(value)) } else { None };
            }
            Some(Cow::Borrowed(value))
        });

    builder.clean(html).to_string()
}

/// XHTML allows `<script/>`; the HTML parser treats it as an open tag and eats
/// everything after it. Drop such self-closing dangerous tags first so the
/// following chapter text survives.
fn strip_self_closing_blocks(html: &str) -> String {
    const BLOCKS: [&str; 6] = ["script", "style", "iframe", "object", "embed", "noscript"];
    let mut out = String::with_capacity(html.len());
    let mut i = 0;
    while i < html.len() {
        if html.as_bytes()[i] == b'<' {
            if let Some(rel_end) = html[i..].find('>') {
                let end = i + rel_end + 1;
                let tag = &html[i..end];
                let lower = tag.to_ascii_lowercase();
                let is_block = BLOCKS.iter().any(|name| lower.starts_with(&format!("<{name}")));
                if is_block && lower.trim_end().ends_with("/>") {
                    i = end;
                    continue;
                }
                out.push_str(tag);
                i = end;
                continue;
            }
        }
        let ch = html[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::sanitize_html;

    #[test]
    fn drops_event_handlers_and_script_uris() {
        let out = sanitize_html(
            "<p onclick=\"steal()\">hi</p><img src=x onerror=\"alert(1)\"><a href=\"javascript:alert(1)\">x</a>",
        );
        assert!(!out.contains("onclick"));
        assert!(!out.contains("onerror"));
        assert!(!out.contains("javascript:"));
        assert!(out.contains("hi"));
    }

    #[test]
    fn drops_split_and_malformed_script_tags() {
        for input in [
            "<iframe/src=https://evil>bad</iframe>after",
            "<script/>after",
            "<script\nsrc=\"https://evil\">alert(1)</script>after",
            "<svg onload=\"alert(1)\"><script>alert(2)</script></svg>after",
        ] {
            let out = sanitize_html(input);
            assert!(!out.to_ascii_lowercase().contains("script"), "{out}");
            assert!(!out.to_ascii_lowercase().contains("onload"), "{out}");
            assert!(!out.to_ascii_lowercase().contains("iframe"), "{out}");
            assert!(out.contains("after"), "content lost for {input}: {out}");
        }
    }

    #[test]
    fn keeps_typography_and_inlined_images() {
        let data = "data:image/png;base64,iVBORw0KGgo=";
        let input = format!(
            "<h1>Chapter</h1><p>A <em>beautiful</em> <strong>day</strong>.</p><blockquote>Q</blockquote><img src=\"{data}\" alt=\"pic\">"
        );
        let out = sanitize_html(&input);
        assert!(out.contains("<h1>Chapter</h1>"));
        assert!(out.contains("<em>beautiful</em>"));
        assert!(out.contains("<blockquote>Q</blockquote>"));
        assert!(out.contains(data));
        assert!(out.contains("alt=\"pic\""));
    }

    #[test]
    fn strips_remote_resources() {
        let out = sanitize_html("<img src=\"https://tracker.example/p.gif\">");
        assert!(!out.contains("tracker.example"));
        assert!(!out.contains("src"));
    }
}
