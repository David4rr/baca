/// Measures the padded viewport width for CSS multi-column pagination and
/// reports total page count back to Rust via `dioxus.send`.
pub const PROBE_SCRIPT: &str = r#"
(() => {
  const measure = () => {
    const el = document.getElementById('baca-page-content');
    const viewport = document.getElementById('baca-page-viewport');
    if (!el || !viewport) { dioxus.send({ pages: 1, width: 1.0 }); return; }
    const cs = window.getComputedStyle(viewport);
    const padL = parseFloat(cs.paddingLeft) || 0;
    const padR = parseFloat(cs.paddingRight) || 0;
    const w = Math.max(1, viewport.clientWidth - padL - padR);
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

/// Hardware-accelerated touch, pointer, tap-zone, wheel, and resize controller
/// for Paged reading mode.
pub const PAGED_GESTURE_SCRIPT: &str = r#"
(() => {
  const frame = document.getElementById('baca-page-frame');
  const viewport = document.getElementById('baca-page-viewport');
  const content = document.getElementById('baca-page-content');
  if (!frame || !viewport || !content) return;
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
      if (Math.abs(dx) > 10 && Math.abs(dx) > Math.abs(dy) * 1.15) {
        isSwiping = true;
      } else if (Math.abs(dy) > 10 && Math.abs(dy) > Math.abs(dx)) {
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

    const sel = window.getSelection && window.getSelection().toString();
    if (sel && sel.length > 0) return;

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
  frame.addEventListener('pointermove', handleMove);
  frame.addEventListener('pointerup', handleEnd);

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

  if (typeof ResizeObserver !== 'undefined') {
    let rafId = 0;
    const ro = new ResizeObserver(() => {
      cancelAnimationFrame(rafId);
      rafId = requestAnimationFrame(() => {
        const sync = document.getElementById('baca-resize-sync');
        if (sync) {
          sync.value = String(Date.now());
          sync.dispatchEvent(new Event('input', { bubbles: true }));
        }
      });
    });
    ro.observe(viewport);
  }
})();
"#;

/// Continuous scroll controller. Keeps `scroll` listener minimal (no DOM layout
/// queries or DB writes) and defers chapter detection & persistence to `scrollend`
/// (with a debounced fallback when `scrollend` is unsupported).
pub const SCROLL_ENGINE_SCRIPT: &str = r#"
(() => {
  const container = document.getElementById('baca-scroll-container');
  if (!container) return;

  const targetChapter = container.dataset.targetChapter;
  if (targetChapter !== undefined && !container.dataset.initialScrolled) {
    container.dataset.initialScrolled = '1';
    requestAnimationFrame(() => {
      const targetEl = document.getElementById('baca-chapter-' + targetChapter);
      if (targetEl) targetEl.scrollIntoView({ behavior: 'auto', block: 'start' });
    });
  }

  if (container.__bacaScrollBound) return;
  container.__bacaScrollBound = true;

  const uiSyncInput = document.getElementById('baca-scroll-sync');
  const commitInput = document.getElementById('baca-scroll-commit');
  const chSyncInput = document.getElementById('baca-chapter-sync');

  const computeFraction = () => {
    const maxScroll = Math.max(1, container.scrollHeight - container.clientHeight);
    return Math.min(1.0, Math.max(0.0, container.scrollTop / maxScroll));
  };

  let ticking = false;
  let lastUiUpdate = 0;
  container.addEventListener('scroll', () => {
    if (ticking) return;
    ticking = true;
    window.requestAnimationFrame(() => {
      ticking = false;
      const now = performance.now();
      if (now - lastUiUpdate < 120) return;
      lastUiUpdate = now;
      if (uiSyncInput) {
        uiSyncInput.value = computeFraction().toFixed(4);
        uiSyncInput.dispatchEvent(new Event('input', { bubbles: true }));
      }
    });
  }, { passive: true });

  const onScrollRest = () => {
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
    if (commitInput) {
      commitInput.value = computeFraction().toFixed(4);
      commitInput.dispatchEvent(new Event('input', { bubbles: true }));
    }
  };

  if ('onscrollend' in window) {
    container.addEventListener('scrollend', onScrollRest, { passive: true });
  } else {
    let fallbackTimer = null;
    container.addEventListener('scroll', () => {
      clearTimeout(fallbackTimer);
      fallbackTimer = setTimeout(onScrollRest, 120);
    }, { passive: true });
  }

  container.addEventListener('click', (e) => {
    if (e.target.closest('a') || e.target.closest('button')) return;
    const sel = window.getSelection && window.getSelection().toString();
    if (sel && sel.length > 0) return;
    const rect = container.getBoundingClientRect();
    const clickX = e.clientX - rect.left;
    const w = rect.width;
    if (clickX >= w * 0.25 && clickX <= w * 0.75) {
      const fsBtn = document.getElementById('baca-fullscreen-toggle-btn');
      if (fsBtn) fsBtn.click();
    }
  });
})();
"#;
