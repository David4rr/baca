# DESIGN SYSTEM SPECIFICATION — BACA.
## Apple-Standard Editorial Minimalism & Haptic Reading Experience

> **Source of Truth:** Based on the master design reference (`Book reader.jpg`).
> **Philosophy:** High-end editorial typography, physical paper warmth, Apple-tier spring dynamics, zero generic AI cliches.

---

## 1. Visual Atmosphere & Aesthetic Stance

- **Aesthetic Persona:** *Editorial Luxury & Apple Human Interface* (Physical paper warmth meets Cupertino precision).
- **Density Dial:** `3 / 10` (Art Gallery Airy & Breathing). Generous macro-whitespace (`py-12` to `py-24`), zero cramped borders.
- **Variance Dial:** `6 / 10` (Asymmetrical Bento & Hero Hierarchy). Hero "Continue Reading" card anchors the library, followed by horizontal recents and clean editorial grid.
- **Motion Dial:** `8 / 10` (Apple Spring Physics & Micro-Haptics). Every tap responds with tactile spring compression (`active:scale-[0.97]`), floating frosted islands (`backdrop-blur-2xl`), and staggered sheet reveals.

---

## 2. Color Calibration & Master Palettes

No pure blacks, no harsh neon gradients, no generic blue/purple AI glows. Every palette is calibrated for long-form reading comfort and luxury tactile feel.

### 2.1 The Master Palettes

| Token | Warm Editorial (Default) | Pure Light | AMOLED Night | Midnight Slate |
|---|---|---|---|---|
| **Canvas Background** | `#F9F6F0` (Warm Cream Linen) | `#F8FAFC` (Pure Pearl) | `#0A0A0A` (Deepest Charcoal) | `#0F172A` (Navy Slate) |
| **Surface (Card/Bento)** | `#FFFFFF` (Pure Crisp Paper) | `#FFFFFF` (Elevated White) | `#161616` (Smoky Glass) | `#1E293B` (Soft Slate) |
| **Surface Subtle** | `#F3EFE6` (Warm Sand) | `#F1F5F9` (Ice Muted) | `#222222` (Subtle Rim) | `#334155` (Slate Rim) |
| **Primary Text** | `#1C1917` (Deep Espresso Black) | `#0F172A` (Rich Obsidian) | `#F4F4F5` (Bone White) | `#F8FAFC` (Clean White) |
| **Secondary Text** | `#78716C` (Warm Stone Gray) | `#64748B` (Cool Slate) | `#A1A1AA` (Muted Zinc) | `#94A3B8` (Soft Gray) |
| **Muted Text / Hints** | `#A8A29E` (Subtle Warm) | `#94A3B8` (Subtle Cool) | `#71717A` (Deep Muted) | `#64748B` (Muted Navy) |
| **Primary CTA Button** | `#1C1917` (Pitch Espresso) | `#0F172A` (Obsidian) | `#F4F4F5` (Bone White) | `#38BDF8` (Sky Accent) |
| **CTA Text** | `#FFFFFF` (Pure White) | `#FFFFFF` (Pure White) | `#0A0A0A` (Deep Black) | `#0F172A` (Deep Navy) |
| **Highlight Accent** | `#FEF08A` (Warm Amber Glow) | `#FEF08A` (Warm Amber) | `#FDE047` (Gold Accent) | `#38BDF8` (Electric Sky) |
| **Border / Hairline** | `rgba(28, 25, 23, 0.06)` | `rgba(15, 23, 42, 0.08)` | `rgba(255, 255, 255, 0.08)` | `rgba(255, 255, 255, 0.1)` |
| **Card Shadow** | `0 10px 30px -4px rgba(28, 25, 23, 0.04), 0 4px 12px -2px rgba(28, 25, 23, 0.02)` | `0 10px 30px rgba(0,0,0,0.03)` | `0 10px 30px rgba(0,0,0,0.5)` | `0 10px 30px rgba(0,0,0,0.3)` |

---

## 3. Typographic Architecture

Typography is the heart of the editorial experience. We pair a refined editorial serif for titles and body prose with an Apple-style human sans-serif for UI chrome and metadata.

### 3.1 Font Pairing
- **Editorial Serif (Headings & Reading Body):** `Newsreader`, `New York`, `Playfair Display`, `EB Garamond`.
  - Characteristics: Elegant optical sizing, high contrast, warm literary curves.
  - Usage: App view title ("My Library"), book titles, chapter eyebrow titles, in-reader body text.
- **UI Sans-Serif (Interface Chrome & Controls):** `Plus Jakarta Sans`, `Geist`, `-apple-system`, `SF Pro Display`.
  - Characteristics: Precision geometric curves, high x-height, crystal-clear readability at micro-sizes.
  - Usage: Navigation buttons, metadata, progress labels, time estimates, modal controls.

