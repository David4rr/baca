pub mod ai_section;
pub mod backup_section;
pub mod theme_section;
pub mod update_section;

use dioxus::prelude::*;
use crate::db::models::UserSettings;
use crate::engine::updater::ReleaseInfo;
use crate::theme::ThemeMode;
pub use ai_section::AiSection;
pub use backup_section::BackupSection;
pub use theme_section::ThemeSection;
pub use update_section::UpdateSection;

#[component]
pub fn SettingsView(
    settings: UserSettings,
    theme: ThemeMode,
    on_change_settings: EventHandler<UserSettings>,
    on_export_backup: EventHandler<()>,
    on_restore_backup: EventHandler<String>,
    on_show_update_modal: EventHandler<ReleaseInfo>,
    on_back: EventHandler<()>,
) -> Element {
    let settings_for_theme = settings.clone();
    let settings_for_ai = settings.clone();
    let current_version = env!("CARGO_PKG_VERSION");

    rsx! {
        div {
            class: "settings-view flex-1 flex flex-col w-full max-w-4xl mx-auto px-5 sm:px-8 pt-8 pb-32 animate-fade-in transition-colors duration-300",

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
                        "Pengaturan"
                    }
                    p {
                        class: "font-sans text-xs sm:text-sm opacity-60",
                        style: "color: {theme.text_secondary()};",
                        "Personalisasi pengalaman membaca dan integrasi"
                    }
                }
            }

            div {
                class: "flex flex-col gap-6",

                // 1. Theme Configuration
                ThemeSection {
                    current_theme: theme,
                    on_select_theme: move |theme_str| {
                        let mut s = settings_for_theme.clone();
                        s.theme = theme_str;
                        on_change_settings.call(s);
                    },
                }

                // 2. AI Key & Model
                AiSection {
                    api_key: settings.gemini_api_key.clone(),
                    model: settings.gemini_model.clone(),
                    theme: theme,
                    on_save: move |(key, model): (String, String)| {
                        let mut s = settings_for_ai.clone();
                        s.gemini_api_key = if key.is_empty() { None } else { Some(key) };
                        s.gemini_model = Some(model);
                        on_change_settings.call(s);
                    },
                }

                // 3. Backup & Restore
                BackupSection {
                    theme: theme,
                    on_export: move |_| on_export_backup.call(()),
                    on_restore: move |json| on_restore_backup.call(json),
                }

                // 4. Auto-Update Section
                UpdateSection {
                    theme: theme,
                    on_show_modal: move |rel| on_show_update_modal.call(rel),
                }

                // 5. About App
                div {
                    class: "p-6 rounded-3xl border shadow-sm flex items-center justify-between",
                    style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",
                    div {
                        class: "flex flex-col",
                        span {
                            class: "font-serif text-base font-semibold",
                            style: "color: {theme.text_primary()};",
                            "Baca — Minimalist Apple Reader"
                        }
                        span {
                            class: "font-sans text-xs opacity-60 mt-0.5",
                            style: "color: {theme.text_secondary()};",
                            "Dibuat dengan Pure Rust & Dioxus • Versi {current_version}"
                        }
                    }
                    span {
                        class: "px-3 py-1 rounded-full text-xs font-mono font-medium border",
                        style: "border-color: {theme.border_color()}; color: {theme.text_secondary()};",
                        "v{current_version}"
                    }
                }
            }
        }
    }
}
