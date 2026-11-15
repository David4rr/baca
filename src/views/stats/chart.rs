use dioxus::prelude::*;
use crate::db::models::DailyReadingStats;
use crate::theme::ThemeMode;

#[component]
pub fn StatsChart(
    stats: Vec<DailyReadingStats>,
    theme: ThemeMode,
) -> Element {
    let max_min = stats.iter().map(|s| s.minutes).max().unwrap_or(1).max(30);

    rsx! {
        div {
            class: "p-6 sm:p-7 rounded-3xl border shadow-sm mb-8 flex flex-col",
            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",

            div {
                class: "flex items-center justify-between mb-6",
                div {
                    class: "flex flex-col",
                    h3 {
                        class: "font-serif text-lg font-semibold",
                        style: "color: {theme.text_primary()};",
                        "Aktivitas 7 Hari Terakhir"
                    }
                    span {
                        class: "text-xs font-sans opacity-60 mt-0.5",
                        style: "color: {theme.text_secondary()};",
                        "Durasi membaca harian dalam menit"
                    }
                }
            }

            // Bar Chart Area
            div {
                class: "flex items-end justify-between gap-2 sm:gap-4 h-44 sm:h-52 pt-4 px-2 select-none",
                for (idx, stat) in stats.iter().enumerate() {
                    {
                        let height_pct = ((stat.minutes as f32 / max_min as f32) * 100.0).clamp(6.0, 100.0);
                        let is_today = idx == stats.len().saturating_sub(1);
                        let bar_bg = if is_today {
                            theme.accent_color().to_string()
                        } else if stat.minutes > 0 {
                            format!("{}99", theme.accent_color())
                        } else {
                            format!("{}20", theme.text_secondary())
                        };

                        rsx! {
                            div {
                                class: "flex-1 flex flex-col items-center h-full justify-end gap-2 group",
                                // Tooltip on hover
                                span {
                                    class: "text-[11px] font-sans font-medium opacity-70 group-hover:opacity-100 transition-opacity",
                                    style: "color: {theme.text_secondary()};",
                                    if stat.minutes > 0 { "{stat.minutes}m" } else { "-" }
                                }
                                // Bar
                                div {
                                    class: "w-full max-w-[36px] rounded-t-xl transition-all duration-500 ease-out",
                                    style: "height: {height_pct}%; background-color: {bar_bg};",
                                }
                                // Day Label
                                span {
                                    class: "text-xs font-sans font-medium tracking-tight mt-1",
                                    style: if is_today {
                                        format!("color: {}; font-weight: 700;", theme.accent_color())
                                    } else {
                                        format!("color: {}; opacity: 0.7;", theme.text_secondary())
                                    },
                                    "{stat.day_label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