### 3.2 Scale Hierarchy
- **Library Title:** `font-serif font-medium text-[32px] tracking-tight leading-none text-[#1C1917]`
- **Library Subtitle:** `font-sans text-[13px] font-normal tracking-normal text-[#78716C]`
- **Chapter Eyebrow:** `font-sans text-[11px] uppercase tracking-[0.25em] font-semibold text-[#A8A29E]`
- **Chapter Headline:** `font-serif font-normal text-[32px] md:text-[38px] tracking-tight leading-[1.15] text-[#1C1917]`
- **Reading Body:** `font-serif text-[18px] leading-[1.8] text-[#1C1917] selection:bg-[#FEF08A]`
- **Meta / Labels:** `font-sans text-[12px] font-medium text-[#78716C]`
- **Micro Progress:** `font-sans text-[11px] font-semibold text-[#1C1917]`

---

## 4. Component Mastery (Direct Reference Breakdown)

### 4.1 "Continue Reading" Hero Bento Card (Shelf)
- **Nested Architecture (Double-Bezel):**
  - Outer Container: `rounded-[28px] bg-white p-5 shadow-[0_12px_36px_rgba(28,25,23,0.04)] border border-black/[0.03]`
  - Layout: Two-row asymmetric composition.
  - **Top Row (Book Details):**
    - Left: Book Cover thumbnail with 2:3 aspect ratio, `rounded-xl`, subtle 3D spine shadow (`shadow-[0_4px_16px_rgba(0,0,0,0.12)]`), width ~84px.
    - Right: Information stack:
      - Header line: Context menu button (`...`) in top right circle (`w-8 h-8 rounded-full bg-stone-100/70 hover:bg-stone-200 flex items-center justify-center`).
      - Book Title: `font-serif font-semibold text-[17px] text-stone-900 leading-snug line-clamp-2`.
      - Author: `font-sans text-[13px] text-stone-500 mt-1`.
      - Progress Section:
        - Percentage badge: `65%` floated on the right.
        - Progress track: `h-1 w-full bg-stone-100 rounded-full overflow-hidden mt-2`.
        - Progress bar: `h-full bg-stone-900 rounded-full transition-all duration-500`.
        - Time left estimate: `14 min left in chapter` (`text-[11px] text-stone-400 mt-1.5`).
  - **Bottom Row (Action Island):**
    - Full-width CTA pill: `Continue Reading` with nested trailing chevron circle `>`.
    - Button styling: `bg-[#1C1917] text-white rounded-2xl h-12 px-5 flex items-center justify-between font-sans text-sm font-medium hover:bg-stone-800 active:scale-[0.98] transition-all`.
    - Trailing Icon: `w-6 h-6 rounded-full bg-white/10 flex items-center justify-center text-xs`.

### 4.2 "Recent" Book Carousel & Cards
- **Section Header:** `Recent` (serif font-semibold text-[18px]) on left + `View all` (sans-serif text-[12px] text-stone-500 hover:text-stone-900) on right.
- **Card Design:**
  - Realistic Cover Art: `rounded-xl shadow-[0_6px_20px_rgba(0,0,0,0.08)] aspect-[2/3] object-cover`.
  - Cover Hover Physics: `hover:-translate-y-1 hover:shadow-[0_12px_28px_rgba(0,0,0,0.14)] transition-all duration-300 ease-out`.
  - Metadata block below cover:
    - Title: `font-serif text-[14px] font-medium text-stone-900 truncate mt-2.5`.
    - Author: `font-sans text-[12px] text-stone-500 truncate mt-0.5`.
    - Progress Tag: `font-sans text-[11px] text-stone-400 font-medium mt-1` (e.g. `38%`, `Finished`, `27%`).

### 4.3 Floating Bottom Navigation Dock (Apple Island)
- **Structure:** Floating detached frosted glass island at bottom center: `fixed bottom-6 left-1/2 -translate-x-1/2 rounded-full bg-white/85 dark:bg-stone-900/85 backdrop-blur-2xl px-6 py-3 shadow-[0_16px_40px_rgba(0,0,0,0.08)] border border-black/[0.05] flex items-center gap-8 z-40`.
- **Items:**
  - `Home / Library` (active state: solid filled black `#1C1917` with subtle scale-105).
  - `Explore / Archive.org` (clean stroke SVG).
  - `Statistics` (clean bar chart SVG).
  - `Settings / Backup` (clean slider gear SVG).
- **Haptic Press:** `active:scale-90 transition-transform duration-150`.

