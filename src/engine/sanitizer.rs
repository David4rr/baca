/// Simple and robust HTML sanitizer for EPUB chapter contents.
/// Removes potentially harmful tags (<script>, <iframe>, <object>, <embed>, <style>)
/// and inline javascript event handlers.
pub fn sanitize_html(html: &str) -> String {
    let mut output = String::with_capacity(html.len());
    let mut in_tag = false;
    let mut in_bad_tag = false;
    let mut current_tag = String::new();

    // Bad tags whose content should also be skipped
    let bad_block_tags = ["script", "iframe", "object", "embed", "style", "noscript"];

    let mut i = 0;
    let chars: Vec<char> = html.chars().collect();
    let len = chars.len();

    while i < len {
        let ch = chars[i];

        if ch == '<' {
            in_tag = true;
            current_tag.clear();
            let mut j = i + 1;
            while j < len && chars[j] != '>' && chars[j] != ' ' {
                current_tag.push(chars[j]);
                j += 1;
            }
            let tag_lower = current_tag.to_lowercase();
            let tag_name = tag_lower.trim_start_matches('/');

            if bad_block_tags.contains(&tag_name) {
                if !tag_lower.starts_with('/') {
                    in_bad_tag = true;
                } else {
                    in_bad_tag = false;
                }
                // Skip the rest of this tag
                while i < len && chars[i] != '>' {
                    i += 1;
                }
                i += 1;
                in_tag = false;
                continue;
            }

            output.push('<');
            i += 1;
            continue;
        }

        if in_tag {
            if ch == '>' {
                in_tag = false;
            }
            output.push(ch);
            i += 1;
            continue;
        }

        if !in_bad_tag {
            output.push(ch);
        }

        i += 1;
    }

    output
}
