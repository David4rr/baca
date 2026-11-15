use dioxus::prelude::*;
use crate::db::models::DailyReadingStats;
use crate::theme::ThemeMode;

#[component]
pub fn StatsModal(
    theme: ThemeMode,
    stats: Vec<DailyReadingStats>,
    total_books: usize,
    total_bookmarks: usize,
    is_open: bool,
    on_close: EventHandler<()>,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    let total_minutes: u32 = stats.iter().map(|s| s.minutes).sum();
    let max_minutes = stats.iter().map(|s| s.minutes).max().unwrap_or(1).max(10);

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-sm animate-fade-in",
            onclick: move |_| on_close.call(()),

            div {
                class: "w-full max-w-md rounded-[28px] border shadow-2xl flex flex-col overflow-hidden animate-slide-up",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()}; box-shadow: {theme.card_shadow()};",
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "p-5 border-b flex items-center justify-between",
                    style: "border-color: {theme.border_color()};",
                    div {
                        class: "flex items-center gap-2.5",
                        div {
                            class: "w-8 h-8 rounded-full flex items-center justify-center",
                            style: "width: 32px; height: 32px; min-width: 32px; min-height: 32px; background-color: {theme.surface_subtle()}; color: {theme.text_primary()};",
                            svg {
                                class: "w-4 h-4",
                                width: "16",
                                height: "16",
                                style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.2",
                                line { x1: "18", y1: "20", x2: "18", y2: "10" }
                                line { x1: "12", y1: "20", x2: "12", y2: "4" }
                                line { x1: "6", y1: "20", x2: "6", y2: "14" }
                            }
                        }
                        div {
                            h3 { class: "font-serif text-lg font-medium", "Statistik Membaca" }
                            p { class: "font-sans text-[11px] opacity-60", "Aktivitas 7 hari terakhir" }
                        }
                    }
                    button {
                        class: "w-8 h-8 rounded-full flex items-center justify-center apple-tap opacity-60 hover:opacity-100 transition-all",
                        style: "width: 32px; height: 32px; min-width: 32px; min-height: 32px; background-color: {theme.surface_subtle()}; color: {theme.text_primary()};",
                        onclick: move |_| on_close.call(()),
                        svg {
                            class: "w-4 h-4",
                            width: "16",
                            height: "16",
                            style: "width: 16px; height: 16px; min-width: 16px; min-height: 16px;",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            line { x1: "18", y1: "6", x2: "6", y2: "18" }
                            line { x1: "6", y1: "6", x2: "18", y2: "18" }
                        }
                    }
                }

                // Metric Cards Summary (Apple clean bento)
                div {
                    class: "p-5 grid grid-cols-3 gap-3 text-center border-b",
                    style: "border-color: {theme.border_color()};",

                    div {
                        class: "p-3.5 rounded-2xl border flex flex-col justify-center",
                        style: "background-color: {theme.surface_subtle()}; border-color: {theme.border_color()};",
                        span { class: "font-serif text-2xl font-bold tracking-tight", "{total_minutes}" }
                        span { class: "font-sans text-[10px] opacity-60 uppercase font-semibold mt-1", "Total Menit" }
                    }
                    div {
                        class: "p-3.5 rounded-2xl border flex flex-col justify-center",
                        style: "background-color: {theme.surface_subtle()}; border-color: {theme.border_color()};",
                        span { class: "font-serif text-2xl font-bold tracking-tight", "{total_books}" }
                        span { class: "font-sans text-[10px] opacity-60 uppercase font-semibold mt-1", "Buku di Rak" }
                    }
                    div {
                        class: "p-3.5 rounded-2xl border flex flex-col justify-center",
                        style: "background-color: {theme.surface_subtle()}; border-color: {theme.border_color()};",
                        span { class: "font-serif text-2xl font-bold tracking-tight", "{total_bookmarks}" }
                        span { class: "font-sans text-[10px] opacity-60 uppercase font-semibold mt-1", "Catatan" }
                    }
                }

                // 7-Day Visual Bar Chart
                div {
                    class: "p-6 flex flex-col gap-3",
                    span { class: "font-sans text-[11px] font-semibold uppercase tracking-wider opacity-50", "DURASI HARIAN (MENIT)" }

                    div {
                        class: "h-36 flex items-end justify-between gap-2.5 pt-4 pb-2 border-b",
                        style: "border-color: {theme.border_color()};",
                        for day in stats.iter() {
                            {
                                let height_pct = if max_minutes > 0 {
                                    ((day.minutes as f32 / max_minutes as f32) * 100.0).max(4.0) as u32
                                } else {
                                    4
                                };
                                let is_today = day.minutes > 0;
                                let day_label = if day.day_label.is_empty() { &day.date_str } else { &day.day_label };

                                rsx! {
                                    div {
                                        key: "{day.date_str}",
                                        class: "flex-1 flex flex-col items-center gap-1.5 h-full justify-end group",
                                        div {
                                            class: "text-[10px] font-mono opacity-0 group-hover:opacity-100 transition-opacity",
                                            "{day.minutes}m"
                                        }
                                        div {
                                            class: "w-full rounded-t-lg transition-all duration-500",
                                            style: if is_today {
                                                format!("height: {}%; background-color: {};", height_pct, theme.accent_color())
                                            } else {
                                                format!("height: {}%; background-color: {}; opacity: 0.3;", height_pct, theme.text_secondary())
                                            },
                                        }
                                        span {
                                            class: "text-[10px] font-sans opacity-60 truncate w-full text-center mt-1",
                                            "{day_label}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
