use crate::state::{AppState, View};
use dioxus::prelude::*;

pub const BACK_SCRIPT: &str = r#"
    (function() {
        try {
            var meta = document.querySelector('meta[name="viewport"]');
            if (!meta) {
                meta = document.createElement('meta');
                meta.name = 'viewport';
                document.head.appendChild(meta);
            }
            meta.content = 'width=device-width, initial-scale=1.0, maximum-scale=1.0, viewport-fit=cover, user-scalable=no';
        } catch(e) {}

        if (window.__baca_back_inited) return;
        window.__baca_back_inited = true;
        try {
            window.history.pushState({ page: 1 }, "");
            window.addEventListener("popstate", function(e) {
                window.history.pushState({ page: 1 }, "");
                var btn = document.getElementById("baca-back-trigger");
                if (btn) btn.click();
            });
        } catch(err) {}
    })();
"#;

pub fn handle_device_back(
    mut state: AppState,
    mut show_update_modal: Signal<bool>,
) {
    if *show_update_modal.read() {
        show_update_modal.set(false);
        return;
    }
    if state.format_modal_book.read().is_some() {
        *state.format_modal_book.write() = None;
        return;
    }
    if *state.is_typography_open.read() {
        *state.is_typography_open.write() = false;
        return;
    }
    if *state.is_toc_open.read() {
        *state.is_toc_open.write() = false;
        return;
    }
    if *state.is_inbook_search_open.read() {
        *state.is_inbook_search_open.write() = false;
        return;
    }
    if *state.is_ai_open.read() {
        *state.is_ai_open.write() = false;
        return;
    }

    let view = state.current_view.read().clone();
    match view {
        View::BookDetail { .. } => {
            *state.current_view.write() = View::Explore;
            return;
        }
        v if v != View::Shelf => {
            *state.current_view.write() = View::Shelf;
            return;
        }
        _ => {}
    }

    let now = chrono::Utc::now().timestamp_millis();
    let last = *state.last_back_press.read();
    if now - last < 2000 {
        std::process::exit(0);
    } else {
        *state.last_back_press.write() = now;
        *state.toast_message.write() = Some("Tekan sekali lagi untuk keluar".to_string());
    }
}
