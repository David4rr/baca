use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn AppleToast(
    message: Option<String>,
    theme: ThemeMode,
    on_dismiss: EventHandler<()>,
) -> Element {
    let msg_clone = message.clone();
    use_effect(use_reactive(&msg_clone, move |msg| {
        if msg.is_some() {
            let on_d = on_dismiss.clone();
            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(2600)).await;
                on_d.call(());
            });
        }
    }));

    if let Some(msg) = message {
        rsx! {
            div {
                class: "fixed left-1/2 -translate-x-1/2 z-50 flex items-center gap-2.5 px-5 py-2.5 rounded-full shadow-2xl border backdrop-blur-2xl animate-spring-pop select-none cursor-pointer max-w-[90vw]",
                style: "top: max(calc(env(safe-area-inset-top, 0px) + 0.85rem), 1.25rem); background-color: {theme.surface_color()}F5; border-color: {theme.border_color()}; box-shadow: 0 16px 36px rgba(0,0,0,0.18);",
                onclick: move |_| on_dismiss.call(()),

                div {
                    class: "w-2 h-2 rounded-full flex-shrink-0 animate-pulse",
                    style: "background-color: {theme.accent_color()};",
                }

                span {
                    class: "font-sans text-xs sm:text-[13px] font-medium tracking-tight truncate",
                    style: "color: {theme.text_primary()};",
                    "{msg}"
                }
            }
        }
    } else {
        rsx! {}
    }
}
