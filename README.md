# 🔎 QORI: Personal, Local-First Search Engine

> A cross-platform desktop application (Windows, macOS, Linux) that runs **100% locally** on your machine.
> Indexes web pages, local markdown/text files, and browser bookmarks with sub-millisecond Tantivy full-text retrieval, optional Ollama vector embeddings, and private SearXNG metasearch.
> **No cloud accounts. No subscriptions. Zero telemetry. 100% air-gapped ready.**

---

## 🏛️ Architecture & Tech Stack

```
┌─────────────────────────── Tauri 2 Desktop Shell ───────────────────────────┐
│                                                                             │
│  Frontend: SvelteKit + TypeScript + Tailwind CSS v4                         │
│  UI Style: Dark Editorial UI, Swiss Typography & Neo-Brutalism              │
│                                                                             │
│                        ◄─── IPC (invoke / events) ───►                      │
│                                                                             │
│  Rust Backend Core:                                                         │
│  ├── Tantivy 0.22           Embedded Lucene-style BM25 full-text engine     │
│  ├── SQLite (rusqlite)      ACID metadata store with WAL mode               │
│  ├── Crawler Engine         Reqwest + Scraper + Robots.txt politeness       │
│  ├── Ingestion Pipelines    Markdown, Netscape HTML & JSON Bookmarks        │
│  ├── Vector Search (opt)    Ollama (nomic-embed-text) + Cosine Similarity   │
│  ├── Rank Fusion (RRF)      Reciprocal Rank Fusion 1 / (60 + rank)          │
│  └── Metasearch (opt)       Localhost SearXNG JSON query orchestrator       │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 🎨 Visual Design System (ArtCraft Inspired)

- **Dark Editorial Aesthetic**: `#0d0e12` near-black background with faint gradient orbit arcs (blue, purple, green, red) and cursor parallax floating cards.
- **Micro-Density Typography**: Extended bold uppercase wordmark `"QORI"` with white-to-gray vertical gradient and staggered letter reveal.
- **Mixed Headline**: Extended sans-serif with a single italic serif accent word: *"Search that belongs to you."*
- **1px Hairline Matrix**: Grid layout for Library collections and Settings.
- **Scroll Ruler**: Vertical scroll ruler along the right edge with numbered ticks (`00`, `05`, `10`...) and real-time scroll percentage readout (`012%`, `054%`).
- **Section Index**: Fixed bottom-right index highlighting active view (`01 / SEARCH`, `02 / RESULTS`, `03 / LIBRARY`, `04 / SOURCES`, `05 / SETTINGS`).
- **Horizontal Marquee Ticker**: Real-time stats across index size, document counts, and latency separated by blue dots (`•`).
- **Neo-Brutalist Controls**: Radius 0 sharp corners, solid white primary buttons with inverted text, thin hairline secondary buttons.
- **Theme Support**: Dark mode by default with instant toggle to high-contrast Light mode.

---

## ⚡ Keyboard Navigation & Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl + K` or `⌘ + K` | Open Command Palette (jump to screens, search actions) |
| `/` | Instant focus on the main query search box |
| `J` | Select next search result |
| `K` | Select previous search result |
| `Enter` | Open highlighted search result in default browser / desktop app |
| `Esc` | Close modals and suggestion dropdowns |

---

## 🔍 Query Syntax

Qori features an advanced syntax parser:

- `"exact phrase"` — Matches exact contiguous token sequences.
- `-word` — Strictly excludes documents containing `word`.
- `site:domain.com` — Restricts results to a specific domain or host.
- `tag:name` — Filters documents assigned to `#name`.
- `before:YYYY-MM-DD` / `after:YYYY-MM-DD` — Temporal boundary filters.

---

## 🚀 Getting Started

### Prerequisites

- **Rust toolchain** (1.90 or newer): [https://rustup.rs](https://rustup.rs)
- **Node.js** (v20 or newer) and **npm**
- **Tauri 2 CLI**: included via npm `@tauri-apps/cli`

*(Optional for Semantic & Metasearch modes)*
- **Ollama**: `ollama pull nomic-embed-text`
- **SearXNG**: `docker run -d -p 8888:8080 searxng/searxng`

### Installation & Development

```bash
# 1. Clone repository
git clone https://github.com/qori/private-search.git
cd private-search

# 2. Install frontend dependencies
npm install

# 3. Run development server (hot-reload frontend + Rust native window)
npx @tauri-apps/cli dev

# Or run frontend in browser preview mode
npm run dev
```

### Production Build

```bash
# Build native desktop executable & installer (.msi / .exe on Windows, .dmg on macOS, .AppImage on Linux)
npx @tauri-apps/cli build
```

---

## 🧪 Testing & Verification

```bash
# Run all Rust backend unit and integration tests
cargo test --manifest-path src-tauri/Cargo.toml

# Run Clippy static analysis
cargo clippy --manifest-path src-tauri/Cargo.toml

# Run Svelte type-checking
npm run check

# Check Rust and frontend formatting
cargo fmt --check --manifest-path src-tauri/Cargo.toml
npm run format
```

---

## 🔒 Privacy & Local-First Guarantee

- **No Remote Telemetry**: Zero analytics, tracking pixels, or diagnostic beacons.
- **Air-Gapped Operation**: Search, indexing, ranking, and autocomplete execute locally.
- **Crawler Politeness**: Strictly obeys `robots.txt` per target domain and enforces default 2-second rate limits.
- **Portable Backups**: Export and restore your complete library into a single `.zip` archive.
