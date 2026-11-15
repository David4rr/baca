use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn ReaderScrubber(
    progress_percent: u32,
    current_chapter: usize,
    total_chapters: usize,
    est_minutes_left: usize,
    theme: ThemeMode,
    on_change_theme: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            class: "fixed bottom-0 left-0 right-0 z-30 pointer-events-none flex flex-col items-center pb-6 px-6 select-none",

            // 1. Apple Scrubber Bar Container
            div {
                class: "w-full max-w-xl pointer-events-auto rounded-[24px] px-6 py-3.5 mb-3 backdrop-blur-2xl border shadow-lg transition-all duration-300",
                style: "background-color: {theme.surface_color()}E6; border-color: {theme.border_color()}; box-shadow: 0 12px 32px rgba(0,0,0,0.06);",

                // Fine Scrubber Track
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

                // Scrubber Labels (65% | Chapter 5 of 12 | 14 min left)
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

            // 2. Apple Segmented Control Theme Dock (Sun / Warm / Dark)
            div {
                class: "pointer-events-auto rounded-full p-1 flex items-center gap-1 backdrop-blur-2xl border shadow-md transition-all duration-300",
                style: "background-color: {theme.surface_color()}EE; border-color: {theme.border_color()};",

                // Sun (Pure Light)
                button {
                    class: "w-9 h-7 rounded-full flex items-center justify-center apple-tap transition-all",
                    style: if theme == ThemeMode::Light {
                        format!("background-color: {}; color: {}; box-shadow: 0 2px 8px rgba(0,0,0,0.12);", theme.text_primary(), theme.bg_color())
                    } else {
                        format!("color: {}; opacity: 0.6;", theme.text_secondary())
                    },
                    title: "Mode Terang",
                    onclick: move |_| on_change_theme.call("light".to_string()),
                    svg {
                        class: "w-4 h-4",
                        width: "16",
                        height: "16",
                        style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        circle { cx: "12", cy: "12", r: "5" }
                        line { x1: "12", y1: "1", x2: "12", y2: "3" }
                        line { x1: "12", y1: "21", x2: "12", y2: "23" }
                        line { x1: "4.22", y1: "4.22", x2: "5.64", y2: "5.64" }
                        line { x1: "18.36", y1: "18.36", x2: "19.78", y2: "19.78" }
                        line { x1: "1", y1: "12", x2: "3", y2: "12" }
                        line { x1: "21", y1: "12", x2: "23", y2: "12" }
                    }
                }

                // Warm Paper (Sepia / Linen)
                button {
                    class: "w-9 h-7 rounded-full flex items-center justify-center apple-tap transition-all",
                    style: if theme == ThemeMode::Warm {
                        format!("background-color: {}; color: {}; box-shadow: 0 2px 8px rgba(0,0,0,0.12);", theme.text_primary(), theme.bg_color())
                    } else {
                        format!("color: {}; opacity: 0.6;", theme.text_secondary())
                    },
                    title: "Mode Kertas Hangat",
                    onclick: move |_| on_change_theme.call("warm".to_string()),
                    svg {
                        class: "w-4 h-4",
                        width: "16",
                        height: "16",
                        style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path { d: "M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" }
                        path { d: "M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z" }
                    }
                }

                // Dark AMOLED (Moon)
                button {
                    class: "w-9 h-7 rounded-full flex items-center justify-center apple-tap transition-all",
                    style: if theme == ThemeMode::Amoled {
                        format!("background-color: {}; color: {}; box-shadow: 0 2px 8px rgba(0,0,0,0.12);", theme.text_primary(), theme.bg_color())
                    } else {
                        format!("color: {}; opacity: 0.6;", theme.text_secondary())
                    },
                    title: "Mode Gelap OLED",
                    onclick: move |_| on_change_theme.call("amoled".to_string()),
                    svg {
                        class: "w-4 h-4",
                        width: "16",
                        height: "16",
                        style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path { d: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" }
                    }
                }
            }
        }
    }
}
