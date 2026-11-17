use dioxus::prelude::*;
use crate::engine::updater::{download_update_package, install_update, ReleaseInfo};
use crate::theme::ThemeMode;

#[component]
pub fn UpdateModal(release: ReleaseInfo, theme: ThemeMode, on_close: EventHandler<()>) -> Element {
    let mut is_downloading = use_signal(|| false);
    let mut progress = use_signal(|| 0.0f32);
    let mut download_complete = use_signal(|| false);
    let mut saved_path = use_signal(|| Option::<String>::None);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut is_installing = use_signal(|| false);

    let current_ver = env!("CARGO_PKG_VERSION");
    let is_android = cfg!(target_os = "android");
    let target = if is_android {
        release.apk_url.clone().map(|u| (u, format!("baca-{}-android.apk", release.version)))
    } else {
        release.desktop_url.clone().map(|u| {
            let ext = if u.ends_with(".zip") { "zip" } else { "tar.gz" };
            (u, format!("baca-{}-{}.{}", release.version, std::env::consts::OS, ext))
        })
    };
    let has_asset = target.is_some();
    let (target_url, target_name) = target.unwrap_or_default();

    let start_download = move |_| {
        if *is_downloading.read() || target_url.is_empty() { return; }
        is_downloading.set(true);
        progress.set(0.0);
        error_msg.set(None);
        let (dl_url, fname) = (target_url.clone(), target_name.clone());
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<f32>();

        spawn(async move {
            while let Some(pct) = rx.recv().await { progress.set(pct); }
        });

        spawn(async move {
            match download_update_package(&dl_url, &fname, move |pct| { let _ = tx.send(pct); }).await {
                Ok(path) => {
                    is_downloading.set(false);
                    saved_path.set(Some(path.to_string_lossy().to_string()));
                    download_complete.set(true);
                }
                Err(e) => { is_downloading.set(false); error_msg.set(Some(e)); }
            }
        });
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-end sm:items-center justify-center p-0 sm:p-4 bg-black/60 backdrop-blur-sm animate-fade-in select-none",
            onclick: move |_| on_close.call(()),

            div {
                class: "w-full max-w-lg rounded-t-3xl sm:rounded-3xl border shadow-lg p-6 sm:p-7 flex flex-col gap-5 animate-spring-pop max-h-[90vh] overflow-y-auto",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; padding-bottom: max(calc(env(safe-area-inset-bottom, 0px) + 1.5rem), 1.75rem);",
                role: "dialog",
                aria_modal: "true",
                aria_label: "Pembaruan aplikasi",
                tabindex: "0",
                onkeydown: move |e: KeyboardEvent| {
                    if e.key() == Key::Escape { on_close.call(()); }
                },
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "flex items-start justify-between gap-3",
                    div {
                        class: "flex items-center gap-3",
                        div {
                            class: "w-11 h-11 rounded-2xl flex items-center justify-center shadow-sm",
                            style: "background-color: {theme.accent_color()}20; color: {theme.accent_color()};",
                            svg { class: "w-6 h-6", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                polyline { points: "7 10 12 15 17 10" }
                                line { x1: "12", y1: "15", x2: "12", y2: "3" }
                            }
                        }
                        div {
                            class: "flex flex-col",
                            h2 { class: "font-serif text-xl font-bold tracking-tight", style: "color: {theme.text_primary()};", "Pembaruan Tersedia" }
                            span { class: "font-sans text-xs opacity-60", style: "color: {theme.text_secondary()};", "Versi baru siap diunduh dan dipasang" }
                        }
                    }
                    button {
                        class: "w-8 h-8 rounded-full flex items-center justify-center border apple-tap opacity-70",
                        style: "border-color: {theme.border_color()}; color: {theme.text_secondary()};",
                        title: "Tutup",
                        aria_label: "Tutup pembaruan",
                        onclick: move |_| on_close.call(()),
                        svg { class: "w-4 h-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5",
                            line { x1: "18", y1: "6", x2: "6", y2: "18" }
                            line { x1: "6", y1: "6", x2: "18", y2: "18" }
                        }
                    }
                }

                // Version comparison pill
                div {
                    class: "flex items-center justify-between p-3.5 rounded-2xl border",
                    style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()};",
                    div { class: "flex items-center gap-2",
                        span { class: "text-xs font-sans opacity-70", style: "color: {theme.text_secondary()};", "Versi Sekarang:" }
                        span { class: "text-xs font-mono font-medium px-2 py-0.5 rounded-full border", style: "border-color: {theme.border_color()}; color: {theme.text_primary()};", "v{current_ver}" }
                    }
                    svg { class: "w-3.5 h-3.5 opacity-40", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5",
                        polyline { points: "9 18 15 12 9 6" }
                    }
                    div { class: "flex items-center gap-2",
                        span { class: "text-xs font-sans font-medium", style: "color: {theme.accent_color()};", "Versi Baru:" }
                        span { class: "text-xs font-mono font-bold px-2 py-0.5 rounded-full text-white shadow-sm", style: "background-color: {theme.accent_color()};", "{release.tag_name}" }
                    }
                }

                // Release Title & Changelog Box
                div {
                    class: "flex flex-col gap-2",
                    h3 { class: "font-serif text-base font-semibold", style: "color: {theme.text_primary()};", "{release.name}" }
                    if !release.body.is_empty() {
                        div { class: "p-3.5 rounded-2xl border text-xs font-sans leading-relaxed max-h-36 overflow-y-auto whitespace-pre-wrap opacity-80", style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};", "{release.body}" }
                    }
                }

                if let Some(err) = error_msg.read().as_ref() {
                    div { class: "p-3 rounded-2xl bg-rose-500/10 border border-rose-500/20 text-rose-500 text-xs font-sans", "{err}" }
                }

                // Action Controls
                div {
                    class: "flex flex-col gap-2.5 pt-2",
                    if *download_complete.read() {
                        div { class: "flex flex-col gap-2",
                            div { class: "p-3 rounded-2xl bg-emerald-500/10 border border-emerald-500/20 text-emerald-600 dark:text-emerald-400 text-xs font-medium text-center flex flex-col items-center gap-1.5",
                                div { class: "flex items-center gap-1.5 font-semibold",
                                    svg { class: "w-3.5 h-3.5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round", polyline { points: "20 6 9 17 4 12" } }
                                    span { "Berkas berhasil diunduh!" }
                                }
                                if let Some(path) = saved_path.read().as_ref() {
                                    span { class: "text-[11px] opacity-75 font-mono truncate", "Tersimpan: {path}" }
                                }
                            }
                            button {
                                class: "w-full py-3.5 rounded-2xl font-sans text-sm font-semibold flex items-center justify-center text-white shadow-md apple-tap disabled:opacity-50",
                                style: "background-color: {theme.accent_color()};",
                                disabled: *is_installing.read(),
                                onclick: move |_| {
                                    let Some(path) = saved_path.read().clone() else { return };
                                    if *is_installing.read() { return; }
                                    is_installing.set(true);
                                    error_msg.set(None);
                                    if let Err(e) = install_update(std::path::Path::new(&path)) {
                                        is_installing.set(false);
                                        error_msg.set(Some(e));
                                    }
                                },
                                if *is_installing.read() { "Memasang..." } else { "Pasang Pembaruan" }
                            }
                        }
                    } else if *is_downloading.read() {
                        div { class: "flex flex-col gap-2 p-3.5 rounded-2xl border", style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()};",
                            div { class: "flex justify-between text-xs font-sans font-medium", style: "color: {theme.text_primary()};", span { "Mengunduh Berkas Pembaruan..." } span { "{*progress.read() as u32}%" } }
                            div { class: "w-full h-2 rounded-full overflow-hidden bg-black/10 dark:bg-white/10",
                                div { class: "h-full transition-all duration-300 rounded-full", style: "width: {progress.read()}%; background-color: {theme.accent_color()};" }
                            }
                        }
                    } else {
                        button {
                            class: "w-full py-3.5 rounded-2xl font-sans text-sm font-semibold flex items-center justify-center gap-2 text-white shadow-md apple-tap disabled:opacity-50",
                            style: "background-color: {theme.accent_color()};",
                            disabled: !has_asset,
                            onclick: start_download,
                            svg { class: "w-4 h-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                polyline { points: "7 10 12 15 17 10" }
                                line { x1: "12", y1: "15", x2: "12", y2: "3" }
                            }
                            if has_asset { "Unduh Pembaruan Sekarang" } else { "Berkas pembaruan belum tersedia" }
                        }
                    }

                    p {
                        class: "text-[11px] font-sans text-center opacity-50",
                        "Berkas diverifikasi dengan SHA-256 sebelum dipasang."
                    }
                }
            }
        }
    }
}
