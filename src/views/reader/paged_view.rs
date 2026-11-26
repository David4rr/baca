use dioxus::prelude::*;

#[component]
pub fn PagedReaderContent(
    rendered_html: String,
    page_index: usize,
    page_count: usize,
    margin_horizontal: u32,
    is_fullscreen: bool,
) -> Element {
    let top_pad = if is_fullscreen {
        "max(calc(env(safe-area-inset-top, 0px) + 1.5rem), 2rem)"
    } else {
        "calc(env(safe-area-inset-top, 0px) + 4.75rem)"
    };
    let bottom_pad = if is_fullscreen {
        "max(calc(env(safe-area-inset-bottom, 0px) + 1.5rem), 2rem)"
    } else {
        "calc(env(safe-area-inset-bottom, 0px) + 10.5rem)"
    };

    let page_style = format!(
        "height: 100%; column-gap: 3rem; column-fill: auto; column-width: var(--baca-col-w, 100%); \
         transform: translateX(calc(-{} * (100% + 3rem))); \
         transition: transform 300ms cubic-bezier(0.16, 1, 0.3, 1);",
        page_index
    );

    rsx! {
        div {
            id: "baca-page-frame",
            "data-page": "{page_index}",
            "data-total-pages": "{page_count}",
            class: "relative w-full h-full overflow-hidden select-text",
            style: "touch-action: pan-y;",
            div {
                id: "baca-page-viewport",
                class: "absolute inset-x-0 max-w-2xl mx-auto overflow-hidden",
                style: "top: {top_pad}; bottom: {bottom_pad}; padding-left: {margin_horizontal}px; padding-right: {margin_horizontal}px; transition: top 300ms ease, bottom 300ms ease;",
                div {
                    id: "baca-page-content",
                    class: "baca-page-content chapter-content select-text",
                    style: "{page_style}",
                    dangerous_inner_html: "{rendered_html}",
                }
            }
        }
    }
}
