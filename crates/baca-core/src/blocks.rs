/// Native reader block types produced from sanitized EPUB chapter HTML so
/// native UI toolkits (Jetpack Compose / SwiftUI) can render 120Hz typography
/// without a WebView.
#[derive(Debug, Clone, PartialEq, uniffi::Enum)]
pub enum ReaderBlock {
    Heading { level: u8, text: String },
    Paragraph { text: String },
    Blockquote { text: String },
    Divider,
}

pub fn html_to_blocks(html: &str) -> Vec<ReaderBlock> {
    let mut blocks = Vec::new();
    let lower = html.to_ascii_lowercase();
    let bytes = html.as_bytes();
    let mut pos = 0;

    while pos < bytes.len() {
        if let Some(rel_lt) = lower[pos..].find('<') {
            let text_before = clean_text(&html[pos..pos + rel_lt]);
            if !text_before.is_empty() {
                blocks.push(ReaderBlock::Paragraph { text: text_before });
            }
            let tag_start = pos + rel_lt;
            let Some(rel_gt) = lower[tag_start..].find('>') else {
                break;
            };
            let tag_end = tag_start + rel_gt + 1;
            let tag_inner = lower[tag_start + 1..tag_end - 1].trim();
            let tag_name = tag_inner.split_whitespace().next().unwrap_or("");

            if matches!(tag_name, "hr" | "hr/") {
                blocks.push(ReaderBlock::Divider);
                pos = tag_end;
                continue;
            }

            let block_kind = match tag_name {
                "h1" => Some(("</h1>", 1u8)),
                "h2" => Some(("</h2>", 2u8)),
                "h3" => Some(("</h3>", 3u8)),
                "h4" | "h5" | "h6" => Some((
                    match tag_name {
                        "h4" => "</h4>",
                        "h5" => "</h5>",
                        _ => "</h6>",
                    },
                    4u8,
                )),
                "blockquote" => Some(("</blockquote>", 10u8)),
                "p" => Some(("</p>", 0u8)),
                "li" => Some(("</li>", 0u8)),
                _ => None,
            };

            if let Some((close_tag, kind)) = block_kind {
                if let Some(rel_close) = lower[tag_end..].find(close_tag) {
                    let inner_html = &html[tag_end..tag_end + rel_close];
                    let text = strip_inline_tags(inner_html);
                    if !text.is_empty() {
                        match kind {
                            1..=4 => blocks.push(ReaderBlock::Heading { level: kind, text }),
                            10 => blocks.push(ReaderBlock::Blockquote { text }),
                            _ => blocks.push(ReaderBlock::Paragraph { text }),
                        }
                    }
                    pos = tag_end + rel_close + close_tag.len();
                    continue;
                }
            }
            pos = tag_end;
        } else {
            let tail = clean_text(&html[pos..]);
            if !tail.is_empty() {
                blocks.push(ReaderBlock::Paragraph { text: tail });
            }
            break;
        }
    }

    blocks
}

fn strip_inline_tags(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => {
                in_tag = true;
                out.push(' ');
            }
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    clean_text(&out)
}

fn clean_text(raw: &str) -> String {
    let decoded = decode_entities(raw);
    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn decode_entities(s: &str) -> String {
    if !s.contains('&') {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        let after = &rest[amp..];
        if let Some(semi) = after.find(';').filter(|&i| i <= 10) {
            let entity = &after[1..semi];
            let replacement = match entity {
                "amp" => Some('&'),
                "lt" => Some('<'),
                "gt" => Some('>'),
                "quot" => Some('"'),
                "apos" | "#39" => Some('\''),
                "nbsp" => Some(' '),
                "mdash" => Some('—'),
                "ndash" => Some('–'),
                "hellip" => Some('…'),
                "lsquo" | "rsquo" => Some('\''),
                "ldquo" | "rdquo" => Some('"'),
                _ if entity.starts_with('#') => parse_numeric_entity(&entity[1..]),
                _ => None,
            };
            if let Some(ch) = replacement {
                out.push(ch);
                rest = &after[semi + 1..];
                continue;
            }
        }
        out.push('&');
        rest = &rest[amp + 1..];
    }
    out.push_str(rest);
    out
}

fn parse_numeric_entity(code: &str) -> Option<char> {
    let num = if let Some(hex) = code.strip_prefix('x').or_else(|| code.strip_prefix('X')) {
        u32::from_str_radix(hex, 16).ok()?
    } else {
        code.parse::<u32>().ok()?
    };
    char::from_u32(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_headings_paragraphs_and_entities() {
        let html = "<h1>Bab &amp; Awal</h1><p>Halo <em>dunia</em> &mdash; teks.</p><hr/><blockquote>Kutipan</blockquote>";
        let blocks = html_to_blocks(html);
        assert_eq!(
            blocks,
            vec![
                ReaderBlock::Heading { level: 1, text: "Bab & Awal".to_string() },
                ReaderBlock::Paragraph { text: "Halo dunia — teks.".to_string() },
                ReaderBlock::Divider,
                ReaderBlock::Blockquote { text: "Kutipan".to_string() },
            ]
        );
    }
}
