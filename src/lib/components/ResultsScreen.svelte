<script lang="ts">
	import { onMount } from 'svelte';
	import {
		Search,
		ExternalLink,
		Bookmark,
		Tag,
		Clock,
		Filter,
		FileText,
		Globe,
		Folder,
		Check,
		Share2,
		SlidersHorizontal,
		Sparkles
	} from 'lucide-svelte';
	import type { DocumentItem, SearchFilter, SearchMode } from '../types';
	import { api } from '../api';

	let {
		results = [],
		query = '',
		mode = 'keyword',
		durationMs = 0,
		totalCount = 0,
		onSearchAgain = (q: string, m: SearchMode, f: SearchFilter) => {},
		onBackToHome = () => {}
	}: {
		results: DocumentItem[];
		query: string;
		mode: SearchMode;
		durationMs: number;
		totalCount: number;
		onSearchAgain?: (q: string, m: SearchMode, f: SearchFilter) => void;
		onBackToHome?: () => void;
	} = $props();

	let selectedIndex = $state(0);
	let filterKind = $state('all');
	let filterDate = $state('all');
	let filterTag = $state('all');
	let searchQuery = $state('');
	let searchInputEl = $state<HTMLInputElement | null>(null);
	let copiedId = $state<number | null>(null);

	// Sync searchQuery when query prop changes
	$effect(() => {
		searchQuery = query;
	});

	const allTags = $derived(Array.from(new Set(results.flatMap((r) => r.tags))).sort());

	const filteredResults = $derived(
		results.filter((doc) => {
			if (filterKind !== 'all' && doc.source_kind !== filterKind) return false;
			if (filterTag !== 'all' && !doc.tags.includes(filterTag)) return false;
			return true;
		})
	);

	async function togglePin(doc: DocumentItem) {
		const nextPinned = !doc.pinned;
		doc.pinned = nextPinned;
		await api.pinDocument(doc.id, nextPinned);
	}

	function openDoc(doc: DocumentItem) {
		api.openDocument(doc.url);
	}

	function copyUrl(doc: DocumentItem) {
		navigator.clipboard.writeText(doc.url);
		copiedId = doc.id;
		setTimeout(() => {
			if (copiedId === doc.id) copiedId = null;
		}, 1800);
	}

	function handleKeydown(e: KeyboardEvent) {
		// If active element is input, don't hijack j/k
		if (
			document.activeElement?.tagName === 'INPUT' ||
			document.activeElement?.tagName === 'TEXTAREA'
		) {
			return;
		}

		if (e.key === 'j') {
			e.preventDefault();
			selectedIndex = Math.min(filteredResults.length - 1, selectedIndex + 1);
			scrollSelectedIntoView();
		} else if (e.key === 'k') {
			e.preventDefault();
			selectedIndex = Math.max(0, selectedIndex - 1);
			scrollSelectedIntoView();
		} else if (e.key === 'Enter') {
			e.preventDefault();
			if (filteredResults[selectedIndex]) {
				openDoc(filteredResults[selectedIndex]);
			}
		} else if (e.key === '/') {
			e.preventDefault();
			searchInputEl?.focus();
		}
	}

	function scrollSelectedIntoView() {
		setTimeout(() => {
			const el = document.getElementById(`result-item-${selectedIndex}`);
			el?.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
		}, 20);
	}

	onMount(() => {
		window.addEventListener('keydown', handleKeydown);
		return () => window.removeEventListener('keydown', handleKeydown);
	});
</script>