### 4.4 Reader Navigation & Controls (Right Screen Reference)
- **Top Bar (Minimal & Floating):**
  - Left: Circular back button `<` (`w-10 h-10 rounded-full bg-white/80 dark:bg-stone-800/80 backdrop-blur shadow-sm border border-black/[0.04] flex items-center justify-center text-stone-800 hover:bg-white active:scale-95`).
  - Right: Cluster of clean icons:
    - `Aa` (Font size, typeface, theme control modal sheet).
    - `≡` (Table of Contents slide-over drawer).
    - `🔖` (Bookmark toggle).
    - `🔍` (In-book fulltext search).
    - `✨` (Gemini AI assistant).

### 4.5 Reader Body & In-Text Highlight
- **Center Canvas:** Contained reading column (`max-w-2xl mx-auto px-6 py-8`).
- **Chapter Eyebrow:** `CHAPTER 5` (uppercase tracking-[0.25em] text-[11px] text-stone-400 font-semibold text-center mb-2`).
- **Chapter Headline:** `The Wealth Equation` (large editorial serif text-[32px] text-stone-900 text-center mb-8 leading-tight`).
- **Highlighted Passage:** Soft warm ochre/gold highlight (`bg-[#FEF08A]/80 dark:bg-amber-900/40 px-1 py-0.5 rounded text-stone-900 dark:text-amber-100`).
- **Action Pill:** Floating pill `✨ Highlights` or `✨ Tanya AI` positioned near selections.

### 4.6 Reader Scrub Slider & Segmented Theme Dock
- **Scrubber Bar:**
  - Ultra-fine track (`h-1 bg-stone-200/80 rounded-full`).
  - Interactive thumb (`w-3 h-3 rounded-full bg-stone-900 shadow-sm`).
  - Status row below track:
    - Left: `65%` (`text-stone-400 text-[11px] font-medium`).
    - Center: `Chapter 5 of 12` (`text-stone-500 text-[12px] font-medium`).
    - Right: `14 min left` (`text-stone-400 text-[11px] font-medium`).
- **Floating Apple Segmented Theme Switcher:**
  - Centered floating pill dock at bottom: `rounded-full bg-stone-200/60 dark:bg-stone-800/60 backdrop-blur-xl p-1 flex items-center gap-1 shadow-sm`.
  - Segment options:
    - ☀️ **Light / Paper** (Sun SVG)
    - 📖 **Warm Editorial** (Book / Equalizer SVG)
    - 🌙 **OLED Dark** (Moon SVG)
  - Active segment: Elevated white pill (`bg-white dark:bg-stone-700 rounded-full shadow-sm px-3 py-1.5 transition-all duration-300`).

---

## 5. Motion Choreography & Apple Spring Physics

All transitions must simulate real mass, momentum, and spring physics. Never use generic linear or standard ease.

### 5.1 Physics Constants
```css
/* Apple Human Interface Spring Curves */
--ease-spring: cubic-bezier(0.16, 1, 0.3, 1);
--ease-spring-snappy: cubic-bezier(0.32, 0.72, 0, 1);
--ease-spring-bouncy: cubic-bezier(0.34, 1.56, 0.64, 1);
```

### 5.2 Micro-Interactions
- **Buttons & Cards Tap:** `active:scale-[0.97]` over `120ms` with `--ease-spring`.
- **Modal & Sheet Presentation:** Smooth spring slide-up from bottom (`translate-y-8 opacity-0 -> translate-y-0 opacity-100`) over `350ms` with backdrop blur fade.
- **TOC Drawer:** Smooth horizontal spring slide from left (`-translate-x-full -> translate-x-0`) over `400ms`.
- **Book Cover Hover:** Subtle `translate-y-[-4px]` with shadow expansion.

---

## 6. Strict Anti-Patterns (Banned AI Clichés)

1. **NO Generic Emojis in UI Chrome:** Never use emojis (like 📚, 📊, ⚙️, 🌐, 🔍) as button icons or headers. Use bespoke, ultra-fine vector SVG paths (Apple SF Symbols style).
2. **NO Harsh Borders or Heavy Shadows:** Never use `border-gray-300`, `border-2`, or `shadow-2xl` black boxes. Use `border-black/[0.04]` and multi-layered ambient diffused shadows.
3. **NO Neon Blue/Purple AI Gradients:** Banned. Only warm physical paper tones, crisp white surfaces, and deep obsidian/espresso accents.
4. **NO Cluttered Layouts:** Double macro-whitespace. The canvas must feel like a luxury art book or high-end Apple editorial publication.
5. **NO Abrupt Cuts:** Every state change, drawer open, search toggle, and theme switch must interpolate smoothly with spring physics.
