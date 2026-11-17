use dioxus::prelude::*;
use serde::Deserialize;

use crate::engine::Chapter;
use crate::theme::ThemeMode;

use super::highlight::highlight_html;
use super::top_bar::ReadingMode;

#[derive(Deserialize)]
struct PageProbe {
    pages: usize,
}

fn strip_page_images(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut cursor = 0;
    while cursor < html.len() {
        if let Some(start) = html[cursor..].find("<img") {
            out.push_str(&html[cursor..cursor + start]);
            let rest = cursor + start;
            if let Some(end) = html[rest..].find('>') {
                cursor = rest + end + 1;
            } else {
                break;
            }
        } else {
            out.push_str(&html[cursor..]);
            break;
        }
    }
    out
}

const PROBE_SCRIPT: &str = r#"
(() => {
  const measure = () => {
    const el = document.querySelector('.baca-page-content');
    if (!el) { dioxus.send({ pages: 1 }); return; }
    const w = el.clientWidth || 1;
    document.documentElement.style.setProperty('--baca-col-w', w + 'px');
    const gap = parseFloat(getComputedStyle(el).columnGap) || 48;
    const pages = Math.max(1, Math.round((el.scrollWidth + gap) / (w + gap)));
    dioxus.send({ pages: pages });
  };
  const fonts = document.fonts && document.fonts.ready ? document.fonts.ready : Promise.resolve();
  const timeout = new Promise(resolve => setTimeout(resolve, 800));
  Promise.race([fonts, timeout])
    .then(() => requestAnimationFrame(() => requestAnimationFrame(measure)));
})();
"#;

const BIND_SCRIPT: &str = r#"
(() => {
  if (!window.__bacaResizeBound) {
    window.__bacaResizeBound = true;
    window.addEventListener('resize', () => {
      const input = document.getElementById('baca-resize-sync');
      if (input) {
        input.value = String(Date.now());
        input.dispatchEvent(new Event('input', { bubbles: true }));
      }
    });
  }
  const scrollEl = document.getElementById('baca-scroll-container');
  if (scrollEl && !scrollEl.dataset.scrollBound) {
    scrollEl.dataset.scrollBound = '1';
    let ticking = false;
    let autoAdvancing = false;
    let lastScrollTop = scrollEl.scrollTop;

    scrollEl.addEventListener('scroll', () => {
      if (!ticking) {
        window.requestAnimationFrame(() => {
          const total = Math.max(1, scrollEl.scrollHeight - scrollEl.clientHeight);
          const fraction = Math.min(1.0, Math.max(0.0, scrollEl.scrollTop / total));
          const input = document.getElementById('baca-scroll-sync');
          if (input) {
            input.value = fraction.toFixed(4);
            input.dispatchEvent(new Event('input', { bubbles: true }));
          }

          const currentTop = scrollEl.scrollTop;
          const isScrollingDown = currentTop > lastScrollTop;
          lastScrollTop = currentTop;

          // Seamless infinite scroll to next chapter at bottom
          if (!autoAdvancing && isScrollingDown && currentTop + scrollEl.clientHeight >= scrollEl.scrollHeight - 35) {
            autoAdvancing = true;
            const nextBtn = document.getElementById('baca-auto-next-btn');
            if (nextBtn) {
              nextBtn.click();
              setTimeout(() => { autoAdvancing = false; }, 900);
            } else {
              autoAdvancing = false;
            }
          }
          ticking = false;
        });
        ticking = true;
      }
    }, { passive: true });
  }
})();
"#;

