<script lang="ts">
	import { onMount } from 'svelte';
	import Navbar from '../lib/components/Navbar.svelte';
	import ScrollRuler from '../lib/components/ScrollRuler.svelte';
	import BackgroundGrid from '../lib/components/BackgroundGrid.svelte';
	import ManualDocModal from '../lib/components/ManualDocModal.svelte';
	import HomeScreen from '../lib/components/HomeScreen.svelte';
	import ResultsScreen from '../lib/components/ResultsScreen.svelte';
	import SettingsScreen from '../lib/components/SettingsScreen.svelte';
	import WebTabScreen from '../lib/components/WebTabScreen.svelte';
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
		{ id: 'tab-1', title: 'New Tab', page: 'search', query: '' }
	]);
	let activeTabId = $state('tab-1');
	let activeTab = $derived(tabs.find((t) => t.id === activeTabId) ?? tabs[0]);
	let settingsSection = $state<'general' | 'library' | 'sources'>('general');

	let theme = $state<'dark' | 'light'>('dark');
	let isManualModalOpen = $state(false);

	// Search State
	let currentQuery = $state('');
	let currentMode = $state<SearchMode>('keyword');
	let searchModeUsed = $state<string>('keyword');
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
		stats = await api.getStats();
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
		window.scrollTo({ top: 0, behavior: 'smooth' });
	}

	function handleNewTab() {
		const newId = 'tab-' + Date.now();
		tabs = [...tabs, { id: newId, title: 'New Tab', page: 'search', query: '' }];
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
			tabs[0].title = 'New Tab';
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

	onMount(() => {
		loadInitialStats();
	});
</script>

<div
	class="relative min-h-screen bg-[var(--bg)] text-[var(--fg)] transition-colors duration-200 selection:bg-[var(--accent-blue)] selection:text-white"
>
	<!-- Background Architectural Grid (Kotak-kotak) -->
	<BackgroundGrid />

	<!-- Fixed Scroll Ruler (Right Edge) -->
	<ScrollRuler />

	<!-- Top Navbar with Tab Bar & Browser Query Bar -->
	<Navbar
		bind:tabs
		bind:activeTabId
		bind:query={currentQuery}
		mode={currentMode}
		totalCount={searchTotal}
		durationMs={searchDurationMs}
		onSearch={handleSearch}
		onNewTab={handleNewTab}
		onSelectTab={handleSelectTab}
		onCloseTab={handleCloseTab}
		onOpenSettings={handleOpenSettings}
		bind:theme
	/>

	<!-- Main Content Container with Wipe Transition -->
	<main class="relative z-10 w-full overflow-hidden">
		{#key activeTab.id + '-' + activeTab.page}
			<div class="screen-wipe min-h-[calc(100vh-80px)]">
				{#if activeTab.page === 'search'}
					<HomeScreen onSearch={handleSearch} {stats} />
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
						onOpenWebTab={(url, title) => {
							const cur = tabs.find((t) => t.id === activeTabId);
							if (cur) {
								cur.page = 'web';
								cur.url = url;
								cur.title = title || url;
								cur.query = url;
								currentQuery = url;
							}
						}}
						onBackToHome={() => {
							const cur = tabs.find((t) => t.id === activeTabId);
							if (cur) {
								cur.page = 'search';
								cur.title = 'New Tab';
								cur.query = '';
								cur.url = undefined;
								currentQuery = '';
							}
						}}
					/>
				{:else if activeTab.page === 'settings'}
					<SettingsScreen
						bind:theme
						bind:activeSection={settingsSection}
						onManualAdd={() => (isManualModalOpen = true)}
					/>
				{:else if activeTab.page === 'web' && activeTab.url}
					<WebTabScreen
						url={activeTab.url}
						title={activeTab.title}
						onClose={() => {
							const cur = tabs.find((t) => t.id === activeTabId);
							if (cur) {
								cur.page = 'search';
								cur.title = 'New Tab';
								cur.query = '';
								cur.url = undefined;
								currentQuery = '';
							}
						}}
					/>
				{/if}
			</div>
		{/key}
	</main>

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
	/* 200-350ms Screen Wipe Transition */
	@keyframes wipeFade {
		0% {
			opacity: 0;
			transform: translateX(12px);
			filter: blur(2px);
		}
		100% {
			opacity: 1;
			transform: translateX(0);
			filter: blur(0);
		}
	}

	.screen-wipe {
		animation: wipeFade 280ms cubic-bezier(0.16, 1, 0.3, 1) forwards;
	}
</style>
