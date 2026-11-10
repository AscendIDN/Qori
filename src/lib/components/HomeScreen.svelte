<script lang="ts">
	import { onMount } from 'svelte';
	import {
		Search,
		Sparkles,
		SlidersHorizontal,
		ArrowRight,
		CornerDownLeft,
		Zap
	} from 'lucide-svelte';
	import type { SearchMode, AppStats } from '../types';
	import { api } from '../api';

	let {
		onSearch = (query: string, mode: SearchMode) => {},
		stats
	}: {
		onSearch?: (query: string, mode: SearchMode) => void;
		stats: AppStats;
	} = $props();

	let query = $state('');
	let activeMode = $state<SearchMode>('keyword');
	let suggestions = $state<string[]>([]);
	let showSuggestions = $state(false);
	let searchInputEl = $state<HTMLInputElement | null>(null);

	async function handleInput() {
		if (query.trim().length > 1) {
			suggestions = await api.suggest(query);
			showSuggestions = suggestions.length > 0;
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
		// Focus search on '/' key press if not already inside an input
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

<div
	class="relative flex min-h-[calc(100vh-130px)] flex-col items-center justify-center px-4 py-12"
>
	<!-- Giant Uppercase Extended-Bold Wordmark with Staggered Reveal Letters -->
	<div class="mb-4 text-center select-none">
		<h1
			class="wordmark-gradient text-4xl font-[var(--font-display)] font-black tracking-[-0.04em] uppercase sm:text-6xl md:text-7xl lg:text-8xl"
		>
			{#each 'QORI'.split('') as char, i}
				<span
					class="inline-block transition-transform duration-300 hover:-translate-y-1"
					style={`animation: letterFade 0.6s cubic-bezier(0.16, 1, 0.3, 1) ${i * 35}ms backwards;`}
				>
					{char === ' ' ? '\u00A0' : char}
				</span>
			{/each}
		</h1>
	</div>

	<!-- Headline mixing Extended Bold Sans-serif with ONE Italic Serif accent word -->
	<p class="mb-10 max-w-xl text-center font-mono text-sm text-[var(--fg-dim)] sm:text-base">
		Personal search that <span
			class="text-xl font-[var(--font-serif)] font-normal text-[var(--fg)] italic sm:text-2xl"
			>belongs</span
		> entirely to you.
	</p>

	<!-- Brutalist Search Box Container -->
	<div class="relative w-full max-w-3xl">
		<!-- Main Brutalist Input Bar -->
		<div
			class="relative flex items-center border border-[var(--line-strong)] bg-[var(--surface)] shadow-2xl transition-colors"
		>
			<div class="pl-4 text-[var(--fg-dim)]">
				<Search class="h-5 w-5" />
			</div>
			<input
				bind:this={searchInputEl}
				bind:value={query}
				oninput={handleInput}
				onkeydown={handleKeydown}
				onfocus={() => {
					if (suggestions.length > 0) showSuggestions = true;
				}}
				type="text"
				placeholder="Search Anything..."
				class="flex-1 bg-transparent px-4 py-4 font-mono text-sm text-[var(--fg)] placeholder:text-[var(--fg-faint)] focus:outline-none focus:ring-0 sm:text-base"
			/>
			<div class="flex items-center gap-2 pr-3">
				<button
					onclick={() => submitSearch()}
					class="flex items-center gap-1.5 bg-[var(--fg)] px-4 py-2 font-mono text-xs font-bold tracking-wider text-[var(--bg)] uppercase transition-all hover:bg-[var(--fg)]/90 active:scale-[0.98]"
				>
					<span>SEARCH</span>
					<CornerDownLeft class="h-3.5 w-3.5 stroke-[2.5]" />
				</button>
			</div>
		</div>

		<!-- Autocomplete Suggestions Dropdown (< 50ms) -->
		{#if showSuggestions && suggestions.length > 0}
			<div
				class="absolute top-full right-0 left-0 z-30 mt-1 border border-[var(--line-strong)] bg-[var(--surface)] shadow-2xl"
			>
				<div
					class="border-b border-[var(--line)] px-3 py-1.5 font-mono text-[9px] tracking-wider text-[var(--fg-faint)] uppercase"
				>
					SUGGESTIONS // HISTORY & TITLES
				</div>
				{#each suggestions as sug}
					<button
						onclick={() => {
							query = sug;
							submitSearch(sug);
						}}
						class="flex w-full items-center justify-between border-b border-[var(--line)] px-4 py-2 text-left font-mono text-xs text-[var(--fg)] transition-colors last:border-b-0 hover:bg-[var(--surface-hover)] hover:text-[var(--accent-blue)]"
					>
						<div class="flex items-center gap-2">
							<Zap class="h-3 w-3 text-[var(--accent-blue)]" />
							<span>{sug}</span>
						</div>
						<span class="text-[9px] text-[var(--fg-faint)] uppercase">[↵]</span>
					</button>
				{/each}
			</div>
		{/if}
	</div>
</div>

<style>
	@keyframes letterFade {
		0% {
			opacity: 0;
			transform: translateY(16px);
		}
		100% {
			opacity: 1;
			transform: translateY(0);
		}
	}
</style>
