# QORI: Prompt Revisi UI/UX (Terinspirasi Opera, Lebih Nyaman Dilihat)

## Latar Belakang

Feedback pengguna: tampilan versi lama (Neo-Brutalism, sudut tajam, kontras keras, grid) terasa **kurang nyaman dilihat**. Arah baru: **lembut, tenang, rapi, dan familiar**, dengan referensi layout dari Opera.

## Apa yang diambil dari referensi Opera (ide, bukan salinan)

| Elemen di referensi | Diterapkan di QORI |
|---|---|
| Sidebar vertikal ramping di kiri berisi ikon | Sidebar QORI: Home, Bookmarks, Workspaces, web apps tersemat, Favorites, History, "..." |
| Tombol aktif berbentuk kotak membulat berwarna aksen | Item sidebar aktif: pill/kotak membulat berwarna ungu aksen |
| Tab bar di atas dengan tab putih membulat dan tombol "+" | Tab bar pill membulat, tab aktif terang |
| Baris navigasi: back, forward, reload, badge VPN, kolom alamat | Dipertahankan, dibungkus dalam satu "pill" besar |
| Ikon di kanan address bar (screenshot, download, dll.) | **Screenshot, Download, Hamburger menu dipertahankan** |
| Area konten dengan sudut membulat di atas wallpaper gradien | Konten membulat + wallpaper gradien lembut |
| Panel kaca (glassmorphism) transparan | Chrome UI semi-transparan dengan blur |
| Start page: kotak pencarian besar membulat + grid speed dial 4x2 | Start page QORI dengan pencarian pill besar + grid situs favorit |

**Yang sengaja TIDAK disalin:** logo, nama, ikon merek, dan aset Opera. QORI tetap punya identitas sendiri (wordmark QORI, warna aksen, kata italic pada tagline).

---

## MASTER PROMPT

````text
ROLE
You are a senior product designer and front-end engineer. Redesign the UI of the existing desktop browser "QORI" (Tauri 2 + SvelteKit + TypeScript + Tailwind). This is a VISUAL AND LAYOUT REVISION ONLY: keep all existing functionality, IPC commands, data model and shortcuts working. Replace the old Neo-Brutalist / hairline-grid look with a calm, soft, comfortable-to-look-at interface.

REFERENCE
Take layout inspiration from the Opera browser (slim icon sidebar, rounded pill tab bar, rounded address bar with inline badges, rounded content area over a gradient wallpaper, frosted-glass panels, start page with a big rounded search pill and a speed-dial grid). Take inspiration only. Do NOT copy Opera's logo, name, icons or any branding. QORI keeps its own identity.

DESIGN GOALS
- Comfortable for long sessions: soft contrast, generous spacing, rounded shapes, no harsh borders, no visual noise.
- Familiar: layout should feel instantly understandable to anyone who has used a modern browser.
- Fast and light: glass effects must be cheap (limit blur layers, provide a "Reduce transparency" setting that swaps glass for solid colors).
- Readable on any wallpaper: always place a subtle scrim behind text so contrast stays WCAG AA.

DESIGN TOKENS
- Radius: 10px (controls), 14px (cards, tiles), 18px (content area, panels), 999px (pills, search).
- Spacing: 4px scale; outer gutter 8px between chrome pieces.
- Typography: UI font "Plus Jakarta Sans" or "Inter" (400/500/600). Keep a serif italic (e.g. "Instrument Serif") ONLY for the one accent word in the tagline and the wordmark flourish. Monospace only for keyboard shortcut hints and URLs in small size.
- Colors (define as CSS variables, with light and dark sets):
  Light: bg-chrome rgba(236,239,248,0.72), surface #FFFFFF, text #1B1D2A, text-dim #5E6275, accent #5B3DF5, accent-soft rgba(91,61,245,0.14), border rgba(20,22,40,0.08).
  Dark: bg-chrome rgba(22,24,36,0.72), surface #1C1E2C, text #F2F3F8, text-dim #A3A7BC, accent #8B7BFF, accent-soft rgba(139,123,255,0.18), border rgba(255,255,255,0.08).
