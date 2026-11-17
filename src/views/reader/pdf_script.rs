const PDF_LIB: &str = include_str!("../../../assets/vendor/pdf.min.js");
const PDF_WORKER: &str = include_str!("../../../assets/vendor/pdf.worker.min.js");

/// Boots the bundled PDF.js library and opens a local document. Runs once per
/// reader mount through `document::eval`; no network access is involved.
pub fn get_boot_script(pdf_base64: &str, initial_progress: f32) -> String {
    let lib = serde_json::to_string(PDF_LIB).unwrap_or_else(|_| "''".to_string());
    let worker = serde_json::to_string(PDF_WORKER).unwrap_or_else(|_| "''".to_string());
    let data = serde_json::to_string(pdf_base64).unwrap_or_else(|_| "''".to_string());
    let initial = initial_progress.clamp(0.0, 1.0);

    format!(
        r#"
(() => {{
  const b64 = {data};
  const fail = (msg) => {{
    const input = document.getElementById('baca-pdf-error');
    if (input) {{ input.value = msg; input.dispatchEvent(new Event('input', {{ bubbles: true }})); }}
  }};

  if (!window.pdfjsLib) {{
    const lib = document.createElement('script');
    lib.textContent = {lib};
    document.head.appendChild(lib);
  }}
  if (!window.pdfjsLib) {{ fail('Pustaka PDF tidak dapat dimuat'); return; }}

  if (!window.__bacaPdfWorker) {{
    const workerBlob = new Blob([{worker}], {{ type: 'application/javascript' }});
    window.__bacaPdfWorker = URL.createObjectURL(workerBlob);
  }}
  pdfjsLib.GlobalWorkerOptions.workerSrc = window.__bacaPdfWorker;

  const canvas = document.getElementById('baca-pdf-canvas');
  if (!canvas) {{ fail('Kanvas PDF tidak tersedia'); return; }}

  const totalInput = document.getElementById('baca-pdf-total');

  window.__baca_pdf_render = function(pageNum, scale) {{
    const pdf = window.__baca_pdf;
    if (!pdf) return;
    const page = Math.min(Math.max(1, pageNum), pdf.numPages);
    const cur = document.getElementById('baca-pdf-current');
    if (cur && cur.value !== String(page)) {{
      cur.value = String(page);
      cur.dispatchEvent(new Event('input', {{ bubbles: true }}));
    }}
    pdf.getPage(page).then(p => {{
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      const viewport = p.getViewport({{ scale: (scale || 1.0) * dpr }});
      const ctx = canvas.getContext('2d');
      canvas.width = viewport.width;
      canvas.height = viewport.height;
      canvas.style.width = '100%';
      canvas.style.height = 'auto';
      p.render({{ canvasContext: ctx, viewport: viewport }}).promise.catch(() => fail('Gagal menggambar halaman'));
    }}).catch(() => fail('Halaman PDF tidak dapat dibuka'));
  }};

  const raw = atob(b64);
  const bytes = new Uint8Array(raw.length);
  for (let i = 0; i < raw.length; i++) bytes[i] = raw.charCodeAt(i);
  window.__bacaPdfData = bytes;

  const task = pdfjsLib.getDocument({{ data: bytes, isEvalSupported: false }});
  task.promise.then(pdf => {{
    window.__baca_pdf = pdf;
    if (totalInput) {{
      totalInput.value = String(pdf.numPages);
      totalInput.dispatchEvent(new Event('input', {{ bubbles: true }}));
    }}
    const start = Math.max(1, Math.round({initial} * pdf.numPages));
    window.__baca_pdf_render(start, 1.0);
    window.__baca_pdf_start_page = start;
  }}).catch(err => fail('Berkas PDF tidak dapat dibuka: ' + (err && err.message ? err.message : 'unknown')));
}})();
"#
    )
}
