use dioxus::prelude::*;
use crate::engine::updater::{check_latest_release, ReleaseInfo};
use crate::theme::ThemeMode;

#[component]
pub fn UpdateSection(
    theme: ThemeMode,
    on_show_modal: EventHandler<ReleaseInfo>,
) -> Element {
    let mut is_checking = use_signal(|| false);
    let mut status_msg = use_signal(|| Option::<String>::None);
    let mut available_release = use_signal(|| Option::<ReleaseInfo>::None);

    let current_version = env!("CARGO_PKG_VERSION");

    let check_updates = move |_| {
        if *is_checking.read() {
            return;
        }
        is_checking.set(true);
        status_msg.set(None);
        available_release.set(None);

        spawn(async move {
            let res = check_latest_release(current_version).await;
            is_checking.set(false);
            match res {
                Ok(Some(release)) => {
                    let tag = release.tag_name.clone();
                    available_release.set(Some(release));
                    status_msg.set(Some(format!("Versi baru {} tersedia!", tag)));
                }
                Ok(None) => {
                    status_msg.set(Some(format!(
                        "Anda telah menggunakan versi terbaru (v{}).",
                        current_version
                    )));
                }
                Err(e) => {
                    status_msg.set(Some(format!("Gagal memeriksa: {}", e)));
                }
            }
        });
    };

    rsx! {
        div {
            class: "p-6 rounded-3xl border shadow-sm flex flex-col gap-4",
            style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",

            div {
                class: "flex items-center justify-between",
                div {
                    class: "flex flex-col",
                    h3 {
                        class: "font-serif text-lg font-semibold",
                        style: "color: {theme.text_primary()};",
                        "Pembaruan Aplikasi"
                    }
                    span {
                        class: "text-xs font-sans opacity-60 mt-0.5",
                        style: "color: {theme.text_secondary()};",
                        "Periksa versi rilis terbaru secara otomatis"
                    }
                }
                span {
                    class: "px-3 py-1 rounded-full text-xs font-mono font-medium border",
                    style: "border-color: {theme.border_color()}; color: {theme.text_secondary()};",
                    "v{current_version}"
                }
            }

            // Status Card or Update Available Card
            if let Some(release) = available_release.read().as_ref() {
                div {
                    class: "p-4 rounded-2xl border flex flex-col gap-3 animate-spring-pop",
                    style: "background-color: {theme.bg_color()}; border-color: {theme.accent_color()}40;",
                    div {
                        class: "flex items-center justify-between",
                        div {
                            class: "flex items-center gap-2",
                            span {
                                class: "w-2.5 h-2.5 rounded-full animate-ping",
                                style: "background-color: {theme.accent_color()};",
                            }
                            span {
                                class: "font-sans text-xs font-semibold",
                                style: "color: {theme.accent_color()};",
                                "Pembaruan Siap: {release.tag_name}"
                            }
                        }
                        span {
                            class: "text-[11px] font-sans opacity-60",
                            style: "color: {theme.text_secondary()};",
                            "{release.version}"
                        }
                    }
                    p {
                        class: "font-serif text-xs opacity-80 line-clamp-2",
                        style: "color: {theme.text_primary()};",
                        "{release.name}"
                    }
                    {
                        let rel = release.clone();
                        rsx! {
                            button {
                                class: "w-full py-2.5 rounded-xl font-sans text-xs font-semibold text-white shadow-sm flex items-center justify-center gap-2 apple-tap transition-all",
                                style: "background-color: {theme.accent_color()};",
                                onclick: move |_| on_show_modal.call(rel.clone()),
                                "Lihat Detail & Perbarui Sekarang"
                            }
                        }
                    }
                }
            } else {
                div {
                    class: "flex items-center justify-between p-3.5 rounded-2xl border",
                    style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()};",
                    div {
                        class: "flex items-center gap-2.5",
                        div {
                            class: "w-2 h-2 rounded-full bg-emerald-500",
                        }
                        span {
                            class: "text-xs font-sans opacity-70",
                            style: "color: {theme.text_primary()};",
                            if let Some(msg) = status_msg.read().as_ref() {
                                "{msg}"
                            } else {
                                "Pembaruan otomatis aktif saat aplikasi dibuka"
                            }
                        }
                    }
                    button {
                        class: "px-3.5 py-1.5 rounded-xl text-xs font-sans font-medium border shadow-sm apple-tap transition-all flex items-center gap-1.5",
                        style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                        disabled: *is_checking.read(),
                        onclick: check_updates,
                        if *is_checking.read() {
                            svg {
                                class: "animate-spin w-3.5 h-3.5",
                                width: "14",
                                height: "14",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.5",
                                circle { cx: "12", cy: "12", r: "10", stroke_opacity: "0.2" }
                                path { d: "M12 2a10 10 0 0 1 10 10", stroke_linecap: "round" }
                            }
                            "Memeriksa..."
                        } else {
                            svg {
                                class: "w-3.5 h-3.5 opacity-70",
                                width: "14",
                                height: "14",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.3" }
                            }
                            "Cek Pembaruan"
                        }
                    }
                }
            }
        }
    }
}
