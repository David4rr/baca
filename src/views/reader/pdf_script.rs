/// PDF.js initialization and rendering script for HTML5 Canvas.
pub fn get_pdf_script(file_url: &str) -> String {
    format!(r#"
        (function() {{
            const url = "{file_url}";
            const canvas = document.getElementById('baca-pdf-canvas');
            if (!canvas) return;

            function initPdfJs() {{
                if (typeof pdfjsLib === 'undefined') {{
                    const s = document.createElement('script');
                    s.src = 'https://cdnjs.cloudflare.com/ajax/libs/pdf.js/3.11.174/pdf.min.js';
                    s.onload = () => {{
                        pdfjsLib.GlobalWorkerOptions.workerSrc = 'https://cdnjs.cloudflare.com/ajax/libs/pdf.js/3.11.174/pdf.worker.min.js';
                        loadPdf(url);
                    }};
                    s.onerror = () => {{
                        const fallback = document.getElementById('baca-pdf-fallback');
                        if (fallback) fallback.style.display = 'flex';
                    }};
                    document.head.appendChild(s);
                }} else {{
                    loadPdf(url);
                }}
            }}

            function loadPdf(pdfUrl) {{
                pdfjsLib.getDocument(pdfUrl).promise.then(pdf => {{
                    window.__baca_pdf = pdf;
                    window.__baca_total_pages = pdf.numPages;
                    const elTotal = document.getElementById('baca-pdf-total-pages');
                    if (elTotal) elTotal.innerText = pdf.numPages;
                    window.__baca_render_pdf_page(1, 1.0);
                }}).catch(() => {{
                    const fallback = document.getElementById('baca-pdf-fallback');
                    if (fallback) fallback.style.display = 'flex';
                }});
            }}

            window.__baca_render_pdf_page = function(pageNum, scale) {{
                if (!window.__baca_pdf) return;
                window.__baca_pdf.getPage(pageNum).then(page => {{
                    const viewport = page.getViewport({{ scale: scale || 1.2 }});
                    const ctx = canvas.getContext('2d');
                    canvas.height = viewport.height;
                    canvas.width = viewport.width;
                    page.render({{ canvasContext: ctx, viewport: viewport }});
                }});
            }};

            initPdfJs();
        }})();
    "#)
}
