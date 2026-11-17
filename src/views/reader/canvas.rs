use dioxus::prelude::*;
use serde::Deserialize;

use crate::engine::Chapter;
use crate::theme::ThemeMode;

use super::highlight::highlight_html;
use super::top_bar::ReadingMode;

#[derive(Deserialize)]
struct PageProbe {
    pages: usize,
    #[serde(default)]
    _width: f64,
}

fn strip_page_images(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut cursor = 0;
    while cursor < html.len() {
        let rest = &html[cursor..];
        let next_img = rest.find("<img");
        let next_image = rest.find("<image");
        let next_svg = rest.find("<svg");

        let mut candidates = Vec::new();
        if let Some(i) = next_img { candidates.push((i, false)); }
        if let Some(i) = next_image { candidates.push((i, false)); }
        if let Some(i) = next_svg { candidates.push((i, true)); }
        candidates.sort_by_key(|c| c.0);

        if let Some((pos, is_svg)) = candidates.first() {
            out.push_str(&html[cursor..cursor + pos]);
            let start = cursor + pos;
            if *is_svg {
                if let Some(end) = html[start..].find("</svg>") {
                    cursor = start + end + 6;
                } else if let Some(end) = html[start..].find('>') {
                    cursor = start + end + 1;
                } else {
                    break;
                }
            } else if let Some(end) = html[start..].find('>') {
                cursor = start + end + 1;
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
    const el = document.getElementById('baca-page-content');
    const frame = document.getElementById('baca-page-frame');
    if (!el || !frame) { dioxus.send({ pages: 1, width: 1.0 }); return; }
    const w = frame.clientWidth || 1;
    document.documentElement.style.setProperty('--baca-col-w', w + 'px');
    const gap = 48;
    const pages = Math.max(1, Math.round((el.scrollWidth + gap) / (w + gap)));
    dioxus.send({ pages: pages, width: w });
  };
  const fonts = document.fonts && document.fonts.ready ? document.fonts.ready : Promise.resolve();
  const timeout = new Promise(resolve => setTimeout(resolve, 600));
  Promise.race([fonts, timeout])
    .then(() => requestAnimationFrame(() => requestAnimationFrame(measure)));
})();
"#;

const PAGED_GESTURE_SCRIPT: &str = r#"
(() => {
  const frame = document.getElementById('baca-page-frame');
  const content = document.getElementById('baca-page-content');
  if (!frame || !content) return;
  if (frame.__bacaSwipeBound) return;
  frame.__bacaSwipeBound = true;

  let startX = 0;
  let startY = 0;
  let currentDx = 0;
  let isSwiping = false;
  let isVertical = false;
  let pointerActive = false;

  const getPoint = (e) => {
    if (e.touches && e.touches.length > 0) return { x: e.touches[0].clientX, y: e.touches[0].clientY };
    return { x: e.clientX, y: e.clientY };
  };

  const handleStart = (e) => {
    const p = getPoint(e);
    startX = p.x;
    startY = p.y;
    currentDx = 0;
    isSwiping = false;
    isVertical = false;
    pointerActive = true;
  };

  const handleMove = (e) => {
    if (!pointerActive) return;
    const p = getPoint(e);
    const dx = p.x - startX;
    const dy = p.y - startY;

    if (!isSwiping && !isVertical) {
      if (Math.abs(dx) > 8 && Math.abs(dx) > Math.abs(dy)) {
        isSwiping = true;
      } else if (Math.abs(dy) > 8 && Math.abs(dy) > Math.abs(dx)) {
        isVertical = true;
      }
    }

    if (isSwiping) {
      if (e.cancelable) e.preventDefault();
      currentDx = dx;
      const page = parseInt(frame.dataset.page || '0');
      content.style.transition = 'none';
      content.style.transform = `translateX(calc(-${page} * (100% + 3rem) + ${dx}px))`;
    }
  };

  const handleEnd = (e) => {
    if (!pointerActive) return;
    pointerActive = false;

    if (isSwiping) {
      const page = parseInt(frame.dataset.page || '0');
      content.style.transition = 'transform 280ms cubic-bezier(0.16, 1, 0.3, 1)';
      if (currentDx < -45) {
        const btn = document.getElementById('baca-paged-next-btn');
        if (btn) btn.click();
      } else if (currentDx > 45) {
        const btn = document.getElementById('baca-paged-prev-btn');
        if (btn) btn.click();
      } else {
        content.style.transform = `translateX(calc(-${page} * (100% + 3rem)))`;
      }
      isSwiping = false;
      return;
    }

    const endPoint = (e.changedTouches && e.changedTouches.length > 0)
      ? { x: e.changedTouches[0].clientX, y: e.changedTouches[0].clientY }
      : getPoint(e);
    const dist = Math.hypot(endPoint.x - startX, endPoint.y - startY);
    if (dist < 12) {
      const rect = frame.getBoundingClientRect();
      const relX = endPoint.x - rect.left;
      const w = rect.width;
      if (relX > w * 0.72) {
        const btn = document.getElementById('baca-paged-next-btn');
        if (btn) btn.click();
      } else if (relX < w * 0.28) {
        const btn = document.getElementById('baca-paged-prev-btn');
        if (btn) btn.click();
      } else {
        const fsBtn = document.getElementById('baca-fullscreen-toggle-btn');
        if (fsBtn) fsBtn.click();
      }
    }
  };

  frame.addEventListener('touchstart', handleStart, { passive: true });
  frame.addEventListener('touchmove', handleMove, { passive: false });
  frame.addEventListener('touchend', handleEnd, { passive: true });
  frame.addEventListener('touchcancel', handleEnd, { passive: true });

  frame.addEventListener('pointerdown', handleStart);
  window.addEventListener('pointermove', handleMove);
  window.addEventListener('pointerup', handleEnd);

  let wheelCooldown = false;
  frame.addEventListener('wheel', (e) => {
    if (wheelCooldown || Math.abs(e.deltaY) < 8) return;
    wheelCooldown = true;
    setTimeout(() => { wheelCooldown = false; }, 300);
    if (e.deltaY > 0) {
      const btn = document.getElementById('baca-paged-next-btn');
      if (btn) btn.click();
    } else if (e.deltaY < 0) {
      const btn = document.getElementById('baca-paged-prev-btn');
      if (btn) btn.click();
    }
  }, { passive: true });
})();
"#;

const SCROLL_ENGINE_SCRIPT: &str = r#"
(() => {
  const container = document.getElementById('baca-scroll-container');
  if (!container) return;

  const targetChapter = container.dataset.targetChapter;
  if (targetChapter !== undefined && !container.dataset.initialScrolled) {
    container.dataset.initialScrolled = '1';
    requestAnimationFrame(() => {
      const targetEl = document.getElementById('baca-chapter-' + targetChapter);
      if (targetEl) {
        targetEl.scrollIntoView({ behavior: 'auto', block: 'start' });
      }
    });
  }

  if (!container.dataset.scrollBound) {
    container.dataset.scrollBound = '1';
    let ticking = false;

    const syncInput = document.getElementById('baca-scroll-sync');
    const chSyncInput = document.getElementById('baca-chapter-sync');

    container.addEventListener('scroll', () => {
      if (!ticking) {
        window.requestAnimationFrame(() => {
          const maxScroll = Math.max(1, container.scrollHeight - container.clientHeight);
          const fraction = Math.min(1.0, Math.max(0.0, container.scrollTop / maxScroll));
          if (syncInput) {
            syncInput.value = fraction.toFixed(4);
            syncInput.dispatchEvent(new Event('input', { bubbles: true }));
          }

          const sections = container.querySelectorAll('.baca-chapter-section');
          const containerTop = container.getBoundingClientRect().top;
          let activeIndex = -1;
          for (let i = 0; i < sections.length; i++) {
            const rect = sections[i].getBoundingClientRect();
            if (rect.bottom > containerTop + 80) {
              activeIndex = parseInt(sections[i].dataset.chapterIndex || '0');
              break;
            }
          }
          if (activeIndex >= 0 && chSyncInput && chSyncInput.value !== String(activeIndex)) {
            chSyncInput.value = String(activeIndex);
            chSyncInput.dispatchEvent(new Event('input', { bubbles: true }));
          }
          ticking = false;
        });
        ticking = true;
      }
    }, { passive: true });

    container.addEventListener('click', (e) => {
      if (e.target.closest('a') || e.target.closest('button')) return;
      const rect = container.getBoundingClientRect();
      const clickX = e.clientX - rect.left;
      const w = rect.width;
      if (clickX >= w * 0.25 && clickX <= w * 0.75) {
        const fsBtn = document.getElementById('baca-fullscreen-toggle-btn');
        if (fsBtn) fsBtn.click();
      }
    });
  }
})();
"#;

#[component]
pub fn ReaderCanvas(
    current_ch: Option<Chapter>,
    all_chapters: Vec<Chapter>,
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
    let mut landing_last = use_signal(|| false);
    let mut landing_for = use_signal(|| None::<usize>);
    let mut resize_epoch = use_signal(|| 0u32);
    let mut measured = use_signal(|| false);

    let is_paged = reading_mode == ReadingMode::Paged;
    let page_index = *page.read();
    let page_count = (*pages.read()).max(1);

    // Strip redundant page scan photos from rendered HTML
    let rendered_single = if is_paged {
        current_ch
            .as_ref()
            .map(|ch| {
                let stripped = strip_page_images(&ch.html_content);
                match &highlight_keyword {
                    Some(kw) if kw.trim().len() >= 2 => highlight_html(&stripped, kw.trim()),
                    _ => stripped,
                }
            })
            .unwrap_or_default()
    } else {
        String::new()
    };

    let rendered_all = if !is_paged {
        all_chapters
            .iter()
            .map(|ch| {
                let stripped = strip_page_images(&ch.html_content);
                match &highlight_keyword {
                    Some(kw) if kw.trim().len() >= 2 => highlight_html(&stripped, kw.trim()),
                    _ => stripped,
                }
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    // Reset pagination state when the chapter changes in Paged mode.
    use_effect(move || {
        let _ = current_chapter;
        if !is_paged {
            return;
        }
        page.set(0);
        pages.set(1);
        measured.set(false);
        let landing = *landing_for.peek() == Some(current_chapter);
        landing_last.set(landing);
        if landing {
            landing_for.set(None);
        }
    });

    // Measure page count in Paged mode
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
            }
        });
    });

    // Bind JavaScript gesture & scroll controllers
    use_effect(move || {
        let paged = is_paged;
        spawn(async move {
            if paged {
                let _ = document::eval(PAGED_GESTURE_SCRIPT);
            } else {
                let _ = document::eval(SCROLL_ENGINE_SCRIPT);
            }
        });
    });

    // Handle jumping to last page when navigating backwards in Paged mode
    use_effect(move || {
        if !is_paged || !*landing_last.peek() || *pages.read() <= 1 {
            return;
        }
        landing_last.set(false);
        let last = *pages.peek() - 1;
        page.set(last);
    });

    // Report progress for the scrubber in Paged mode
    use_effect(move || {
        if !is_paged || !*measured.read() {
            return;
        }
        let p = *page.read();
        let total = (*pages.read()).max(1);
        let fraction = ((p + 1) as f32 / total as f32).clamp(0.0, 1.0);
        on_metrics.call((current_chapter, fraction));
    });

    let mut go_next = move || {
        let p = *page.peek();
        let total = (*pages.peek()).max(1);
        if p + 1 < total {
            page.set(p + 1);
        } else if current_chapter + 1 < total_chapters {
            page.set(0);
            landing_last.set(false);
            on_change_chapter.call(current_chapter + 1);
        }
    };

    let mut go_prev = move || {
        let p = *page.peek();
        if p > 0 {
            page.set(p - 1);
        } else if current_chapter > 0 {
            landing_for.set(Some(current_chapter - 1));
            landing_last.set(true);
            on_change_chapter.call(current_chapter - 1);
        }
    };

    let page_style = format!(
        "height: 100%; column-gap: 3rem; column-fill: auto; column-width: var(--baca-col-w, 100%); \
         transform: translateX(calc(-{} * (100% + 3rem))); \
         transition: transform 300ms cubic-bezier(0.16, 1, 0.3, 1);",
        page_index
    );

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
                    "data-page": "{page_index}",
                    "data-total-pages": "{page_count}",
                    class: "relative w-full h-full overflow-hidden select-text",
                    style: "touch-action: pan-y;",
                    div {
                        class: "absolute inset-x-0 max-w-2xl mx-auto h-full overflow-hidden",
                        style: "top: {paged_top_padding}; bottom: {paged_bottom_padding}; padding-left: {margin_horizontal}px; padding-right: {margin_horizontal}px; transition: top 300ms ease, bottom 300ms ease;",
                        div {
                            id: "baca-page-content",
                            class: "baca-page-content chapter-content select-text",
                            style: "{page_style}",
                            dangerous_inner_html: "{rendered_single}",
                        }
                    }
                }
            } else {
                div {
                    id: "baca-scroll-container",
                    "data-target-chapter": "{current_chapter}",
                    class: "w-full h-full overflow-y-auto overscroll-y-contain select-text",
                    style: "touch-action: pan-y; -webkit-overflow-scrolling: touch;",
                    div {
                        class: "max-w-2xl mx-auto space-y-12 transition-all duration-300",
                        style: "padding-top: {scroll_top_padding}; padding-bottom: {scroll_bottom_padding}; padding-left: {margin_horizontal}px; padding-right: {margin_horizontal}px;",

                        for (idx, ch) in all_chapters.iter().enumerate() {
                            div {
                                key: "{ch.index}_{idx}",
                                id: "baca-chapter-{idx}",
                                class: "baca-chapter-section",
                                "data-chapter-index": "{idx}",

                                div {
                                    class: "text-center mb-8 pt-6 select-none",
                                    h1 {
                                        class: "font-serif text-[26px] sm:text-[32px] font-normal tracking-tight text-center mt-2 leading-[1.2]",
                                        style: "color: {theme.text_primary()};",
                                        "{ch.title}"
                                    }
                                }

                                div {
                                    class: "chapter-content select-text leading-relaxed text-justify",
                                    dangerous_inner_html: "{rendered_all.get(idx).unwrap_or(&ch.html_content)}",
                                }

                                if idx + 1 < total_chapters {
                                    div {
                                        class: "my-12 flex items-center justify-center gap-3 opacity-30 select-none",
                                        div { class: "h-px w-12 bg-current" }
                                        div { class: "w-1.5 h-1.5 rounded-full bg-current" }
                                        div { class: "h-px w-12 bg-current" }
                                    }
                                }
                            }
                        }

                        div {
                            class: "py-16 text-center text-xs font-sans opacity-40 tracking-wider uppercase select-none",
                            "Akhir dari Buku Ini"
                        }
                    }
                }
            }

            // Hidden action triggers
            button {
                id: "baca-paged-next-btn",
                class: "hidden",
                onclick: move |_| go_next(),
            }
            button {
                id: "baca-paged-prev-btn",
                class: "hidden",
                onclick: move |_| go_prev(),
            }
            button {
                id: "baca-fullscreen-toggle-btn",
                class: "hidden",
                onclick: move |_| on_toggle_fullscreen.call(()),
            }

            // Hidden synchronizers
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
                id: "baca-chapter-sync",
                r#type: "text",
                style: "display: none;",
                oninput: move |e| {
                    if !is_paged {
                        if let Ok(ch) = e.value().parse::<usize>() {
                            if ch != current_chapter && ch < total_chapters {
                                on_change_chapter.call(ch);
                            }
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
