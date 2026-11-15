use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn AppleToast(
    message: Option<String>,
    theme: ThemeMode,
    on_dismiss: EventHandler<()>,
) -> Element {
    if let Some(msg) = message {
        rsx! {
            div {
                class: "fixed bottom-24 left-1/2 -translate-x-1/2 z-50 flex items-center gap-2.5 px-5 py-3 rounded-full shadow-xl border backdrop-blur-2xl animate-spring-pop select-none cursor-pointer",
                style: "background-color: {theme.surface_color()}F0; border-color: {theme.border_color()}; box-shadow: 0 12px 32px rgba(0,0,0,0.18);",
                onclick: move |_| on_dismiss.call(()),

                div {
                    class: "w-2 h-2 rounded-full",
                    style: "background-color: {theme.accent_color()};",
                }

                span {
                    class: "font-sans text-xs sm:text-sm font-medium tracking-tight",
                    style: "color: {theme.text_primary()};",
                    "{msg}"
                }
            }
        }
    } else {
        rsx! {}
    }
}
