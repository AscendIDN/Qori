<script lang="ts">
	import { onMount } from 'svelte';
	import Wallpaper, { type WallpaperId } from '../lib/components/Wallpaper.svelte';
	import Sidebar from '../lib/components/Sidebar.svelte';
	import TopBar from '../lib/components/TopBar.svelte';
	import Navbar from '../lib/components/Navbar.svelte';
	import StartPage from '../lib/components/StartPage.svelte';
	import ResultsScreen from '../lib/components/ResultsScreen.svelte';
	import SettingsScreen from '../lib/components/SettingsScreen.svelte';
	import WebTabScreen from '../lib/components/WebTabScreen.svelte';
	import CommandPalette from '../lib/components/CommandPalette.svelte';
	import ManualDocModal from '../lib/components/ManualDocModal.svelte';
	import type {
		SearchMode,
		SearchFilter,
		SearchResponse,
		AppStats,
		DocumentItem,
		BrowserTab
	} from '../lib/types';
	import { api } from '../lib/api';

	let tabs = $state<BrowserTab[]>([
		{ id: 'tab-1', title: 'Start Page', page: 'search', query: '' }
	]);
	let activeTabId = $state('tab-1');
	let activeTab = $derived(tabs.find((t) => t.id === activeTabId) ?? tabs[0]);
	let settingsSection = $state<'general' | 'library' | 'sources'>('general');

	// Appearance & Customization Tokens
	let theme = $state<'dark' | 'light'>('dark');
	let wallpaper = $state<WallpaperId>('dark-lavender');
	let wallpaperDim = $state(0);
	let wallpaperBlur = $state(0);
	let reduceTransparency = $state(false);

	// Modals & Panels
	let isCommandPaletteOpen = $state(false);
	let isManualModalOpen = $state(false);

	// Search State
	let currentQuery = $state('');
	let currentMode = $state<SearchMode>('hybrid');
	let searchModeUsed = $state<string>('hybrid');
	let searchFallback = $state<string | null>(null);
	let searchResults = $state<DocumentItem[]>([]);
	let searchDurationMs = $state(0);
	let searchTotal = $state(0);

	// App Stats
	let stats = $state<AppStats>({
		total_documents: 1985,
		total_sources: 4,
		index_size_bytes: 14820412,
		last_crawl_at: '2026-11-05 09:15:30',
		search_count: 87
	});

	async function loadInitialStats() {
		try {
			stats = await api.getStats();
		} catch (err) {
			console.warn('Failed to load initial stats:', err);
		}
	}

	async function handleSearch(query: string, mode: SearchMode, filter?: SearchFilter) {
		currentQuery = query;
		currentMode = mode;
		const cur = tabs.find((t) => t.id === activeTabId);
		if (cur) {
			cur.query = query;
			cur.title = query;
			cur.page = 'results';
		}
		try {
			const resp = await api.search(query, mode, filter);
			searchResults = resp.documents;
			searchDurationMs = resp.duration_ms;
			searchTotal = resp.total;
			searchModeUsed = resp.mode ?? mode;
			searchFallback = resp.fallback ?? null;
		} catch (err) {
			console.error('Search failed:', err);
		}
	}

	function handleNewTab() {
		const newId = 'tab-' + Date.now();
		tabs = [...tabs, { id: newId, title: 'Start Page', page: 'search', query: '' }];
		activeTabId = newId;
		currentQuery = '';
	}

	function handleSelectTab(id: string) {
		activeTabId = id;
		const t = tabs.find((x) => x.id === id);
		if (t) {
			currentQuery = t.query;
			if (t.settingsSection) {
				settingsSection = t.settingsSection;
			}
		}
	}

	function handleCloseTab(id: string) {
		if (tabs.length === 1) {
			tabs[0].title = 'Start Page';
			tabs[0].page = 'search';
			tabs[0].query = '';
			currentQuery = '';
			return;
		}
		const index = tabs.findIndex((t) => t.id === id);
		const wasActive = activeTabId === id;
		tabs = tabs.filter((t) => t.id !== id);
		if (wasActive) {
			const nextTab = tabs[Math.max(0, index - 1)];
			activeTabId = nextTab.id;
			currentQuery = nextTab.query;
		}
	}

	function handleOpenSettings(section: 'general' | 'library' | 'sources' = 'general') {
		settingsSection = section;
		const cur = tabs.find((t) => t.id === activeTabId);
		if (cur) {
			cur.page = 'settings';
			cur.title = section === 'sources' ? 'Sources' : section === 'library' ? 'Library' : 'Settings';
			cur.settingsSection = section;
		}
	}

	function handleSidebarSelectPage(page: 'search' | 'library' | 'sources' | 'settings') {
		if (page === 'search') {
			const cur = tabs.find((t) => t.id === activeTabId);
			if (cur) {
				cur.page = 'search';
				cur.title = 'Start Page';
				cur.query = '';
				cur.url = undefined;
				currentQuery = '';
			}
		} else {
			handleOpenSettings(page === 'settings' ? 'general' : page);
		}
	}

	function handleOpenUrl(url: string, title?: string) {
		const cur = tabs.find((t) => t.id === activeTabId);
		if (cur) {
			cur.page = 'web';
			cur.url = url;
			cur.title = title || url;
			cur.query = url;
			currentQuery = url;
		} else {
			const newId = 'tab-' + Date.now();
			tabs = [...tabs, { id: newId, title: title || url, page: 'web', query: url, url }];
			activeTabId = newId;
			currentQuery = url;
		}
	}

	function handleCommandNavigate(screen: 'search' | 'library' | 'sources' | 'settings') {
		handleSidebarSelectPage(screen);
	}

	function toggleTheme() {
		theme = theme === 'dark' ? 'light' : 'dark';
		document.documentElement.setAttribute('data-theme', theme);
	}

	async function handleExport() {
		try {
			const fn = await api.exportLibrary();
			alert(`Library exported successfully: ${fn}`);
		} catch (e) {
			console.warn('Export failed:', e);
		}
	}

	onMount(() => {
		loadInitialStats();
		document.documentElement.setAttribute('data-theme', theme);
	});