- Glass: backdrop-filter blur(24px) saturate(140%) on chrome (sidebar, tab bar, nav bar) only. Content webviews are never blurred.
- Shadows: soft, large radius, low opacity (e.g. 0 8px 30px rgba(0,0,0,0.12)).
- Default wallpaper: soft multi-stop gradient (blue, lavender, pink) with gentle blurred shapes; ship 6 wallpapers (3 light, 3 dark) plus "custom image" and "solid color". Wallpaper dim and blur sliders in Settings.

WINDOW LAYOUT (top to bottom, left to right)
1. SIDEBAR (left, 56px wide collapsed, 264px expanded, toggle with Ctrl+B or a chevron on hover)
   - Top: QORI logo mark (small), then Home (Start page) as a rounded accent-filled button when active, then Bookmarks (star).
   - Workspaces: round colored avatar icons stacked vertically (click to switch, "+" to add), active one has an accent ring.
   - Middle: pinned web apps / shortcuts chosen by the user (favicons in rounded 36px tiles with subtle surface background), drag to reorder, right-click to unpin.
   - Bottom: Favorites (heart), History (clock), Library (QORI index), Sources (crawler), and "..." for sidebar customization.
   - Expanded mode shows labels, the vertical tab list (favicon, title, close button, sleep indicator, tab stacks collapsible), and a small memory meter in the footer.
   - Active item = rounded accent pill. Hover = soft accent-soft background. No hard borders.
2. TOP BAR (height ~44px)
   - Tabs as rounded pills on the chrome background: active tab = white/surface pill with favicon + title + close on hover; inactive tabs = transparent with dim text. "+" new tab button as a small circular button.
   - Right side of this row: a search icon button that opens the command palette (hint "Ctrl K" on hover), then window controls (minimize, maximize, close) styled softly.
   - Option in Settings to hide the tab bar when using vertical tabs.
3. NAV BAR (height ~52px), one large rounded pill container (radius 14px) with a soft surface background, containing from left to right:
   - Back, forward, reload buttons (icon buttons, disabled state dimmed).
   - The purple "VPN" badge (KEEP). It shows state: off (neutral), on (accent filled) and a tooltip with the proxy/server label. Click opens a small popover to toggle and choose proxy.
   - Address / search field: placeholder "Search with QORI or enter web address", magnifier icon, shows site security icon + shield counter when on a page.
   - Right side icons (KEEP): Screenshot (camera), Download (arrow with a badge for active downloads), then a profile/workspace button. Add the shields/privacy icon and the memory-saver icon (replaces battery) if space allows.
   - The HAMBURGER MENU (KEEP) as the last icon in its own small rounded pill to the right of the address bar. It opens a clean panel with: New tab, New private window, History, Downloads, Bookmarks, Library, Sources, Settings, Themes & Wallpaper, Shortcuts, Privacy report, About. Show keyboard shortcuts at the right of each row.
   - Optional small "Command" button (shows Ctrl K) in a separate rounded pill to the right.
4. CONTENT AREA
   - Rounded container (radius 18px), 8px gutter from the edges, sitting over the wallpaper. Web tabs render inside it. Split / tiling layouts keep rounded corners per pane with a 8px soft gap instead of hairlines; the focused pane shows a thin accent outline.

START PAGE (replaces the old hero)
- Full-bleed wallpaper inside the rounded content area.
- Centered large rounded search pill (about 840px wide, 72px tall, white/surface, soft shadow): QORI logo mark on the left, placeholder "Search your library or the web", search icon button on the right. Below or inside it, small mode chips: Library | Hybrid | Web (default Hybrid), keyboard accessible.
- Under the search pill: small QORI wordmark with the tagline "Personal search that *belongs* entirely to you." (accent word in serif italic). Keep it subtle, not huge.
- Speed dial: grid of 4 columns x 2 rows of rounded 14px tiles (about 200x120) with frosted background, centered favicon/icon, label below. Last tile is "Add a site" with a plus. Drag to reorder, right-click to edit/remove. Show a pin for frequently visited sites automatically (user can disable).
- Hover on tiles: slight lift (translateY -2px) and brighter surface, 150 ms ease.

