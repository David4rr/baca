# Baca. 📖

[![GitHub release](https://img.shields.io/github/v/release/David4rr/baca?style=for-the-badge&color=orange&label=VERSION)](https://github.com/David4rr/baca/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.80+-DEA584?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Dioxus](https://img.shields.io/badge/Dioxus-0.6-000000?style=for-the-badge&logo=rust&logoColor=white)](https://dioxuslabs.com)

**Baca.** adalah aplikasi e-book reader lokal, ringan, dan modern yang ditulis ulang sepenuhnya menggunakan **Pure Rust** dan **Dioxus 0.6**. Mendukung multiplatform: **Android, macOS, Windows, dan Linux** dari satu basis kode terpadu.

Tanpa server, tanpa pelacakan (*zero telemetry*), tanpa ketergantungan webview lambat atau runtime JS besar. Koleksi buku, progres membaca, dan catatan Anda disimpan 100% lokal dengan database SQLite berkinerja tinggi.

---

## 🌟 Fitur Utama (v3.0 Pure Rust)

- **Pure Rust Engine & Dioxus UI**: Zero JavaScript overhead. Performa native, hemat memori, dan responsif instan.
- **Cross-Platform**:
  - 📱 **Android**: Native APK (`arm64-v8a`) dengan integrasi penyimpanan internal & folder Download.
  - 💻 **Desktop**: macOS (Apple Silicon / Intel), Windows (x86_64), dan Linux.
- **Streaming EPUB Parser**: Ekstraksi metadata OPF, cover art, dan dekompresi bab streaming cepat tanpa bloat memory.
- **SQLite Local Database**: Menyimpan metadata buku, progres baca per chapter, bookmark, dan preferensi pengguna dengan transaksi ACID.
- **Modern Editorial Aesthetic**:
  - Tema: **AMOLED Pitch Black**, **Warm Paper**, **Slate**, dan **Light Minimalist**.
  - Tipografi dinamis dengan kontrol ukuran font instan.
  - Table of Contents (TOC) drawer & navigasi bab mulus.
- **Security & Privacy First**:
  - Built-in HTML Sanitizer untuk membersihkan tag berbahaya (`<script>`, `<iframe>`, `javascript:` URI) dari berkas EPUB.
  - 100% offline tanpa koneksi cloud wajib.

---

## 📦 Unduh Aplikasi (Releases)

Unduh versi terbaru untuk perangkat Anda langsung dari [GitHub Releases](https://github.com/David4rr/baca/releases/latest):

| Platform | Format | Tautan |
|---|---|---|
| **Android** | `.apk` (ARM64) | [Unduh APK](https://github.com/David4rr/baca/releases/latest) |
| **macOS** | `.tar.gz` (Apple Silicon) | [Unduh macOS Binary](https://github.com/David4rr/baca/releases/latest) |
| **Windows** | `.zip` (x86_64) | [Unduh Windows Binary](https://github.com/David4rr/baca/releases/latest) |

---

## 🛠️ Pengembangan Lokal

### Prasyarat
- [Rust Toolchain](https://rustup.rs/) (1.80+)
- Dioxus CLI:
  ```bash
  cargo install dioxus-cli --version 0.6.3
  ```

### Menjalankan di Desktop
```bash
dx serve
# atau
cargo run --release
```

### Menjalankan Unit & Integration Tests
```bash
cargo test
```

### Build Android APK
Pastikan Android SDK & NDK telah terinstal:
```bash
dx build --platform android --release
```

---

## 📄 Lisensi
Didistribusikan di bawah lisensi [MIT](LICENSE).
