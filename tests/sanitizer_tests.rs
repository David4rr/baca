use baca::engine::sanitizer::sanitize_html;

#[test]
fn test_sanitize_strips_scripts() {
    let input = "<p>Hello <script>alert('pwned');</script>World!</p>";
    let cleaned = sanitize_html(input);
    assert!(!cleaned.contains("<script>"));
    assert!(!cleaned.contains("alert('pwned');"));
    assert!(cleaned.contains("Hello"));
    assert!(cleaned.contains("World!"));
}

#[test]
fn test_sanitize_strips_iframes() {
    let input = "<div>Before <iframe src='https://malicious.com'>bad</iframe> After</div>";
    let cleaned = sanitize_html(input);
    assert!(!cleaned.contains("<iframe"));
    assert!(!cleaned.contains("malicious.com"));
    assert!(cleaned.contains("Before"));
    assert!(cleaned.contains("After"));
}

#[test]
fn test_sanitize_preserves_typography_markup() {
    let input = "<h1>Chapter 1</h1><p>A <em>beautiful</em> <strong>day</strong>.</p><blockquote>Quote</blockquote>";
    let cleaned = sanitize_html(input);
    assert_eq!(cleaned, input);
}
