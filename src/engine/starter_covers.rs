/// Generates bespoke luxury editorial SVG book covers (2:3 aspect ratio, 400x600).
/// Designed with Apple-standard typography, realistic spine lighting, and zero generic cliches.
pub fn generate_editorial_cover(title: &str, author: &str, subtitle: &str, theme_tag: &str) -> String {
    let (bg_color, text_color, accent_color, motif_svg) = match theme_tag {
        "psychology" => (
            "#F5EFE6",
            "#1C1917",
            "#B45309",
            r##"<circle cx="200" cy="270" r="54" fill="none" stroke="#B45309" stroke-width="1.5" stroke-dasharray="3 3"/>
               <circle cx="200" cy="270" r="42" fill="none" stroke="#1C1917" stroke-width="1"/>
               <line x1="200" y1="210" x2="200" y2="330" stroke="#B45309" stroke-width="1.5"/>
               <line x1="140" y1="270" x2="260" y2="270" stroke="#B45309" stroke-width="1.5"/>"##,
        ),
        "atomic" => (
            "#F8F7F4",
            "#111827",
            "#DC2626",
            r##"<circle cx="200" cy="270" r="50" fill="none" stroke="#DC2626" stroke-width="2"/>
               <circle cx="200" cy="270" r="32" fill="none" stroke="#111827" stroke-width="1.5" stroke-dasharray="4 4"/>
               <circle cx="200" cy="270" r="8" fill="#DC2626"/>
               <circle cx="240" cy="240" r="5" fill="#111827"/>"##,
        ),
        "sapiens" => (
            "#3B251A",
            "#FDF8F0",
            "#F59E0B",
            r##"<polygon points="200,215 245,295 155,295" fill="none" stroke="#F59E0B" stroke-width="1.5"/>
               <circle cx="200" cy="270" r="22" fill="none" stroke="#FDF8F0" stroke-width="1"/>
               <circle cx="200" cy="270" r="3" fill="#F59E0B"/>"##,
        ),
        "midnight" => (
            "#0B132B",
            "#F8FAFC",
            "#38BDF8",
            r##"<rect x="155" y="225" width="90" height="90" rx="16" fill="none" stroke="#38BDF8" stroke-width="1.5"/>
               <circle cx="200" cy="270" r="26" fill="none" stroke="#F8FAFC" stroke-width="1"/>
               <line x1="170" y1="270" x2="230" y2="270" stroke="#38BDF8" stroke-width="1"/>"##,
        ),
        _ => (
            "#0F2E20",
            "#F8FAFC",
            "#EAB308",
            r##"<polygon points="200,220 238,255 224,305 176,305 162,255" fill="none" stroke="#EAB308" stroke-width="1.5"/>"##,
        ),
    };

    let title_escaped = title.replace('\n', "<tspan x=\"200\" dy=\"32\">");

    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 600" width="400" height="600">
  <defs>
    <linearGradient id="spine-grad" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#000000" stop-opacity="0.35"/>
      <stop offset="3%" stop-color="#FFFFFF" stop-opacity="0.2"/>
      <stop offset="6%" stop-color="#000000" stop-opacity="0.1"/>
      <stop offset="10%" stop-color="#000000" stop-opacity="0"/>
    </linearGradient>
  </defs>
  <rect width="400" height="600" fill="{bg_color}"/>
  <rect x="20" y="20" width="360" height="560" fill="none" stroke="{accent_color}" stroke-opacity="0.3" stroke-width="1"/>
  <rect x="24" y="24" width="352" height="552" fill="none" stroke="{accent_color}" stroke-opacity="0.15" stroke-width="0.5"/>
  <text x="200" y="80" text-anchor="middle" font-family="-apple-system, BlinkMacSystemFont, sans-serif" font-size="10" font-weight="600" letter-spacing="4" fill="{accent_color}" opacity="0.9">INTERNATIONAL BESTSELLER</text>
  <text x="200" y="145" text-anchor="middle" font-family="Newsreader, Playfair Display, Georgia, serif" font-size="28" font-weight="600" fill="{text_color}" letter-spacing="-0.5">
    {title_escaped}
  </text>
  <text x="200" y="200" text-anchor="middle" font-family="-apple-system, BlinkMacSystemFont, sans-serif" font-size="11" fill="{text_color}" opacity="0.7" letter-spacing="0.5">
    {subtitle}
  </text>
  <g transform="translate(0, 30)">
    {motif_svg}
  </g>
  <line x1="160" y1="480" x2="240" y2="480" stroke="{accent_color}" stroke-opacity="0.5" stroke-width="1"/>
  <text x="200" y="520" text-anchor="middle" font-family="-apple-system, BlinkMacSystemFont, sans-serif" font-size="12" font-weight="600" letter-spacing="3" fill="{text_color}">
    {author}
  </text>
  <rect x="0" y="0" width="40" height="600" fill="url(#spine-grad)"/>
</svg>"##
    );

    format!("data:image/svg+xml;utf8,{}", urlencoding::encode(&svg))
}
