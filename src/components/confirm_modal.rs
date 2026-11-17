use dioxus::prelude::*;
use crate::theme::ThemeMode;

#[component]
pub fn ConfirmDeleteModal(
    title: String,
    message: String,
    theme: ThemeMode,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "fixed inset-0 z-50 bg-black/40 backdrop-blur-sm animate-fade-in flex items-center justify-center p-4",
            onclick: move |_| on_cancel.call(()),
            div {
                class: "w-full max-w-sm rounded-[24px] border p-6 shadow-lg flex flex-col gap-4 animate-slide-up select-none",
                style: "background-color: {theme.surface_color()}; border-color: {theme.border_color()}; color: {theme.text_primary()};",
                onclick: |e| e.stop_propagation(),

                div { class: "flex flex-col gap-1 text-center",
                    h3 { class: "font-serif text-lg font-semibold", "{title}" }
                    p { class: "font-sans text-xs opacity-65 leading-relaxed", "{message}" }
                }

                div { class: "flex gap-3 pt-2",
                    button {
                        class: "flex-1 py-2.5 rounded-xl border text-xs font-sans font-medium apple-tap transition-all",
                        style: "border-color: {theme.border_color()}; background-color: {theme.surface_subtle()};",
                        onclick: move |_| on_cancel.call(()),
                        "Batal"
                    }
                    button {
                        class: "flex-1 py-2.5 rounded-xl text-xs font-sans font-semibold bg-rose-500 text-white shadow-sm apple-tap transition-all hover:bg-rose-600",
                        onclick: move |_| on_confirm.call(()),
                        "Hapus"
                    }
                }
            }
        }
    }
}