<div class="w-full pb-20">
	<!-- Query Bar and Meta Header -->
	<div class="border-b border-[var(--line)] bg-[var(--surface)] px-4 py-4 md:px-8">
		<div class="mx-auto flex max-w-7xl flex-col justify-between gap-4 md:flex-row md:items-center">
			<!-- Search Input in Header -->
			<div
				class="relative flex max-w-2xl flex-1 items-center border border-[var(--line-strong)] bg-[var(--bg)] transition-colors focus-within:border-[var(--fg)]"
			>
				<Search class="ml-3 h-4 w-4 text-[var(--fg-dim)]" />
				<input
					bind:this={searchInputEl}
					bind:value={searchQuery}
					onkeydown={(e) => {
						if (e.key === 'Enter') {
							onSearchAgain(searchQuery, mode, { kind: filterKind, tag: filterTag });
						}
					}}
					type="text"
					placeholder="Refine search query..."
					class="w-full bg-transparent px-3 py-2 font-mono text-sm text-[var(--fg)] placeholder:text-[var(--fg-faint)] focus:outline-none"
				/>
				<button
					onclick={() => onSearchAgain(searchQuery, mode, { kind: filterKind, tag: filterTag })}
					class="bg-[var(--fg)] px-3 py-1.5 font-mono text-xs font-bold tracking-wider text-[var(--bg)] uppercase hover:bg-[var(--fg)]/90"
				>
					QUERY
				</button>
			</div>

			<!-- Result Timing & Mode Stats -->
			<div class="flex items-center gap-3 font-mono text-xs text-[var(--fg-dim)]">
				<span
					class="border border-[var(--line)] bg-[var(--bg-deep)] px-2 py-1 text-[10px] font-bold text-[var(--accent-blue)] uppercase"
				>
					MODE: {mode}
				</span>
				<span class="font-bold text-[var(--fg)]">{filteredResults.length}</span>
				<span>RESULTS IN</span>
				<span class="font-bold text-[var(--accent-green)]">{durationMs} MS</span>
			</div>
		</div>
	</div>

	<!-- Main Body: Left Filter Rail + Editorial Results -->
	<div class="mx-auto flex max-w-7xl flex-col gap-8 px-4 py-8 md:px-8 lg:flex-row">
		<!-- Left Filter Rail -->
		<aside class="flex w-full shrink-0 flex-col gap-6 font-mono text-xs select-none lg:w-64">
			<div
				class="flex items-center gap-2 border-b border-[var(--line)] pb-2 tracking-widest text-[var(--fg-dim)] uppercase"
			>
				<Filter class="h-3.5 w-3.5 text-[var(--accent-blue)]" />
				<span>FILTERS // REFINE</span>
			</div>

			<!-- Source Filter -->
			<div class="flex flex-col gap-1.5">
				<div class="text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">SOURCE TYPE</div>
				{#each [{ id: 'all', label: 'All Sources' }, { id: 'web', label: 'Web Pages' }, { id: 'folder', label: 'Local Files' }, { id: 'bookmark', label: 'Bookmarks' }] as opt}
					<button
						onclick={() => (filterKind = opt.id)}
						class={`flex items-center justify-between border px-2.5 py-1.5 text-left transition-all ${
							filterKind === opt.id
								? 'border-[var(--fg)] bg-[var(--fg)] font-bold text-[var(--bg)]'
								: 'border-transparent text-[var(--fg-dim)] hover:border-[var(--line)] hover:bg-[var(--surface)]'
						}`}
					>
						<span>{opt.label}</span>
						{#if filterKind === opt.id}
							<Check class="h-3 w-3" />
						{/if}
					</button>
				{/each}
			</div>

			<!-- Date Filter -->
			<div class="flex flex-col gap-1.5">
				<div class="text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">DATE RANGE</div>
				{#each [{ id: 'all', label: 'Anytime' }, { id: '24h', label: 'Past 24 Hours' }, { id: 'week', label: 'Past Week' }, { id: 'month', label: 'Past Month' }] as opt}
					<button
						onclick={() => (filterDate = opt.id)}
						class={`flex items-center justify-between border px-2.5 py-1.5 text-left transition-all ${
							filterDate === opt.id
								? 'border-[var(--fg)] bg-[var(--fg)] font-bold text-[var(--bg)]'
								: 'border-transparent text-[var(--fg-dim)] hover:border-[var(--line)] hover:bg-[var(--surface)]'
						}`}
					>
						<span>{opt.label}</span>
						{#if filterDate === opt.id}
							<Check class="h-3 w-3" />
						{/if}
					</button>
				{/each}
			</div>

			<!-- Tags Filter -->
			{#if allTags.length > 0}
				<div class="flex flex-col gap-1.5">
					<div class="text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">TAGS</div>
					<button
						onclick={() => (filterTag = 'all')}
						class={`flex items-center justify-between border px-2.5 py-1.5 text-left transition-all ${
							filterTag === 'all'
								? 'border-[var(--fg)] bg-[var(--fg)] font-bold text-[var(--bg)]'
								: 'border-transparent text-[var(--fg-dim)] hover:border-[var(--line)] hover:bg-[var(--surface)]'
						}`}
					>
						<span>All Tags</span>
					</button>
					{#each allTags as tag}
						<button
							onclick={() => (filterTag = tag)}
							class={`flex items-center justify-between border px-2.5 py-1.5 text-left transition-all ${
								filterTag === tag
									? 'border-[var(--fg)] bg-[var(--fg)] font-bold text-[var(--bg)]'
									: 'border-transparent text-[var(--fg-dim)] hover:border-[var(--line)] hover:bg-[var(--surface)]'
							}`}
						>
							<span>#{tag}</span>
							{#if filterTag === tag}
								<Check class="h-3 w-3" />
							{/if}
						</button>
					{/each}
				</div>
			{/if}

			<!-- Keyboard Shortcuts Help -->
			<div
				class="mt-4 flex flex-col gap-1 border border-[var(--line)] bg-[var(--surface)] p-3 text-[10px] text-[var(--fg-faint)]"
			>
				<div class="mb-1 font-bold tracking-wider text-[var(--fg-dim)] uppercase">
					KEYBOARD SHORTCUTS
				</div>
				<div class="flex justify-between">
					<span>Next Result</span><kbd class="text-[var(--fg)]">J</kbd>
				</div>
				<div class="flex justify-between">
					<span>Prev Result</span><kbd class="text-[var(--fg)]">K</kbd>
				</div>
				<div class="flex justify-between">
					<span>Open Item</span><kbd class="text-[var(--fg)]">ENTER</kbd>
				</div>
				<div class="flex justify-between">
					<span>Focus Query</span><kbd class="text-[var(--fg)]">/</kbd>
				</div>
			</div>
		</aside>

		<!-- Main Editorial Result List -->
		<main class="min-w-0 flex-1">
			{#if filteredResults.length === 0}
				<div class="border border-dashed border-[var(--line)] p-12 text-center font-mono">
					<p class="mb-2 text-sm tracking-widest text-[var(--fg-dim)] uppercase">
						NO DOCUMENTS MATCHED YOUR QUERY
					</p>
					<p class="mb-6 text-xs text-[var(--fg-faint)]">
						Try broadening keywords, removing filters, or crawling new web sources.
					</p>
					<button
						onclick={onBackToHome}
						class="border border-[var(--line-strong)] bg-[var(--surface)] px-4 py-2 font-mono text-xs tracking-wider text-[var(--fg)] uppercase hover:border-[var(--fg)]"
					>
						RETURN TO SEARCH CONSOLE
					</button>
				</div>
			{:else}
				<div class="flex flex-col gap-4">
					{#each filteredResults as doc, i}
						{@const isSelected = i === selectedIndex}
						<article
							id={`result-item-${i}`}
							onclick={() => (selectedIndex = i)}
							class={`group relative border p-5 transition-all ${
								isSelected
									? 'translate-x-1 border-[var(--fg)] bg-[var(--surface)] shadow-lg'
									: 'border-[var(--line)] bg-[var(--surface)]/60 hover:border-[var(--line-strong)] hover:bg-[var(--surface)]'
							}`}
							style={`animation: resultEnter 0.35s cubic-bezier(0.16, 1, 0.3, 1) ${i * 30}ms backwards;`}
						>
							<!-- Top Row: Index number, Mono URL, Source Badge, Pin Action -->
							<div class="mb-2 flex items-center justify-between gap-3 font-mono text-xs">
								<div class="flex min-w-0 items-center gap-2">
									<span
										class={`font-bold ${isSelected ? 'text-[var(--accent-blue)]' : 'text-[var(--fg-faint)]'}`}
									>
										{(i + 1).toString().padStart(2, '0')}
									</span>
									<span class="text-[var(--line-strong)]">/</span>
									<span
										class="truncate text-[11px] text-[var(--accent-blue)] underline-offset-2 hover:underline"
									>
										{doc.url}
									</span>
								</div>

								<div class="flex shrink-0 items-center gap-2">
									<span
										class="border border-[var(--line)] bg-[var(--bg-deep)] px-2 py-0.5 text-[9px] tracking-wider text-[var(--fg-dim)] uppercase"
									>
										{doc.source_label || doc.source_kind || 'LOCAL'}
									</span>
									<button
										onclick={(e) => {
											e.stopPropagation();
											togglePin(doc);
										}}
										class={`p-1 transition-colors ${
											doc.pinned
												? 'text-[var(--accent-blue)]'
												: 'text-[var(--fg-faint)] hover:text-[var(--fg)]'
										}`}
										title={doc.pinned ? 'Pinned (Click to unpin)' : 'Pin document'}
									>
										<Bookmark class="h-3.5 w-3.5 fill-current" />
									</button>
								</div>
							</div>

							<!-- Large Title -->
							<h2
								class="mb-2 cursor-pointer text-lg font-bold tracking-tight text-[var(--fg)] transition-colors hover:text-[var(--accent-blue)] sm:text-xl"
								onclick={() => openDoc(doc)}
							>
								{doc.title || doc.url}
							</h2>

							<!-- Highlighted Snippet -->
							{#if doc.snippet}
								<div
									class="snippet-content mb-4 text-xs leading-relaxed text-[var(--fg-dim)] sm:text-sm"
								>
									{@html doc.snippet}
								</div>
							{:else if doc.description}
								<p class="mb-4 text-xs leading-relaxed text-[var(--fg-dim)] sm:text-sm">
									{doc.description}
								</p>
							{/if}

							<!-- Bottom Row: Metadata, Tags, Action Buttons -->
							<div
								class="flex flex-wrap items-center justify-between gap-4 border-t border-[var(--line)] pt-3 font-mono text-[10px] text-[var(--fg-faint)]"
							>
								<div class="flex flex-wrap items-center gap-3">
									<span class="flex items-center gap-1">
										<Clock class="h-3 w-3" />
										<span>{doc.fetched_at?.slice(0, 10) || 'RECENT'}</span>
									</span>
									<span>•</span>
									<span>{doc.word_count?.toLocaleString() || 0} WORDS</span>
									{#if doc.score}
										<span>•</span>
										<span class="text-[var(--accent-purple)]">SCORE: {doc.score.toFixed(2)}</span>
									{/if}

									<!-- Tags -->
									{#if doc.tags && doc.tags.length > 0}
										<div class="ml-2 flex items-center gap-1">
											{#each doc.tags as t}
												<span
													class="border border-[var(--line)] bg-[var(--bg)] px-1.5 py-0.5 text-[9px] text-[var(--fg-dim)]"
												>
													#{t}
												</span>
											{/each}
										</div>
									{/if}
								</div>

								<!-- Action Buttons with Mono Invert on Hover -->
								<div class="flex items-center gap-2">
									<button
										onclick={(e) => {
											e.stopPropagation();
											copyUrl(doc);
										}}
										class="border border-[var(--line)] bg-transparent px-2.5 py-1 text-[10px] tracking-wider text-[var(--fg-dim)] uppercase transition-all hover:border-[var(--fg)] hover:bg-[var(--fg)] hover:font-bold hover:text-[var(--bg)]"
									>
										{copiedId === doc.id ? 'COPIED!' : 'COPY URL'}
									</button>

									<button
										onclick={(e) => {
											e.stopPropagation();
											openDoc(doc);
										}}
										class="flex items-center gap-1 border border-[var(--line-strong)] bg-[var(--fg)] px-3 py-1 text-[10px] font-bold tracking-wider text-[var(--bg)] uppercase transition-all hover:bg-[var(--fg)]/90 active:scale-[0.98]"
									>
										<span>OPEN</span>
										<ExternalLink class="h-3 w-3 stroke-[2.5]" />
									</button>
								</div>
							</div>
						</article>
					{/each}
				</div>
			{/if}
		</main>
	</div>
</div>

<style>
	@keyframes resultEnter {
		0% {
			opacity: 0;
			transform: translateY(12px);
		}
		100% {
			opacity: 1;
			transform: translateY(0);
		}
	}

	:global(.snippet-content mark) {
		background-color: rgba(47, 107, 255, 0.28);
		color: #ffffff;
		padding: 0.1rem 0.25rem;
		font-weight: 600;
	}
	:global([data-theme='light'] .snippet-content mark) {
		background-color: rgba(28, 82, 216, 0.22);
		color: #101217;
	}
</style>
