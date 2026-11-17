pub mod chart;
pub mod summary;

use dioxus::prelude::*;
use crate::db::models::{Book, DailyReadingStats};
use crate::theme::ThemeMode;
pub use chart::StatsChart;
pub use summary::StatsSummary;

#[component]
pub fn StatsView(
    stats: Vec<DailyReadingStats>,
    books: Vec<Book>,
    theme: ThemeMode,
    on_back: EventHandler<()>,
) -> Element {
    let total_minutes: u32 = stats.iter().map(|s| s.minutes).sum();
    let daily_average = if !stats.is_empty() {
        total_minutes / (stats.len() as u32)
    } else {
        0
    };

    let mut streak = 0u32;
    for (idx, day) in stats.iter().rev().enumerate() {
        if idx == 0 && day.minutes == 0 {
            continue; // today is still in progress; keep yesterday's streak alive
        }
        if day.minutes > 0 {
            streak += 1;
        } else {
            break;
        }
    }
    let completed_books = books.iter().filter(|b| b.progress_percent >= 0.999).count();

    rsx! {
        div {
            class: "stats-view flex-1 flex flex-col w-full max-w-4xl mx-auto px-5 sm:px-8 pt-[calc(env(safe-area-inset-top,0px)+3.25rem)] pb-32 animate-fade-in transition-colors duration-300 min-w-0",

            // Top Header with Back Button
            div {
                class: "flex items-center gap-3.5 mb-8 select-none",
                button {
                    class: "w-10 h-10 rounded-full flex items-center justify-center border shadow-sm apple-tap transition-all",
                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                    onclick: move |_| on_back.call(()),
                    svg {
                        class: "w-5 h-5",
                        width: "20",
                        height: "20",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                }
                div {
                    class: "flex flex-col",
                    h1 {
                        class: "font-serif text-2xl sm:text-3xl font-medium tracking-tight",
                        style: "color: {theme.text_primary()};",
                        "Statistik Membaca"
                    }
                    p {
                        class: "font-sans text-xs sm:text-sm opacity-60",
                        style: "color: {theme.text_secondary()};",
                        "Wawasan waktu dan kebiasaan literatur Anda"
                    }
                }
            }

            // Summary Metric Cards
            StatsSummary {
                total_minutes: total_minutes,
                daily_average: daily_average,
                current_streak: streak,
                completed_books: completed_books,
                theme: theme,
            }

            // 7-Day Chart
            StatsChart {
                stats: stats,
                theme: theme,
            }

            // Reading Progress Breakdown
            div {
                class: "p-6 sm:p-7 rounded-3xl border shadow-sm flex flex-col",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",
                h3 {
                    class: "font-serif text-lg font-semibold mb-4",
                    style: "color: {theme.text_primary()};",
                    "Status Koleksi"
                }
                div {
                    class: "flex flex-col divide-y",
                    style: "border-color: {theme.border_color()};",
                    for book in books.iter().take(5) {
                        div {
                            class: "py-3 flex items-center justify-between gap-4",
                            div {
                                class: "flex flex-col min-w-0 flex-1",
                                span {
                                    class: "font-sans text-sm font-medium truncate",
                                    style: "color: {theme.text_primary()};",
                                    "{book.title}"
                                }
                                span {
                                    class: "font-sans text-xs opacity-60 truncate",
                                    style: "color: {theme.text_secondary()};",
                                    "{book.author}"
                                }
                            }
                            div {
                                class: "flex items-center gap-2",
                                div {
                                    class: "w-20 sm:w-28 h-1.5 rounded-full overflow-hidden",
                                    style: "background-color: {theme.border_color()};",
                                    div {
                                        class: "h-full rounded-full transition-all duration-300",
                                        style: "width: {book.progress_percent * 100.0}%; background-color: {theme.accent_color()};",
                                    }
                                }
                                span {
                                    class: "font-sans text-xs font-semibold w-9 text-right",
                                    style: "color: {theme.text_primary()};",
                                    "{((book.progress_percent * 100.0) as u32)}%"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
