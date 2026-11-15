use dioxus::prelude::*;
use crate::db::models::{Book, UserSettings};
use crate::engine::EpubBook;
use crate::theme::ThemeMode;

#[component]
pub fn ReaderView(
    book: Book,
    epub: EpubBook,
    current_chapter: usize,
    theme: ThemeMode,
    settings: UserSettings,
    on_change_chapter: EventHandler<usize>,
    on_back: EventHandler<()>,
    on_toggle_typography: EventHandler<()>,
    on_toggle_toc: EventHandler<()>,
    on_toggle_search: EventHandler<()>,
    on_toggle_ai: EventHandler<()>,
    on_change_theme: EventHandler<String>,
) -> Element {
    let total_chapters = epub.chapters.len();
    let current_ch = epub.chapters.get(current_chapter);

    let progress_percent = if total_chapters > 0 {
        (((current_chapter as f32) / (total_chapters as f32)) * 100.0).round() as u32
    } else {
        0
    };

    let est_minutes_left = if total_chapters > 0 {
        ((total_chapters - current_chapter) * 4).max(1)
    } else {
        0
    };

    let font_family_css = match settings.font_family.as_str() {
        "sans" => "'Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
        "mono" => "'JetBrains Mono', ui-monospace, SFMono-Regular, monospace",
        _ => "'Newsreader', 'Playfair Display', Georgia, serif",
    };

    rsx! {
        div {
            class: "reader-screen flex-1 flex flex-col items-center relative overflow-y-auto min-h-screen selection:bg-amber-200 transition-colors duration-300",
            style: "background-color: {theme.bg_color()}; color: {theme.text_primary()};",

            // 1. Floating Apple Top Navigation Bar (matching reference)
            div {
                class: "sticky top-0 z-30 w-full max-w-3xl mx-auto px-6 py-4 flex items-center justify-between pointer-events-auto backdrop-blur-md transition-all duration-300",
                style: "background-color: {theme.bg_color()}CC;",

                // Circular Back Button (<)
                button {
                    class: "w-10 h-10 rounded-full flex items-center justify-center shadow-sm border apple-tap transition-all",
                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    title: "Kembali ke Rak Buku",
                    onclick: move |_| on_back.call(()),
                    svg {
                        class: "w-4 h-4",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                }

                // Top Right Action Cluster: Aa, TOC, Search, AI (matching reference)
                div {
                    class: "flex items-center gap-2",

                    // In-Book Search
                    button {
                        class: "w-9 h-9 rounded-full flex items-center justify-center apple-tap opacity-70 hover:opacity-100 transition-all",
                        style: "color: {theme.text_primary()};",
                        title: "Cari Teks dalam Bab",
                        onclick: move |_| on_toggle_search.call(()),
                        svg {
                            class: "w-4 h-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "11", cy: "11", r: "8" }
                            line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                        }
                    }

                    // Gemini AI
                    button {
                        class: "w-9 h-9 rounded-full flex items-center justify-center apple-tap opacity-70 hover:opacity-100 transition-all",
                        style: "color: {theme.text_primary()};",
                        title: "Tanya AI (Gemini)",
                        onclick: move |_| on_toggle_ai.call(()),
                        svg {
                            class: "w-4 h-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z" }
                        }
                    }

                    // Typography & Appearance ("Aa")
                    button {
                        class: "w-9 h-9 rounded-full flex items-center justify-center apple-tap font-serif text-[15px] font-medium opacity-80 hover:opacity-100 transition-all",
                        style: "color: {theme.text_primary()};",
                        title: "Pengaturan Tipografi & Tampilan",
                        onclick: move |_| on_toggle_typography.call(()),
                        "Aa"
                    }

                    // Table of Contents (≡)
                    button {
                        class: "w-9 h-9 rounded-full flex items-center justify-center apple-tap opacity-70 hover:opacity-100 transition-all",
                        style: "color: {theme.text_primary()};",
                        title: "Daftar Bab & Isi",
                        onclick: move |_| on_toggle_toc.call(()),
                        svg {
                            class: "w-4 h-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line { x1: "8", y1: "6", x2: "21", y2: "6" }
                            line { x1: "8", y1: "12", x2: "21", y2: "12" }
                            line { x1: "8", y1: "18", x2: "21", y2: "18" }
                            line { x1: "3", y1: "6", x2: "3.01", y2: "6" }
                            line { x1: "3", y1: "12", x2: "3.01", y2: "12" }
                            line { x1: "3", y1: "18", x2: "3.01", y2: "18" }
                        }
                    }
                }
            }

            // 2. Main Editorial Reading Canvas (matching reference)
            main {
                class: "reading-canvas w-full max-w-2xl px-6 sm:px-8 pt-4 pb-48 flex flex-col transition-all duration-300",
                style: "font-family: {font_family_css}; font-size: {settings.font_size}px; line-height: {settings.line_height};",

                if let Some(ch) = current_ch {
                    div {
                        class: "chapter-content space-y-6 select-text animate-fade-in",

                        // Chapter Eyebrow ("CHAPTER 5")
                        div {
                            class: "text-center mb-2",
                            span {
                                class: "font-sans text-[11px] uppercase tracking-[0.25em] font-semibold opacity-60",
                                style: "color: {theme.text_secondary()};",
                                "CHAPTER {current_chapter + 1}"
                            }
                        }

                        // Chapter Headline ("The Wealth Equation")
                        h1 {
                            class: "font-serif text-[30px] sm:text-[36px] font-normal tracking-tight text-center mb-8 leading-[1.2]",
                            style: "color: {theme.text_primary()};",
                            "{ch.title}"
                        }

                        // Chapter Body Text (Sanitized HTML)
                        div {
                            class: "prose max-w-none leading-relaxed text-justify",
                            dangerous_inner_html: "{ch.html_content}",
                        }

                        // Floating Highlight Action Pill (matching reference "Highlights")
                        div {
                            class: "flex justify-end pt-4 pb-2",
                            button {
                                class: "rounded-full px-4 py-1.5 text-xs font-sans font-medium flex items-center gap-1.5 shadow-sm border apple-tap transition-all",
                                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_secondary()};",
                                onclick: move |_| on_toggle_ai.call(()),
                                svg {
                                    class: "w-3 h-3 text-amber-500",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2.5",
                                    path { d: "m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z" }
                                }
                                span { "Tanya AI" }
                            }
                        }
                    }
                } else {
                    div {
                        class: "p-12 text-center opacity-60",
                        "Bab buku tidak dapat dimuat."
                    }
                }

                // Next/Previous Chapter Navigators
                div {
                    class: "mt-16 pt-8 border-t flex items-center justify-between select-none",
                    style: "border-color: {theme.border_color()};",

                    button {
                        class: "px-4 py-2 rounded-xl border text-xs font-sans font-medium apple-tap transition-all disabled:opacity-20 disabled:pointer-events-none",
                        style: "border-color: {theme.border_color()}; background-color: {theme.surface_color()}; color: {theme.text_primary()};",
                        disabled: current_chapter == 0,
                        onclick: move |_| {
                            if current_chapter > 0 {
                                on_change_chapter.call(current_chapter - 1);
                            }
                        },
                        "← Bab Sebelumnya"
                    }

                    button {
                        class: "px-4 py-2 rounded-xl border text-xs font-sans font-medium apple-tap transition-all disabled:opacity-20 disabled:pointer-events-none",
                        style: "border-color: {theme.border_color()}; background-color: {theme.surface_color()}; color: {theme.text_primary()};",
                        disabled: current_chapter + 1 >= total_chapters,
                        onclick: move |_| {
                            if current_chapter + 1 < total_chapters {
                                on_change_chapter.call(current_chapter + 1);
                            }
                        },
                        "Bab Selanjutnya →"
                    }
                }
            }

            // 3. Bottom Scrubber & Floating Apple Segmented Dock (matching reference)
            div {
                class: "fixed bottom-0 left-0 right-0 z-30 pointer-events-none flex flex-col items-center pb-6 px-6",

                // Scrubber Bar Container
                div {
                    class: "w-full max-w-xl pointer-events-auto rounded-[24px] px-6 py-3.5 mb-3 backdrop-blur-2xl border shadow-lg transition-all duration-300",
                    style: "background-color: {theme.surface_color()}E6; border-color: {theme.border_color()}; box-shadow: 0 12px 32px rgba(0,0,0,0.06);",

                    // Fine scrubber slider
                    div {
                        class: "w-full relative h-3 flex items-center cursor-pointer",
                        div {
                            class: "w-full h-1 rounded-full",
                            style: "background-color: {theme.surface_subtle()};",
                            div {
                                class: "h-full rounded-full transition-all duration-300",
                                style: "width: {progress_percent}%; background-color: {theme.accent_color()};",
                            }
                        }
                    }

                    // Scrubber labels (65% | Chapter 5 of 12 | 14 min left)
                    div {
                        class: "flex items-center justify-between text-[11px] font-sans font-medium mt-1",
                        span {
                            style: "color: {theme.text_muted()};",
                            "{progress_percent}%"
                        }
                        span {
                            style: "color: {theme.text_secondary()};",
                            "Chapter {current_chapter + 1} of {total_chapters}"
                        }
                        span {
                            style: "color: {theme.text_muted()};",
                            "{est_minutes_left} min left"
                        }
                    }
                }

                // Apple Segmented Control Theme Dock (Sun / Sepia / Moon)
                div {
                    class: "pointer-events-auto rounded-full p-1 flex items-center gap-1 backdrop-blur-2xl border shadow-md transition-all duration-300",
                    style: "background-color: {theme.surface_color()}EE; border-color: {theme.border_color()};",

                    // Sun (Light)
                    button {
                        class: "w-9 h-7 rounded-full flex items-center justify-center apple-tap transition-all",
                        style: if theme == ThemeMode::Light {
                            format!("background-color: {}; color: {}; shadow-sm", theme.text_primary(), theme.bg_color())
                        } else {
                            format!("color: {}; opacity: 0.6;", theme.text_secondary())
                        },
                        title: "Mode Terang",
                        onclick: move |_| on_change_theme.call("light".to_string()),
                        svg {
                            class: "w-4 h-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "12", cy: "12", r: "5" }
                            line { x1: "12", y1: "1", x2: "12", y2: "3" }
                            line { x1: "12", y1: "21", x2: "12", y2: "23" }
                            line { x1: "4.22", y1: "4.22", x2: "5.64", y2: "5.64" }
                            line { x1: "18.36", y1: "18.36", x2: "19.78", y2: "19.78" }
                            line { x1: "1", y1: "12", x2: "3", y2: "12" }
                            line { x1: "21", y1: "12", x2: "23", y2: "12" }
                        }
                    }

                    // Book (Warm Editorial - Default)
                    button {
                        class: "w-9 h-7 rounded-full flex items-center justify-center apple-tap transition-all",
                        style: if theme == ThemeMode::Warm {
                            format!("background-color: {}; color: {}; shadow-sm", theme.text_primary(), theme.bg_color())
                        } else {
                            format!("color: {}; opacity: 0.6;", theme.text_secondary())
                        },
                        title: "Mode Editorial Hangat",
                        onclick: move |_| on_change_theme.call("warm".to_string()),
                        svg {
                            class: "w-4 h-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M4 19.5A2.5 2.5 0 0 1 6.5 17H20" }
                            path { d: "M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" }
                        }
                    }

                    // Moon (AMOLED Dark)
                    button {
                        class: "w-9 h-7 rounded-full flex items-center justify-center apple-tap transition-all",
                        style: if theme == ThemeMode::Amoled {
                            format!("background-color: {}; color: {}; shadow-sm", theme.text_primary(), theme.bg_color())
                        } else {
                            format!("color: {}; opacity: 0.6;", theme.text_secondary())
                        },
                        title: "Mode Gelap OLED",
                        onclick: move |_| on_change_theme.call("amoled".to_string()),
                        svg {
                            class: "w-4 h-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" }
                        }
                    }
                }
            }
        }
    }
}
