<script lang="ts">
	import { onMount } from 'svelte';
	import { Search } from 'lucide-svelte';
	import type { SearchMode, AppStats } from '../types';
	import { api } from '../api';

	let {
		onSearch = (query: string, mode: SearchMode) => {},
		onOpenUrl = (url: string, title: string) => {},
		stats
	}: {
		onSearch?: (query: string, mode: SearchMode) => void;
		onOpenUrl?: (url: string, title: string) => void;
		stats?: AppStats;
	} = $props();

	let query = $state('');
	let activeMode = $state<SearchMode>('hybrid');
	let suggestions = $state<string[]>([]);
	let showSuggestions = $state(false);
	let searchInputEl = $state<HTMLInputElement | null>(null);

	async function handleInput() {
		if (query.trim().length > 1) {
			try {
				suggestions = await api.suggest(query);
				showSuggestions = suggestions.length > 0;
			} catch {
				suggestions = [];
				showSuggestions = false;
			}
		} else {
			suggestions = [];
			showSuggestions = false;
		}
	}

	function submitSearch(q?: string) {
		const finalQuery = (q ?? query).trim();
		if (!finalQuery) return;
		showSuggestions = false;
		onSearch(finalQuery, activeMode);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			submitSearch();
		} else if (e.key === 'Escape') {
			showSuggestions = false;
		}
	}

	onMount(() => {
		function handleGlobalSlash(e: KeyboardEvent) {
			if (
				e.key === '/' &&
				document.activeElement?.tagName !== 'INPUT' &&
				document.activeElement?.tagName !== 'TEXTAREA'
			) {
				e.preventDefault();
				searchInputEl?.focus();
			}
		}
		window.addEventListener('keydown', handleGlobalSlash);
		return () => window.removeEventListener('keydown', handleGlobalSlash);
	});
</script>

<div class="relative flex min-h-[calc(100vh-120px)] flex-col items-center justify-center px-4 py-12 select-none">
	<!-- 1. Judul & Subjudul Ditaruh Di Atas Search Bar -->
	<div class="mb-7 text-center">
		<h1 class="wordmark-title text-4xl sm:text-5xl md:text-6xl font-black tracking-[-0.03em] uppercase">
			QORI
		</h1>
		<p class="mt-2 text-sm sm:text-base text-[var(--text-dim)]">
			Personal search that <span class="tagline-accent text-lg sm:text-xl text-[var(--text)] italic">belongs</span> entirely to you.
		</p>
	</div>

	<!-- 2. Centered Main Search Pill (~840px wide x 68px tall, white/surface, soft shadow) -->
	<div class="relative w-full max-w-[840px]">
		<div
			class="relative flex h-[68px] w-full items-center gap-3 rounded-full bg-[var(--surface)]/80 backdrop-blur-xl px-5 shadow-[var(--shadow-search)] border border-white/10 focus-within:border-[var(--accent)] focus-within:ring-3 focus-within:ring-[var(--accent-soft)] transition-all duration-200"
		>

			<!-- Search Input Field -->
			<input
				bind:this={searchInputEl}
				type="text"
				bind:value={query}
				oninput={handleInput}
				onkeydown={handleKeydown}
				placeholder="Search the web"
				class="flex-1 bg-transparent text-base sm:text-lg font-medium text-[var(--text)] placeholder-[var(--text-faint)] focus:outline-none focus:ring-0 focus-visible:outline-none outline-none border-none ring-0 shadow-none"
			/>

			<!-- Search Submit Action Button -->
			<button
				onclick={() => submitSearch()}
				class="h-10 px-5 rounded-full bg-[var(--accent)] hover:bg-[var(--accent-hover)] text-white flex items-center gap-2 text-xs font-semibold shadow-sm transition-all hover:scale-102"
				title="Submit Search (Enter)"
				aria-label="Search"
			>
				<Search class="h-4 w-4" />
				<span class="hidden sm:inline">Search</span>
			</button>
		</div>

		<!-- Live Tantivy Suggestions Popover -->
		{#if showSuggestions && suggestions.length > 0}
			<div
				class="absolute left-0 right-0 top-[76px] z-50 rounded-[var(--radius-lg)] bg-[var(--surface)] border border-[var(--border)] shadow-[var(--shadow-popover)] overflow-hidden"
			>
				{#each suggestions as sug}
					<button
						onclick={() => {
							query = sug;
							submitSearch(sug);
						}}
						class="flex w-full items-center gap-3 px-5 py-3 text-left text-sm text-[var(--text)] hover:bg-[var(--accent-soft)] transition-colors border-b border-[var(--border)] last:border-b-0"
					>
						<Search class="h-4 w-4 text-[var(--text-dim)]" />
						<span>{sug}</span>
					</button>
				{/each}
			</div>
		{/if}
	</div>
</div>
