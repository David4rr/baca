use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn StatsSummary(
    total_minutes: u32,
    daily_average: u32,
    current_streak: u32,
    completed_books: usize,
    theme: ThemeMode,
) -> Element {
    let hours = total_minutes / 60;
    let mins = total_minutes % 60;
    let time_display = if hours > 0 {
        format!("{}j {}m", hours, mins)
    } else {
        format!("{}m", mins)
    };

    rsx! {
        div {
            class: "grid grid-cols-2 sm:grid-cols-4 gap-3 sm:gap-4 mb-8",

            // 1. Total Reading Time
            div {
                class: "p-4 sm:p-5 rounded-2xl border flex flex-col justify-between shadow-sm",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",
                span {
                    class: "text-xs font-sans font-medium tracking-tight opacity-60",
                    style: "color: {theme.text_secondary()};",
                    "Total Waktu"
                }
                span {
                    class: "font-serif text-2xl sm:text-3xl font-bold mt-2",
                    style: "color: {theme.text_primary()};",
                    "{time_display}"
                }
            }

            // 2. Daily Average
            div {
                class: "p-4 sm:p-5 rounded-2xl border flex flex-col justify-between shadow-sm",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",
                span {
                    class: "text-xs font-sans font-medium tracking-tight opacity-60",
                    style: "color: {theme.text_secondary()};",
                    "Rata-rata Harian"
                }
                span {
                    class: "font-serif text-2xl sm:text-3xl font-bold mt-2",
                    style: "color: {theme.text_primary()};",
                    "{daily_average}m"
                }
            }

            // 3. Streak
            div {
                class: "p-4 sm:p-5 rounded-2xl border flex flex-col justify-between shadow-sm",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",
                span {
                    class: "text-xs font-sans font-medium tracking-tight opacity-60",
                    style: "color: {theme.text_secondary()};",
                    "Streak Membaca"
                }
                div {
                    class: "flex items-baseline gap-1 mt-2",
                    span {
                        class: "font-serif text-2xl sm:text-3xl font-bold",
                        style: "color: {theme.accent_color()};",
                        "{current_streak}"
                    }
                    span {
                        class: "text-xs font-sans opacity-60",
                        style: "color: {theme.text_secondary()};",
                        "hari"
                    }
                }
            }

            // 4. Completed Books
            div {
                class: "p-4 sm:p-5 rounded-2xl border flex flex-col justify-between shadow-sm",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",
                span {
                    class: "text-xs font-sans font-medium tracking-tight opacity-60",
                    style: "color: {theme.text_secondary()};",
                    "Buku Selesai"
                }
                span {
                    class: "font-serif text-2xl sm:text-3xl font-bold mt-2",
                    style: "color: {theme.text_primary()};",
                    "{completed_books}"
                }
            }
        }
    }
}