OTHER SCREENS (restyle with the same language)
- Results page: single-column list of rounded cards, large readable title, small muted URL, snippet with soft highlight (accent-soft background), tags as small pills, pin and open actions on hover. Left filter rail collapsible, filters as pill chips.
- Library, Sources, Settings: open as sidebar panels or as internal pages inside the content area using the same cards, toggles, sliders and pill buttons. Settings has a search box at the top.
- Command palette (Ctrl+K): centered rounded glass panel (radius 18px, 640px wide), large input, grouped results with icons, active row highlighted by accent-soft pill, shortcut hints on the right.
- Context menus, popovers, tooltips, dialogs: rounded 12px, glass or surface, soft shadow, consistent spacing.
- Downloads popover: list with file icon, name, progress bar (accent), actions.
- Screenshot flow: small popover with "Visible area / Full page / Selection" choices.

MOTION
- Subtle and fast: 120-200 ms ease-out for hover, press, popovers; sidebar expand/collapse 200 ms; tab open/close slide-fade; workspace switch crossfade 180 ms.
- No decorative continuous animation. Respect prefers-reduced-motion. Keep 60 fps and low CPU when idle.

ACCESSIBILITY AND COMFORT
- WCAG AA contrast for all text, including over wallpapers (use scrim).
- Full keyboard navigation with visible focus ring (2px accent outline with offset).
- UI scale setting (90% / 100% / 110% / 125%), density (compact / comfortable), and a "Reduce transparency" toggle.
- Light, Dark, and Auto (follow system) themes, with the theme toggle available from the hamburger menu and the command palette.

CUSTOMIZATION HOOKS (so users can personalize later)
- All colors, radii and blur values come from CSS variables in one tokens file.
- Settings > Appearance: theme, accent color picker, wallpaper, wallpaper dim/blur, sidebar position (left/right), sidebar size, show/hide tab bar, nav bar icon order, density, UI scale.

IMPLEMENTATION NOTES
- Refactor the chrome into components: Sidebar, WorkspaceSwitcher, TabPill, TabBar, NavPill, AddressField, VpnBadge, IconButton, HamburgerMenu, StartPage, SpeedDial, SearchPill, ResultCard, CommandPalette, Popover.
- Keep existing IPC wiring and keyboard shortcuts untouched. Only presentation and layout change.
- Provide a short style guide page (route /design) showing tokens and all components in light and dark.
- Remove the old grid background, hairline dividers, scroll ruler, marquee ticker and sharp 0px radius styles.

DELIVERABLES
1. Updated tokens + global styles.
2. Refactored components listed above.
3. Redesigned start page, results page, sidebar (collapsed and expanded), nav bar, hamburger menu, command palette.
4. Light and dark themes with 6 wallpapers.
5. A before/after summary and a list of any behavior you could not preserve.

After finishing each major piece (tokens, shell layout, start page, other screens), verify the app builds and runs, then summarize what changed.
````

---

## Catatan Perubahan dari Desain Lama

- **Hilang:** grid latar, garis 1px, sudut tajam, penggaris scroll, ticker, tampilan serba monospace.
- **Tetap:** VPN badge, Screenshot, Download, Hamburger menu, command palette, vertical tabs, workspaces, split view, semua shortcut.
- **Baru:** sidebar ikon ala browser modern, tab dan nav berbentuk pill, wallpaper gradien lembut, panel kaca, start page dengan speed dial.
- **Library, Sources, Settings** pindah dari tab atas ke sidebar dan hamburger menu agar tab bar fokus ke tab web.