#[component]
pub fn ReaderCanvas(
    current_ch: Option<Chapter>,
    current_chapter: usize,
    total_chapters: usize,
    font_family_css: String,
    font_size: u32,
    line_height: f32,
    margin_horizontal: u32,
    theme: ThemeMode,
    reading_mode: ReadingMode,
    is_fullscreen: bool,
    highlight_keyword: Option<String>,
    on_change_chapter: EventHandler<usize>,
    on_metrics: EventHandler<(usize, f32)>,
    on_clear_highlight: EventHandler<()>,
    on_toggle_fullscreen: EventHandler<()>,
) -> Element {
    let mut pages = use_signal(|| 1usize);
    let mut page = use_signal(|| 0usize);
    let mut frame_w = use_signal(|| 1.0f64);
    let mut landing_last = use_signal(|| false);
    let mut landing_for = use_signal(|| None::<usize>);
    let mut resize_epoch = use_signal(|| 0u32);
    let mut measured = use_signal(|| false);

    // Live swipe drag signals
    let mut drag_offset_x = use_signal(|| 0.0f64);
    let mut is_dragging = use_signal(|| false);
    let mut touch_origin = use_signal(|| (0.0f64, 0.0f64));
    let mut drag_axis = use_signal(|| 0i8); // 0 = uncommitted, 1 = horizontal, 2 = vertical

    let is_paged = reading_mode == ReadingMode::Paged;
    let page_index = *page.read();
    let page_count = (*pages.read()).max(1);

    // Strip redundant page scan photos from rendered HTML
    let rendered = current_ch
        .as_ref()
        .map(|ch| {
            let stripped = strip_page_images(&ch.html_content);
            match &highlight_keyword {
                Some(kw) if kw.trim().len() >= 2 => highlight_html(&stripped, kw.trim()),
                _ => stripped,
            }
        })
        .unwrap_or_default();

    // Reset pagination state when the chapter changes
    use_effect(move || {
        let _ = current_chapter;
        if !is_paged {
            return;
        }
        page.set(0);
        pages.set(1);
        measured.set(false);
        drag_offset_x.set(0.0);
        is_dragging.set(false);
        let landing = *landing_for.peek() == Some(current_chapter);
        landing_last.set(landing);
        if landing {
            landing_for.set(None);
        }
    });

    // In scroll mode, reset scroll position to top when advancing,
    // or jump to bottom when navigating backwards (landing_last).
    use_effect(move || {
        let _ = current_chapter;
        if is_paged {
            return;
        }
        let is_last = *landing_last.peek();
        if is_last {
            landing_last.set(false);
            on_metrics.call((current_chapter, 1.0));
        } else {
            on_metrics.call((current_chapter, 0.0));
        }
        spawn(async move {
            let script = if is_last {
                "(() => {
                    const el = document.getElementById('baca-scroll-container');
                    if (el) requestAnimationFrame(() => { el.scrollTop = el.scrollHeight - el.clientHeight; });
                })();"
            } else {
                "(() => {
                    const el = document.getElementById('baca-scroll-container');
                    if (el) el.scrollTop = 0;
                })();"
            };
            let _ = document::eval(script);
        });
    });

    // Measure how many viewport-height pages this chapter produces (Paged Mode only).
    use_effect(move || {
        if !is_paged {
            return;
        }
        let _ = current_chapter;
        let _ = font_size;
        let _ = line_height;
        let _ = font_family_css;
        let _ = margin_horizontal;
        let _ = highlight_keyword;
        let _ = *resize_epoch.read();
        spawn(async move {
            let probe = document::eval(PROBE_SCRIPT);
            if let Ok(p) = probe.join::<PageProbe>().await {
                let clamped = p.pages.max(1);
                if *pages.peek() != clamped {
                    pages.set(clamped);
                }
                measured.set(true);
                let w = document::eval(
                    "(() => { const el = document.getElementById('baca-page-frame'); dioxus.send(el ? el.clientWidth : 1); })();",
                );
                if let Ok(width) = w.join::<f64>().await {
                    frame_w.set(width.max(1.0));
                }
            }
        });
    });

    // Bind scroll listener for continuous scroll mode & resize listener for window.
    use_effect(move || {
        let _ = reading_mode;
        spawn(async move {
            let _ = document::eval(BIND_SCRIPT);
        });
    });

    // Jump to the previous chapter's last page when navigating backwards in Paged mode.
    use_effect(move || {
        if !is_paged || !*landing_last.peek() || *pages.read() <= 1 {
            return;
        }
        landing_last.set(false);
        let last = *pages.peek() - 1;
        page.set(last);
    });

    // Report progress for the scrubber in Paged mode.
    use_effect(move || {
        if !is_paged || !*measured.read() {
            return;
        }
        let p = *page.read();
        let total = (*pages.read()).max(1);
        let fraction = ((p + 1) as f32 / total as f32).clamp(0.0, 1.0);
        on_metrics.call((current_chapter, fraction));
    });

    let go_next = {
        let on_change = on_change_chapter;
        move || {
            let p = *page.peek();
            let total = (*pages.peek()).max(1);
            if p + 1 < total {
                page.set(p + 1);
            } else if current_chapter + 1 < total_chapters {
                page.set(0);
                landing_last.set(false);
                on_change.call(current_chapter + 1);
            }
        }
    };

    let go_prev = {
        let on_change = on_change_chapter;
        move || {
            let p = *page.peek();
            if p > 0 {
                page.set(p - 1);
            } else if current_chapter > 0 {
                landing_for.set(Some(current_chapter - 1));
                landing_last.set(true);
                on_change.call(current_chapter - 1);
            }
        }
    };

    // Live page translation style for Paged mode
    let current_drag = *drag_offset_x.read();
    let dragging = *is_dragging.read();
    let page_style = if dragging {
        format!(
            "height: 100%; column-gap: 3rem; column-fill: auto; \
             transform: translateX(calc(-{} * (100% + 3rem) + {}px)); transition: none;",
            page_index, current_drag
        )
    } else {
        format!(
            "height: 100%; column-gap: 3rem; column-fill: auto; \
             transform: translateX(calc(-{} * (100% + 3rem))); \
             transition: transform 320ms cubic-bezier(0.16, 1, 0.3, 1);",
            page_index
        )
    };

    let paged_top_padding = if is_fullscreen {
        "max(calc(env(safe-area-inset-top, 0px) + 1.5rem), 2rem)"
    } else {
        "calc(env(safe-area-inset-top, 0px) + 4.75rem)"
    };
    let paged_bottom_padding = if is_fullscreen {
        "max(calc(env(safe-area-inset-bottom, 0px) + 1.5rem), 2rem)"
    } else {
        "calc(env(safe-area-inset-bottom, 0px) + 10.5rem)"
    };

    let scroll_top_padding = if is_fullscreen {
        "max(calc(env(safe-area-inset-top, 0px) + 2rem), 2.5rem)"
    } else {
        "max(calc(env(safe-area-inset-top, 0px) + 5rem), 5.5rem)"
    };
    let scroll_bottom_padding = if is_fullscreen {
        "max(calc(env(safe-area-inset-bottom, 0px) + 2rem), 2.5rem)"
    } else {
        "max(calc(env(safe-area-inset-bottom, 0px) + 11rem), 12rem)"
    };

    rsx! {
        main {
            class: "reading-canvas relative w-full h-[100svh] flex flex-col overflow-hidden",
            style: "font-family: {font_family_css}; font-size: {font_size}px; line-height: {line_height};",

            if is_paged {
                div {
                    id: "baca-page-frame",
                    class: "relative w-full h-full overflow-hidden select-text",
                    style: "touch-action: pan-y;",
                    onwheel: {
                        let mut next = go_next;
                        let mut prev = go_prev;
                        move |e: WheelEvent| {
                            let dy = e.delta().strip_units().y;
                            if dy > 0.0 { next() } else if dy < 0.0 { prev() }
                        }
                    },
                    ontouchstart: move |e: TouchEvent| {
                        if let Some(t) = e.touches().first() {
                            let p = t.client_coordinates();
                            touch_origin.set((p.x, p.y));
                            drag_axis.set(0);
                            drag_offset_x.set(0.0);
                        }
                    },
                    ontouchmove: move |e: TouchEvent| {
                        if let Some(t) = e.touches().first() {
                            let p = t.client_coordinates();
                            let (sx, sy) = *touch_origin.peek();
                            let (dx, dy) = (p.x - sx, p.y - sy);
                            if *drag_axis.peek() == 0 {
                                if dx.abs() > dy.abs() && dx.abs() > 8.0 {
                                    drag_axis.set(1);
                                    is_dragging.set(true);
                                } else if dy.abs() > dx.abs() && dy.abs() > 8.0 {
                                    drag_axis.set(2);
                                }
                            }
                            if *drag_axis.peek() == 1 {
                                // Horizontal swipe drag with boundary rubber-band damping
                                let damp = if (page_index == 0 && dx > 0.0) || (page_index + 1 >= page_count && dx < 0.0) { 0.3 } else { 1.0 };
                                drag_offset_x.set(dx * damp);
                            }
                        }
                    },
                    ontouchend: {
                        let mut next = go_next;
                        let mut prev = go_prev;
                        move |_| {
                            if *drag_axis.peek() == 1 {
                                let offset = *drag_offset_x.peek();
                                if offset < -45.0 {
                                    next();
                                } else if offset > 45.0 {
                                    prev();
                                }
                            }
                            drag_offset_x.set(0.0);
                            is_dragging.set(false);
                            drag_axis.set(0);
                        }
                    },
                    onclick: {
                        let mut next = go_next;
                        let mut prev = go_prev;
                        move |e: MouseEvent| {
                            if drag_offset_x.peek().abs() > 10.0 { return; }
                            let x = e.element_coordinates().x;
                            let w = *frame_w.peek();
                            if x > w * 0.72 {
                                next();
                            } else if x < w * 0.28 {
                                prev();
                            } else {
                                // Center tap toggles distraction-free fullscreen mode
                                on_toggle_fullscreen.call(());
                            }
                        }
                    },
                    div {
                        class: "absolute inset-x-0 max-w-2xl mx-auto overflow-y-auto overscroll-y-contain",
                        style: "top: {paged_top_padding}; bottom: {paged_bottom_padding}; padding-left: {margin_horizontal}px; padding-right: {margin_horizontal}px; -webkit-overflow-scrolling: touch; transition: top 300ms ease, bottom 300ms ease;",
                        div {
                            class: "baca-page-content chapter-content select-text",
                            style: "{page_style}",
                            dangerous_inner_html: "{rendered}",
                        }
                    }
                }
            } else {
                div {
                    id: "baca-scroll-container",
                    class: "w-full h-full overflow-y-auto overscroll-y-contain select-text",
                    style: "touch-action: pan-y; -webkit-overflow-scrolling: touch;",
                    onclick: move |e: MouseEvent| {
                        let x = e.element_coordinates().x;
                        let w = e.client_coordinates().x;
                        let _ = (x, w);
                    },
                    div {
                        class: "max-w-2xl mx-auto space-y-6 transition-all duration-300",
                        style: "padding-top: {scroll_top_padding}; padding-bottom: {scroll_bottom_padding}; padding-left: {margin_horizontal}px; padding-right: {margin_horizontal}px;",

                        // Clean Page Title without "BAB" prefix
                        if let Some(ch) = &current_ch {
                            div {
                                class: "text-center mb-8 select-none",
                                h1 {
                                    class: "font-serif text-[26px] sm:text-[32px] font-normal tracking-tight text-center mt-2 leading-[1.2]",
                                    style: "color: {theme.text_primary()};",
                                    "{ch.title}"
                                }
                            }
                        }

                        div {
                            class: "chapter-content select-text leading-relaxed text-justify",
                            dangerous_inner_html: "{rendered}",
                        }

                        // Auto-advance sentinel at bottom (No manual button)
                        if current_chapter + 1 < total_chapters {
                            div {
                                id: "baca-scroll-sentinel",
                                class: "py-12 flex flex-col items-center justify-center opacity-40 select-none",
                                style: "color: {theme.text_secondary()};",
                                span { class: "font-sans text-xs tracking-wider", "Lanjut menggulir untuk bab berikutnya..." }
                            }
                        } else {
                            div {
                                class: "py-16 text-center text-xs font-sans opacity-40 tracking-wider uppercase",
                                "Akhir dari Buku Ini"
                            }
                        }
                    }
                }
            }

            // Hidden Triggers for Scroll Mode Auto-Navigation
            button {
                id: "baca-auto-next-btn",
                class: "hidden",
                onclick: {
                    let on_ch = on_change_chapter;
                    move |_| {
                        if current_chapter + 1 < total_chapters {
                            on_ch.call(current_chapter + 1);
                        }
                    }
                },
            }
            button {
                id: "baca-auto-prev-btn",
                class: "hidden",
                onclick: {
                    let on_ch = on_change_chapter;
                    move |_| {
                        if current_chapter > 0 {
                            landing_last.set(true);
                            on_ch.call(current_chapter - 1);
                        }
                    }
                },
            }

            input {
                id: "baca-scroll-sync",
                r#type: "text",
                style: "display: none;",
                oninput: move |e| {
                    if !is_paged {
                        if let Ok(f) = e.value().parse::<f32>() {
                            on_metrics.call((current_chapter, f));
                        }
                    }
                },
            }
            input {
                id: "baca-resize-sync",
                r#type: "text",
                style: "display: none;",
                oninput: move |_| {
                    let next = *resize_epoch.peek() + 1;
                    resize_epoch.set(next);
                },
            }

            if total_chapters == 0 {
                div { class: "p-12 text-center opacity-60 font-sans text-sm", "Bab buku tidak dapat dimuat." }
            }

            if let Some(kw) = &highlight_keyword {
                div { class: "fixed left-1/2 -translate-x-1/2 bottom-40 z-20 select-none",
                    button {
                        class: "rounded-full px-3 py-1.5 text-xs font-sans font-medium flex items-center gap-1.5 border apple-tap shadow-md",
                        style: "background-color: #fef3c7; border-color: #f59e0b; color: #78350f;",
                        onclick: move |_| on_clear_highlight.call(()),
                        span { "Menyorot: \"{kw}\"" }
                        svg { class: "w-3 h-3", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5",
                            line { x1: "18", y1: "6", x2: "6", y2: "18" }
                            line { x1: "6", y1: "6", x2: "18", y2: "18" }
                        }
                    }
                }
            }
        }
    }
}
