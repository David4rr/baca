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
            class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-fade-in",
            onclick: move |_| on_close.call(()),

            div {
                class: "w-full max-w-md rounded-3xl border shadow-2xl flex flex-col overflow-hidden",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "p-4 border-b flex items-center justify-between",
                    style: "border-color: {theme.border_color()};",
                    div {
                        class: "flex items-center gap-2",
                        span { class: "text-lg", "📊" }
                        h3 { class: "font-bold text-sm", "Statistik Aktivitas Membaca" }
                    }
                    button {
                        class: "p-1.5 rounded-full hover:bg-white/10 opacity-60 hover:opacity-100 text-sm font-bold",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                // Metric Cards Summary
                div {
                    class: "p-5 grid grid-cols-3 gap-3 text-center border-b",
                    style: "border-color: {theme.border_color()};",

                    div {
                        class: "p-3 rounded-2xl bg-black/20 flex flex-col",
                        span { class: "text-xl font-extrabold", style: "color: {theme.accent_color()};", "{total_minutes}" }
                        span { class: "text-[10px] opacity-60 uppercase font-semibold mt-0.5", "Menit (7 Hari)" }
                    }
                    div {
                        class: "p-3 rounded-2xl bg-black/20 flex flex-col",
                        span { class: "text-xl font-extrabold", style: "color: {theme.accent_color()};", "{total_books}" }
                        span { class: "text-[10px] opacity-60 uppercase font-semibold mt-0.5", "Buku di Rak" }
                    }
                    div {
                        class: "p-3 rounded-2xl bg-black/20 flex flex-col",
                        span { class: "text-xl font-extrabold", style: "color: {theme.accent_color()};", "{total_bookmarks}" }
                        span { class: "text-[10px] opacity-60 uppercase font-semibold mt-0.5", "Catatan" }
                    }
                }

                // 7-Day Visual Bar Chart
                div {
                    class: "p-5",
                    h4 { class: "text-xs font-bold uppercase tracking-wider opacity-60 mb-4", "Aktivitas 7 Hari Terakhir" }

                    div {
                        class: "h-36 flex items-end justify-between gap-2 pt-6 pb-2 px-2 border-b",
                        style: "border-color: {theme.border_color()};",

                        for day in stats.iter() {
                            {
                                let height_pct = ((day.minutes as f32 / max_minutes as f32) * 100.0).max(6.0) as u32;
                                let is_today = day == stats.last().unwrap_or(day);

                                rsx! {
                                    div {
                                        key: "{day.date_str}",
                                        class: "flex-1 flex flex-col items-center gap-1.5 h-full justify-end group relative",

                                        // Tooltip on hover
                                        span {
                                            class: "opacity-0 group-hover:opacity-100 transition-opacity absolute -top-5 text-[9px] font-bold px-1 py-0.5 rounded bg-black/80 text-white pointer-events-none whitespace-nowrap",
                                            "{day.minutes} m"
                                        }

                                        // Bar
                                        div {
                                            class: "w-full max-w-[28px] rounded-t-lg transition-all duration-300",
                                            style: if is_today {
                                                format!("height: {}%; background-color: {};", height_pct, theme.accent_color())
                                            } else {
                                                format!("height: {}%; background-color: rgba(255, 255, 255, 0.2);", height_pct)
                                            },
                                        }

                                        // Day label
                                        span {
                                            class: "text-[10px] font-semibold opacity-60",
                                            "{day.day_label}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    p { class: "text-[11px] opacity-40 text-center mt-3", "Waktu membaca dicatat otomatis secara lokal setiap kali membuka bab." }
                }
            }
        }
    }
}
