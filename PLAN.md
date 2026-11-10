# 🔎 QORI: Personal Search Engine (Local-First Desktop App)

> Search engine pribadi yang berjalan **100% lokal** sebagai aplikasi desktop (Windows / macOS / Linux).
> Tanpa akun, tanpa langganan, tanpa tracking. Data tinggal di mesinmu.

---

## 🌟 Fitur Terbaru: Webpage Sitelinks Branching & Left-Aligned Layout

Aplikasi telah mengadopsi struktur layout pencarian sisi kiri (*left-aligned column layout*) dan hierarki percabangan sub-halaman (*sitelinks branching*) dengan tetap mempertahankan estetika **Neo-Brutalist & Dark Editorial**:

1. **Tata Letak Sisi Kiri (*Left-Aligned Layout*)**:
   - Kolom hasil pencarian berada di sisi kiri layar (`mr-auto max-w-3xl px-4 sm:px-6 md:pl-10 lg:pl-16`) layaknya Google, memberikan ruang visual bersih di sisi kanan.
2. **Kategori Bar di Bawah Search Bar**:
   - Tab navigasi kategori: `Semua`, `Gambar`, `Video`, dan `Berita` dengan ikon dan indikator aktif.
3. **Hero Card dengan Cabang Sub-Halaman (*Sitelinks Branches*)**:
   - Untuk hasil pencarian relevan dengan domain utama (seperti `youtube music`, `golang`, dll.), kartu hasil diprioritaskan di posisi teratas (#1 Hero Card).
   - Menampilkan **minimal 4 dan maksimal 6 cabang sub-halaman** dengan judul, cuplikan ringkasan, dan indikator navigasi `>`.
   - Mengklik cabang manapun langsung membuka sub-halaman tersebut di dalam tab Qori (`WebTabScreen.svelte`).
   - Tautan eksplorasi di bagian bawah: *"Jelajahi halaman lainnya dari {domain} »"*.

---

README ini berisi:

1. Gambaran proyek
2. Rekomendasi arsitektur & tech stack (backend, data, search)
3. Spesifikasi desain UI/UX (referensi: ArtCraft)
4. **Master Prompt** siap tempel ke AI coding agent (Claude Code, dll.)
5. Struktur folder, skema data, roadmap, dan acceptance criteria

---

## 1. Gambaran Proyek

Aplikasi desktop yang berfungsi sebagai mesin pencari pribadi dengan tiga sumber hasil:

| Sumber                         | Fungsi                                                                                  |
| ------------------------------ | --------------------------------------------------------------------------------------- |
| **Local Index**                | Halaman web yang kamu crawl sendiri, bookmark, riwayat browsing, dan file/dokumen lokal |
| **Metasearch (opsional)**      | Meneruskan query ke SearXNG lokal untuk hasil web umum, tanpa tracking                  |
| **Semantic Search (opsional)** | Pencarian berdasarkan makna memakai embedding lokal (Ollama)                            |

Prinsip: **local-first, privacy-first, open-source, tidak butuh internet untuk mencari data milikmu sendiri.**

---

## 2. Rekomendasi Arsitektur

Karena kamu belum menentukan backend, ini stack yang aku rekomendasikan. Alasannya: ringan, cepat, satu bahasa untuk inti, dan selaras dengan ArtCraft yang dibangun dengan Rust.

### 2.1 Ringkasan Stack

| Layer                 | Pilihan                                            | Alasan                                            |
| --------------------- | -------------------------------------------------- | ------------------------------------------------- |
| App shell             | **Tauri 2** (Rust)                                 | Installer kecil (~10 MB), native, aman, lintas OS |
| Frontend              | **SvelteKit** atau **React + Vite** + TypeScript   | Animasi mulus, ekosistem besar                    |
| Styling               | **Tailwind CSS** + CSS variables                   | Cepat membangun design system                     |
| Animasi               | **GSAP** + ScrollTrigger / **Motion**              | Untuk motion eksperimental ala ArtCraft           |
| Search engine inti    | **Tantivy** (Rust, mirip Lucene)                   | Full-text search super cepat, embedded            |
| Database metadata     | **SQLite** (via `rusqlite` / `sqlx`)               | Satu file, tanpa server                           |
| Crawler               | **reqwest** + **scraper** + **readability** (Rust) | Patuh robots.txt, ekstraksi konten bersih         |
| Semantic (opsional)   | **Ollama** (`nomic-embed-text`) + **sqlite-vec**   | Embedding lokal, tanpa cloud                      |
| Metasearch (opsional) | **SearXNG** (Docker, localhost)                    | Agregator hasil web yang privat                   |
| Parser file lokal     | `pdf-extract`, `docx-rs`, `pulldown-cmark`         | Index PDF, DOCX, Markdown, TXT                    |
| Scheduler             | `tokio` + `tokio-cron-scheduler`                   | Re-crawl berkala di background                    |

### 2.2 Diagram Alur

```
┌─────────────────────── Tauri Desktop App ───────────────────────┐
│                                                                 │
│  Frontend (Svelte/React)  ◄── IPC (invoke/events) ──►  Rust Core│
│  Dark Editorial UI                                              │
│                                          ┌───────────────────┐  │
│                                          │ Query Orchestrator│  │
│                                          └───┬───────┬───────┘  │
│                                              │       │          │
│                          ┌───────────────────┘       └────────┐ │
│                          ▼                                    ▼ │
│                 ┌─────────────────┐                ┌──────────────┐
│                 │ Tantivy Index   │                │ SearXNG      │
│                 │ (full-text)     │                │ (optional)   │
│                 └────────┬────────┘                └──────────────┘
│                          │                                        │
│     ┌────────────────────┼─────────────────────┐                  │
│     ▼                    ▼                     ▼                  │
│  SQLite             sqlite-vec            Crawler / File Watcher  │
│  (docs, sources,    (embeddings,          (web, bookmarks,        │
│   history, settings) opsional)             folder lokal)          │
└─────────────────────────────────────────────────────────────────┘
```

### 2.3 Alur Data

1. **Ingest**: crawler / file watcher / import bookmark mengambil konten.
2. **Extract**: bersihkan HTML → teks utama, judul, deskripsi, bahasa, favicon.
3. **Index**: simpan metadata ke SQLite, teks ke Tantivy, (opsional) embedding ke sqlite-vec.
4. **Query**: user mengetik → orchestrator query Tantivy (BM25) + (opsional) vector + (opsional) SearXNG.
5. **Rank**: gabungkan hasil dengan **Reciprocal Rank Fusion (RRF)**.
6. **Render**: hasil ditampilkan dengan highlight snippet.

### 2.4 Skema Database (SQLite)

```sql
CREATE TABLE sources (
  id INTEGER PRIMARY KEY,
  kind TEXT NOT NULL,            -- 'web' | 'folder' | 'bookmark' | 'history'
  uri TEXT NOT NULL UNIQUE,
  label TEXT,
  crawl_depth INTEGER DEFAULT 1,
  refresh_hours INTEGER DEFAULT 168,
  enabled INTEGER DEFAULT 1,
  last_run_at TEXT
);

CREATE TABLE documents (
  id INTEGER PRIMARY KEY,
  source_id INTEGER REFERENCES sources(id) ON DELETE CASCADE,
  url TEXT NOT NULL UNIQUE,
  title TEXT,
  description TEXT,
  content_hash TEXT,
  lang TEXT,
  favicon BLOB,
  word_count INTEGER,
  fetched_at TEXT,
  pinned INTEGER DEFAULT 0
);

CREATE TABLE tags (
  id INTEGER PRIMARY KEY,
  name TEXT UNIQUE
);

CREATE TABLE document_tags (
  document_id INTEGER REFERENCES documents(id) ON DELETE CASCADE,
  tag_id INTEGER REFERENCES tags(id) ON DELETE CASCADE,
  PRIMARY KEY (document_id, tag_id)
);

CREATE TABLE search_history (
  id INTEGER PRIMARY KEY,
  query TEXT NOT NULL,
  result_count INTEGER,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE settings (
  key TEXT PRIMARY KEY,
  value TEXT
);
```

### 2.5 Perintah IPC (Rust ↔ Frontend)

| Command                                       | Fungsi                                                         |
| --------------------------------------------- | -------------------------------------------------------------- |
| `search(query, filters, mode)`                | Jalankan pencarian (`keyword` / `semantic` / `hybrid` / `web`) |
| `suggest(prefix)`                             | Autocomplete dari riwayat + judul dokumen                      |
| `add_source(kind, uri, options)`              | Tambah sumber crawl / folder / bookmark                        |
| `crawl_now(source_id)`                        | Mulai crawl manual                                             |
| `get_crawl_progress()`                        | Event stream progres crawler                                   |
| `open_document(id)`                           | Buka URL / file di browser atau aplikasi default               |
| `pin_document(id)` / `tag_document(id, tags)` | Organisasi                                                     |
| `export_library()` / `import_library()`       | Backup ke file `.zip`                                          |
| `get_stats()`                                 | Jumlah dokumen, ukuran index, dll.                             |

### 2.6 Privasi & Etika Crawler

- Wajib baca & patuhi `robots.txt`
- User-Agent jelas: `PrivateSearchBot/1.0 (personal use)`
- Rate limit per domain (default 1 request / 2 detik)
- Tidak ada telemetri, tidak ada panggilan jaringan selain yang diminta user
- Semua data di folder app data lokal, bisa dienkripsi (opsi SQLCipher)

---

## 3. Spesifikasi Desain UI/UX

**Gaya:** Dark Editorial UI + Swiss International Typographic Style + Neo-Brutalism + Experimental Motion Design.
**Referensi visual:** website ArtCraft (getartcraft.com).

### 3.1 Elemen Visual Khas yang Harus Ditiru (dari referensi)

1. **Latar gelap hampir hitam** dengan tekstur sangat halus; busur/orbit melengkung tipis bergradasi (biru, ungu, hijau, merah) sebagai dekorasi latar.
2. **Wordmark raksasa** berhuruf _extended_ tebal, uppercase, dengan gradien putih ke abu di tepi bawah.
3. **Headline campuran**: sans-serif extended tebal + **satu kata dalam serif italic** sebagai aksen. Contoh: "Search that _belongs to you._"
4. **Label monospace uppercase** kecil dengan letter-spacing lebar untuk navigasi, tombol, dan metadata.
5. **Grid hairline 1px**: sel-sel dengan garis tipis seperti tabel editorial (lihat grid "Sixteen apps").
6. **Penomoran seksi**: `01 / CRAFTING APPS`, `02 / ARTCRAFT STUDIO`, `03 / STUDIO FEATURES`.
7. **Penggaris scroll** di tepi kanan dengan tick angka (0, 5, 15, 20) dan indikator persen (`012%`, `023%`).
8. **Indeks seksi di pojok kanan bawah**, teks uppercase rata kanan, item aktif lebih terang dan item lain redup.
9. **Ticker / marquee** horizontal berisi fitur dipisah titik biru.
10. **Tombol**: primer = kotak putih solid teks hitam mono; sekunder = outline putih tipis. Sudut **tajam (radius 0)**.
11. **Kartu mengambang** berlinier tipis (rounded rect samar) tersebar di latar hero dengan efek parallax.
12. **Navbar tipis** dengan pemisah vertikal 1px, item aktif berlatar putih (seperti "HOME"), tombol CTA putih di pojok kanan.

### 3.2 Design Tokens

```css
:root {
	--bg: #0d0e12;
	--bg-deep: #050506;
	--surface: #14151a;
	--line: rgba(255, 255, 255, 0.12);
	--line-strong: rgba(255, 255, 255, 0.28);
	--fg: #f5f5f7;
	--fg-dim: rgba(245, 245, 247, 0.62);
	--fg-faint: rgba(245, 245, 247, 0.34);
	--accent-blue: #2f6bff;
	--accent-purple: #8a2be2;
	--accent-green: #3ddc84;
	--accent-red: #e5484d;

	--font-display: 'Unbounded', 'Syne', 'Monument Extended', sans-serif; /* extended bold */
	--font-serif: 'Instrument Serif', 'Playfair Display', serif; /* italic accent */
	--font-body: 'Inter Tight', 'Inter', sans-serif;
	--font-mono: 'JetBrains Mono', 'Space Mono', monospace;

	--radius: 0px;
	--grid-gap: 1px;
}
```

### 3.3 Layar Utama

| Layar                 | Isi                                                                                                                                                                   |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Home / Search**     | Wordmark raksasa, kotak search besar bergaya brutalist (border tebal, fokus berkedip), tombol mode (Keyword / Semantic / Hybrid / Web), ticker statistik              |
| **Results**           | Daftar hasil bergaya editorial: nomor urut `01`, judul besar, URL mono, snippet dengan highlight, tag, tombol pin. Filter di sisi kiri (sumber, tanggal, bahasa, tag) |
| **Library**           | Grid hairline seperti "Sixteen apps": setiap sel adalah sumber/koleksi dengan ikon, jumlah dokumen, status crawl                                                      |
| **Sources & Crawler** | Tambah URL / folder, atur depth & jadwal, log progres real-time bergaya terminal                                                                                      |
| **Command Palette**   | `Ctrl/Cmd + K` untuk pindah layar, cari cepat, jalankan aksi                                                                                                          |
| **Settings**          | Tema, font, privasi, lokasi data, SearXNG endpoint, model embedding, backup                                                                                           |

### 3.4 Motion & Interaksi

- Intro: wordmark muncul huruf per huruf (stagger) + busur orbit menggambar dirinya (stroke-dashoffset).
- Parallax pada kartu mengambang mengikuti kursor.
- Penggaris scroll dan persen tersinkron dengan scroll.
- Hover pada sel grid: garis menebal, ikon bergeser 4px, label mono berubah invert (putih/hitam).
- Transisi antar layar: wipe horizontal bergaya editorial (200-350 ms).
- Hasil search muncul dengan stagger 30 ms per item.
- Hormati `prefers-reduced-motion`.

### 3.5 Aksesibilitas

- Kontras teks minimal WCAG AA
- Navigasi penuh dengan keyboard, focus ring jelas (outline putih 2px)
- Ukuran teks bisa diatur di Settings
- Mode terang opsional (toggle ikon matahari di navbar, seperti referensi)

---

## 4. 🧠 MASTER PROMPT (Tempel ke AI Coding Agent)

> Salin seluruh blok di bawah ke Claude Code / agent coding lain.

```text
ROLE
You are a senior full-stack engineer and award-winning product designer. Build a complete, production-quality desktop application called "Private Search": a personal, local-first search engine.

GOAL
A cross-platform desktop app (Windows, macOS, Linux) that runs entirely on the user's machine. It indexes web pages the user chooses to crawl, bookmarks, browsing history exports, and local documents, then lets the user search them instantly. Optionally it can query a local SearXNG instance for general web results and use local embeddings (Ollama) for semantic search. No cloud, no accounts, no telemetry.

TECH STACK (follow unless you find a blocking issue, then explain the change)
- App shell: Tauri 2 (Rust)
- Frontend: SvelteKit + TypeScript + Tailwind CSS + GSAP (ScrollTrigger)
- Full-text search: Tantivy
- Metadata DB: SQLite (sqlx), migrations included
- Crawler: reqwest + scraper + readability extraction, obeying robots.txt, per-domain rate limiting, content-hash dedupe
- File ingestion: txt, md, pdf, docx with a folder watcher (notify crate)
- Optional semantic search: Ollama embeddings (nomic-embed-text) stored with sqlite-vec; hybrid ranking with Reciprocal Rank Fusion
- Optional metasearch: SearXNG over localhost HTTP (JSON format)
- Background jobs: tokio + cron scheduler for periodic re-crawls

VISUAL DESIGN (critical, follow exactly)
Design a premium, modern, visually immersive interface using a combination of Dark Editorial UI, Swiss International Typographic Style, Neo-Brutalism, and Experimental Motion Design. Take direct inspiration from the ArtCraft website:
- Near-black background (#0d0e12) with faint thin gradient orbit arcs (blue, purple, green, red) and faint outlined floating cards in the hero with cursor parallax.
- Giant uppercase extended-bold wordmark "PRIVATE SEARCH" with a white-to-grey vertical gradient.
- Headlines mix an extended bold sans-serif with ONE italic serif accent word (e.g. "Search that *belongs to you.*").
- Small uppercase monospace labels with wide letter-spacing for nav, buttons, metadata.
- 1px hairline grid layouts (like the "Sixteen apps. One craft." app grid) for the Library screen.
- Section numbering such as "01 / SEARCH", "02 / LIBRARY", "03 / SOURCES".
- A vertical scroll ruler on the right edge with tick numbers and a live percentage readout.
- A right-aligned section index at bottom-right where the active item is bright and others are dim.
- A horizontal marquee ticker with live stats (documents indexed, index size, last crawl) separated by blue dots.
- Buttons: primary = solid white rectangle with black mono text; secondary = thin white outline; ALL corners sharp (radius 0).
- Thin top navbar with 1px vertical dividers, active item as a white filled tab, white CTA at the far right, theme toggle icon.
- Include a light theme variant, but dark is the default and hero experience.
Motion: staggered letter reveal for the wordmark, SVG stroke-draw for arcs, parallax floating cards, wipe transitions between screens (200-350 ms), staggered result entrance (30 ms), hover states that invert mono labels and thicken hairlines. Respect prefers-reduced-motion. Keep 60 fps.

SCREENS
1. Home/Search: hero, large brutalist search box, mode switch (Keyword | Semantic | Hybrid | Web), recent searches, ticker.
2. Results: editorial list (index number, large title, mono URL, highlighted snippet, tags, pin button), left filter rail (source, date, language, tags), keyboard navigation (j/k, Enter, /).
3. Library: hairline grid of sources/collections with doc counts and crawl status.
4. Sources & Crawler: add URL/folder/bookmark file, depth and schedule options, live terminal-style crawl log, pause/resume.
5. Command Palette (Ctrl/Cmd+K).
6. Settings: theme, fonts, data location, SearXNG endpoint, embedding model, backup/restore, privacy summary.

FUNCTIONAL REQUIREMENTS
- Autocomplete suggestions from history and document titles (<50 ms).
- Query syntax: "exact phrase", -exclude, site:example.com, tag:name, before:/after: dates.
- Highlighted snippets, pinning, tagging, open in default browser.
- Import bookmarks (Chrome/Firefox/Edge HTML or JSON) and optional browser history export.
- Export/import the whole library as a .zip.
- Everything works offline except crawling and the optional SearXNG mode.
- Strong error handling and visible, human-readable status messages.

DATABASE
Use this schema (create migrations): sources, documents, tags, document_tags, search_history, settings. Store the Tantivy index in the app data directory.

IPC COMMANDS (Rust)
search, suggest, add_source, crawl_now, get_crawl_progress, open_document, pin_document, tag_document, export_library, import_library, get_stats.

QUALITY BAR
- Clean architecture: Rust modules (crawler, indexer, search, store, ipc), typed frontend API layer.
- Unit tests for query parsing, ranking fusion, and crawler politeness. A few integration tests for indexing + searching.
- Lint/format configured (clippy, rustfmt, eslint, prettier).
- Accessible (WCAG AA contrast, full keyboard use, visible focus).
- README with setup, build, and usage instructions.

DELIVERY PLAN
Work in phases and verify each one runs before moving on:
Phase 1 scaffold + design system + static screens
Phase 2 SQLite + Tantivy + manual document add + working keyword search
Phase 3 crawler + scheduler + live progress
Phase 4 file ingestion + bookmark import
Phase 5 semantic/hybrid search + SearXNG mode
Phase 6 motion polish, settings, backup, packaging installers
After each phase, summarize what works and what remains. Ask me only when a decision truly blocks progress; otherwise choose sensible defaults and tell me.
```

---

## 5. Struktur Folder yang Disarankan

```
private-search/
├── README.md
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── migrations/
│   └── src/
│       ├── main.rs
│       ├── ipc/            # command handlers
│       ├── store/          # SQLite access
│       ├── indexer/        # Tantivy
│       ├── search/         # query parser, ranking, RRF
│       ├── crawler/        # fetch, robots, extract, scheduler
│       ├── ingest/         # pdf/docx/md, bookmarks, watcher
│       └── embed/          # Ollama + sqlite-vec (opsional)
├── src/
│   ├── lib/
│   │   ├── api/            # wrapper invoke() bertipe
│   │   ├── components/     # Ticker, ScrollRuler, SectionIndex, HairlineGrid, ...
│   │   ├── motion/         # GSAP timelines
│   │   └── styles/         # tokens.css, global.css
│   └── routes/
│       ├── +page.svelte        # Home/Search
│       ├── results/
│       ├── library/
│       ├── sources/
│       └── settings/
└── docs/
```

---

## 6. Roadmap

| Fase | Target                    | Hasil                                                          |
| ---- | ------------------------- | -------------------------------------------------------------- |
| 1    | Scaffold + design system  | App terbuka, UI statis sesuai desain                           |
| 2    | SQLite + Tantivy          | Tambah dokumen manual, keyword search jalan                    |
| 3    | Crawler + scheduler       | Crawl situs pilihan, progres real-time                         |
| 4    | File & bookmark ingestion | Index folder lokal & bookmark                                  |
| 5    | Semantic + SearXNG        | Hybrid ranking, hasil web privat                               |
| 6    | Polish & packaging        | Animasi final, backup, installer `.exe` / `.dmg` / `.AppImage` |

---

## 7. Acceptance Criteria (Definisi "Selesai")

- [ ] Instalasi lewat satu installer, jalan tanpa server eksternal
- [ ] Search di 10.000+ dokumen terasa instan (< 100 ms)
- [ ] Crawler patuh robots.txt dan rate limit
- [ ] UI sesuai spesifikasi: grid hairline, penggaris scroll, ticker, tipografi campuran
- [ ] Semua fitur inti bisa dipakai tanpa mouse
- [ ] Tidak ada koneksi jaringan di luar yang diminta user
- [ ] Backup & restore library berfungsi
- [ ] Lulus lint dan test

---

## 8. Prasyarat Pengembangan

```bash
# Rust toolchain
curl https://sh.rustup.rs -sSf | sh

# Node.js >= 20
# Tauri CLI
cargo install tauri-cli --version "^2"

# Opsional
docker run -d -p 8888:8080 searxng/searxng     # metasearch lokal
ollama pull nomic-embed-text                   # embedding lokal
```

Jalankan: `cargo tauri dev` &nbsp;|&nbsp; Build: `cargo tauri build`

---

## 9. Catatan Penting

- Mulai dari **Fase 1-2** dulu. Mesin pencari yang bisa mencari data milikmu sendiri sudah sangat berguna.
- Crawl hanya situs yang kamu izinkan/butuhkan; patuhi ketentuan layanan situs.
- Semantic search & SearXNG bersifat **opsional**, jadi app tetap berfungsi tanpa keduanya.
- Font (Unbounded, Instrument Serif, JetBrains Mono) tersedia gratis di Google Fonts. Sertakan secara lokal agar app tetap offline.

---

_Dibuat sebagai blueprint proyek. Bebas dimodifikasi sesuai kebutuhan._
