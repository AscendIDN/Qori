<script lang="ts">
	import { onMount } from 'svelte';
	import Navbar from '../lib/components/Navbar.svelte';
	import ScrollRuler from '../lib/components/ScrollRuler.svelte';
	import SectionIndex from '../lib/components/SectionIndex.svelte';
	import Ticker from '../lib/components/Ticker.svelte';
	import OrbitArcs from '../lib/components/OrbitArcs.svelte';
	import CommandPalette from '../lib/components/CommandPalette.svelte';
	import ManualDocModal from '../lib/components/ManualDocModal.svelte';
	import HomeScreen from '../lib/components/HomeScreen.svelte';
	import ResultsScreen from '../lib/components/ResultsScreen.svelte';
	import LibraryScreen from '../lib/components/LibraryScreen.svelte';
	import SourcesScreen from '../lib/components/SourcesScreen.svelte';
	import SettingsScreen from '../lib/components/SettingsScreen.svelte';
	import type {
		SearchMode,
		SearchFilter,
		SearchResponse,
		AppStats,
		DocumentItem
	} from '../lib/types';
	import { api } from '../lib/api';

	let activeTab = $state('search');
	let theme = $state<'dark' | 'light'>('dark');
	let isCommandPaletteOpen = $state(false);
	let isManualModalOpen = $state(false);

	// Search State
	let currentQuery = $state('');
	let currentMode = $state<SearchMode>('keyword');
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
		try {
			const resp = await api.search(query, mode, filter);
			searchResults = resp.documents;
			searchDurationMs = resp.duration_ms;
			searchTotal = resp.total;
		} catch (err) {
			console.error('Search failed:', err);
		}
		activeTab = 'results';
		window.scrollTo({ top: 0, behavior: 'smooth' });
	}

	onMount(() => {
		loadInitialStats();
	});
</script>

<div
	class="relative min-h-screen bg-[var(--bg)] text-[var(--fg)] transition-colors duration-200 selection:bg-[var(--accent-blue)] selection:text-white"
>
	<!-- Background Orbit Arcs and Floating Cards (Hero decoration) -->
	<OrbitArcs />

	<!-- Fixed Scroll Ruler (Right Edge) -->
	<ScrollRuler />

	<!-- Section Index (Bottom-Right) -->
	<SectionIndex bind:activeTab />

	<!-- Top Navbar -->
	<Navbar
		bind:activeTab
		bind:theme
		onOpenCommandPalette={() => (isCommandPaletteOpen = true)}
		onQuickAdd={() => (activeTab = 'sources')}
	/>

	<!-- Main Content Container with Wipe Transition -->
	<main class="relative z-10 w-full overflow-hidden">
		{#key activeTab}
			<div class="screen-wipe min-h-[calc(100vh-80px)]">
				{#if activeTab === 'search'}
					<HomeScreen onSearch={handleSearch} {stats} />
				{:else if activeTab === 'results'}
					<ResultsScreen
						results={searchResults}
						query={currentQuery}
						mode={currentMode}
						durationMs={searchDurationMs}
						totalCount={searchTotal}
						onSearchAgain={handleSearch}
						onBackToHome={() => (activeTab = 'search')}
					/>
				{:else if activeTab === 'library'}
					<LibraryScreen
						onNavigateSources={() => (activeTab = 'sources')}
						onManualAdd={() => (isManualModalOpen = true)}
					/>
				{:else if activeTab === 'sources'}
					<SourcesScreen />
				{:else if activeTab === 'settings'}
					<SettingsScreen bind:theme />
				{/if}
			</div>
		{/key}
	</main>

	<!-- Horizontal Marquee Ticker at the Bottom -->
	<footer class="relative z-20 w-full">
		<Ticker {stats} />
	</footer>

	<!-- Command Palette Modal (Ctrl+K) -->
	<CommandPalette
		bind:isOpen={isCommandPaletteOpen}
		onSelectTab={(tab) => (activeTab = tab)}
		onQuickAdd={() => (activeTab = 'sources')}
		onExport={async () => {
			const fn = await api.exportLibrary();
			alert(`Exported: ${fn}`);
		}}
		onToggleTheme={() => {
			theme = theme === 'dark' ? 'light' : 'dark';
			document.documentElement.setAttribute('data-theme', theme);
		}}
	/>

	<!-- Manual Document Entry Modal -->
	<ManualDocModal
		bind:isOpen={isManualModalOpen}
		onAdded={() => {
			loadInitialStats();
			activeTab = 'library';
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
