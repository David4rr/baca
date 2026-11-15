use dioxus::prelude::*;
use crate::engine::updater::{download_update_package, ReleaseInfo};
use crate::theme::ThemeMode;

#[component]
pub fn UpdateModal(release: ReleaseInfo, theme: ThemeMode, on_close: EventHandler<()>) -> Element {
    let mut is_downloading = use_signal(|| false);
    let mut progress = use_signal(|| 0.0f32);
    let mut download_complete = use_signal(|| false);
    let mut saved_path = use_signal(|| Option::<String>::None);
    let mut error_msg = use_signal(|| Option::<String>::None);

    let current_ver = env!("CARGO_PKG_VERSION");
    let is_android = cfg!(target_os = "android");
    let (target_url, target_name) = if is_android && release.apk_url.is_some() {
        (release.apk_url.clone().unwrap(), format!("baca-{}-android.apk", release.version))
    } else if let Some(ref desk) = release.desktop_url {
        (desk.clone(), format!("baca-{}-macos.tar.gz", release.version))
    } else {
        (release.html_url.clone(), format!("baca-{}", release.tag_name))
    };

    let start_download = move |_| {
        if *is_downloading.read() { return; }
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
                class: "w-full max-w-lg rounded-t-3xl sm:rounded-3xl border shadow-2xl p-6 sm:p-7 flex flex-col gap-5 animate-spring-pop max-h-[90vh] overflow-y-auto",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()};",
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
                    button { class: "w-8 h-8 rounded-full flex items-center justify-center border apple-tap opacity-70", style: "border-color: {theme.border_color()}; color: {theme.text_secondary()};", onclick: move |_| on_close.call(()), "✕" }
                }

                // Version comparison pill
                div {
                    class: "flex items-center justify-between p-3.5 rounded-2xl border",
                    style: "background-color: {theme.bg_color()}; border-color: {theme.border_color()};",
                    div { class: "flex items-center gap-2",
                        span { class: "text-xs font-sans opacity-70", style: "color: {theme.text_secondary()};", "Versi Sekarang:" }
                        span { class: "text-xs font-mono font-medium px-2 py-0.5 rounded-full border", style: "border-color: {theme.border_color()}; color: {theme.text_primary()};", "v{current_ver}" }
                    }
                    span { class: "font-sans text-xs opacity-40", "➔" }
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
                            div { class: "p-3 rounded-2xl bg-emerald-500/10 border border-emerald-500/20 text-emerald-600 dark:text-emerald-400 text-xs font-medium text-center flex flex-col gap-1",
                                span { class: "font-semibold", "✓ Berkas berhasil diunduh!" }
                                if let Some(path) = saved_path.read().as_ref() {
                                    span { class: "text-[11px] opacity-75 font-mono truncate", "Tersimpan: {path}" }
                                }
                            }
                            a { class: "w-full py-3.5 rounded-2xl font-sans text-sm font-semibold flex items-center justify-center text-white shadow-md apple-tap", style: "background-color: {theme.accent_color()};", href: "{release.html_url}", target: "_blank", "Buka Laman Rilis & Pasang" }
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
                            class: "w-full py-3.5 rounded-2xl font-sans text-sm font-semibold flex items-center justify-center gap-2 text-white shadow-md apple-tap",
                            style: "background-color: {theme.accent_color()};",
                            onclick: start_download,
                            svg { class: "w-4 h-4", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                polyline { points: "7 10 12 15 17 10" }
                                line { x1: "12", y1: "15", x2: "12", y2: "3" }
                            }
                            "Unduh Pembaruan Sekarang"
                        }
                    }

                    a {
                        class: "w-full py-2.5 rounded-2xl font-sans text-xs font-medium text-center border apple-tap opacity-80 hover:opacity-100",
                        style: "border-color: {theme.border_color()}; color: {theme.text_secondary()};",
                        href: "{release.html_url}",
                        target: "_blank",
                        "Buka Laman Rilis di GitHub ↗"
                    }
                }
            }
        }
    }
}