</script>

<div
	class="relative flex h-screen w-screen overflow-hidden bg-transparent text-[var(--text)] transition-colors duration-200 select-none"
>
	<!-- Dynamic Ambient Wallpaper System (6 Curated Multi-Stop Gradients + Dim/Blur) -->
	<Wallpaper
		{wallpaper}
		{theme}
		dim={wallpaperDim}
		blur={wallpaperBlur}
	/>

	<!-- Left Column: Slim Modern Browser Sidebar (64px collapsed / 264px expanded) -->
	<Sidebar
		{tabs}
		{activeTabId}
		activePage={activeTab.page}
		onSelectPage={handleSidebarSelectPage}
		onSelectTab={handleSelectTab}
		onCloseTab={handleCloseTab}
		onNewTab={handleNewTab}
		onOpenUrl={handleOpenUrl}
	/>

	<!-- Right Column: Browser Chrome (TopBar + NavBar) & Seamless Content Area -->
	<div class="relative z-10 flex flex-1 flex-col h-screen min-w-0 overflow-hidden">
		<!-- Top Bar: Pill Tabs + New Tab + Command Search + Window Controls -->
		<TopBar
			{tabs}
			{activeTabId}
			onSelectTab={handleSelectTab}
			onCloseTab={handleCloseTab}
			onNewTab={handleNewTab}
			onOpenCommandPalette={() => (isCommandPaletteOpen = true)}
		/>

		<!-- Nav Bar: Navigation Buttons, Purple VPN Badge, Search Field, Tools & Hamburger Menu -->
		<Navbar
			bind:tabs
			bind:activeTabId
			bind:query={currentQuery}
			mode={currentMode}
			onSearch={handleSearch}
			onNewTab={handleNewTab}
			onOpenSettings={handleOpenSettings}
			bind:theme
		/>

		<!-- Main Content Area: Seamless over wallpaper without enclosing card -->
		<main class="relative flex-1 min-h-0 overflow-y-auto overflow-x-hidden">
			{#key activeTab.id + '-' + activeTab.page}
				<div class="screen-wipe min-h-full">
					{#if activeTab.page === 'search'}
						<StartPage
							onSearch={handleSearch}
							onOpenUrl={handleOpenUrl}
							{stats}
						/>
					{:else if activeTab.page === 'results'}
						<ResultsScreen
							results={searchResults}
							query={currentQuery}
							mode={currentMode}
							modeUsed={searchModeUsed}
							fallbackMessage={searchFallback}
							durationMs={searchDurationMs}
							totalCount={searchTotal}
							onSearchAgain={handleSearch}
							onOpenWebTab={handleOpenUrl}
							onBackToHome={() => handleSidebarSelectPage('search')}
						/>
					{:else if activeTab.page === 'settings'}
						<SettingsScreen
							bind:theme
							bind:activeSection={settingsSection}
							bind:wallpaper
							bind:wallpaperDim
							bind:wallpaperBlur
							bind:reduceTransparency
							onManualAdd={() => (isManualModalOpen = true)}
						/>
					{:else if activeTab.page === 'web' && activeTab.url}
						<WebTabScreen
							url={activeTab.url}
							title={activeTab.title}
							onClose={() => handleSidebarSelectPage('search')}
						/>
					{/if}
				</div>
			{/key}
		</main>
	</div>

	<!-- Command Palette Modal (Ctrl+K or TopBar Search) -->
	<CommandPalette
		bind:isOpen={isCommandPaletteOpen}
		onNavigate={handleCommandNavigate}
		onToggleTheme={toggleTheme}
		onAddSource={() => (isManualModalOpen = true)}
		onExport={handleExport}
		onSearch={(q) => handleSearch(q, currentMode)}
		{theme}
	/>

	<!-- Manual Document Entry Modal -->
	<ManualDocModal
		bind:isOpen={isManualModalOpen}
		onAdded={() => {
			loadInitialStats();
			settingsSection = 'library';
		}}
	/>
</div>

<style>
	/* Subtle and fast content wipe (180ms ease-out) */
	@keyframes softWipe {
		0% {
			opacity: 0;
			transform: translateY(6px);
		}
		100% {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.screen-wipe {
		animation: softWipe 180ms cubic-bezier(0.16, 1, 0.3, 1) forwards;
	}
</style>
