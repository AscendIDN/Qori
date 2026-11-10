<script lang="ts">
	import { onMount } from 'svelte';
	import {
		ExternalLink,
		Bookmark,
		Clock,
		Globe
	} from 'lucide-svelte';
	import type { DocumentItem, SearchFilter, SearchMode } from '../types';
	import { api } from '../api';

	let {
		results = [],
		query = '',
		mode = 'keyword',
		modeUsed = 'keyword',
		fallbackMessage = null,
		durationMs = 0,
		totalCount = 0,
		onSearchAgain = (q: string, m: SearchMode, f: SearchFilter) => {},
		onOpenWebTab = (url: string, title: string) => {},
		onBackToHome = () => {}
	}: {
		results: DocumentItem[];
		query: string;
		mode: SearchMode;
		modeUsed?: string;
		fallbackMessage?: string | null;
		durationMs: number;
		totalCount: number;
		onSearchAgain?: (q: string, m: SearchMode, f: SearchFilter) => void;
		onOpenWebTab?: (url: string, title: string) => void;
		onBackToHome?: () => void;
	} = $props();

	let selectedIndex = $state(0);
	let searchQuery = $state('');
	let searchInputEl = $state<HTMLInputElement | null>(null);
	let copiedId = $state<number | null>(null);

	// Sync searchQuery when query prop changes
	$effect(() => {
		searchQuery = query;
	});

	const filteredResults = $derived(results);

	async function togglePin(doc: DocumentItem) {
		const nextPinned = !doc.pinned;
		doc.pinned = nextPinned;
		await api.pinDocument(doc.id, nextPinned);
	}

	function openDoc(doc: DocumentItem) {
		onOpenWebTab(doc.url, doc.title || doc.url);
	}

	function openExternal(doc: DocumentItem) {
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
	<!-- Main Body: Editorial Results -->
	<div class="mx-auto max-w-4xl px-4 py-8 md:px-8">
		<!-- Main Editorial Result List -->
		<main class="w-full">
			<!-- Results Meta Bar (Google-like clean stats) -->
			<div class="mb-6 flex flex-wrap items-center justify-between gap-3 border-b border-[var(--line)] pb-3 font-mono text-xs select-none">
				<div class="flex items-center gap-2 text-xs tracking-wider text-[var(--fg-dim)]">
					<span>Hasil pencarian untuk <span class="font-bold text-[var(--fg)]">"{query}"</span></span>
				</div>

				<div class="flex items-center gap-2 text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">
					<span>{filteredResults.length} HASIL WEB</span>
					<span>•</span>
					<span>{durationMs} MS</span>
				</div>
			</div>

			{#if filteredResults.length === 0}
				<div class="border border-dashed border-[var(--line)] p-12 text-center font-mono">
					<p class="mb-2 text-sm tracking-widest text-[var(--fg-dim)] uppercase">
						TIDAK ADA HASIL UNTUK PENCARIAN INI
					</p>
					<p class="mb-6 text-xs text-[var(--fg-faint)]">
						Pastikan koneksi internet aktif atau coba gunakan kata kunci yang lebih umum.
					</p>
					<button
						onclick={onBackToHome}
						class="border border-[var(--line-strong)] bg-[var(--surface)] px-4 py-2 font-mono text-xs tracking-wider text-[var(--fg)] uppercase hover:border-[var(--fg)]"
					>
						KEMBALI KE BERANDA
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
										onclick={() => openDoc(doc)}
										class="cursor-pointer truncate text-[11px] text-[var(--accent-blue)] underline-offset-2 hover:underline"
									>
										{doc.url}
									</span>
									<button
										onclick={(e) => {
											e.stopPropagation();
											openExternal(doc);
										}}
										class="text-[var(--fg-faint)] transition-colors hover:text-[var(--fg)]"
										title="Open in System Browser"
									>
										<ExternalLink class="h-3 w-3" />
									</button>
								</div>

								<div class="flex shrink-0 items-center gap-2">
									<span
										class="border border-[var(--line)] bg-[var(--bg-deep)] px-2 py-0.5 text-[9px] tracking-wider text-[var(--fg-dim)] uppercase"
									>
										{doc.source_label || doc.source_kind || 'WEB'}
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

							<!-- Bottom Row: Metadata & Action Buttons -->
							<div
								class="flex flex-wrap items-center justify-between gap-4 border-t border-[var(--line)] pt-3 font-mono text-[10px] text-[var(--fg-faint)]"
							>
								<div class="flex flex-wrap items-center gap-3">
									<span class="flex items-center gap-1.5 font-semibold uppercase text-[var(--accent-green)]">
										<Globe class="h-3 w-3" />
										<span>{doc.source_label || 'WEB'}</span>
									</span>
									<span>•</span>
									<span class="flex items-center gap-1">
										<Clock class="h-3 w-3" />
										<span>{doc.fetched_at?.slice(0, 10) || 'TERBARU'}</span>
									</span>
								</div>

								<!-- Action Buttons -->
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
											openExternal(doc);
										}}
										class="flex items-center gap-1 border border-[var(--line)] bg-[var(--surface)] px-2.5 py-1 text-[10px] tracking-wider text-[var(--fg-dim)] uppercase transition-all hover:border-[var(--fg)] hover:text-[var(--fg)]"
										title="Buka di browser eksternal"
									>
										<span>BROWSER EKSTERNAL</span>
										<ExternalLink class="h-3 w-3" />
									</button>

									<button
										onclick={(e) => {
											e.stopPropagation();
											openDoc(doc);
										}}
										class="flex items-center gap-1 border border-[var(--line-strong)] bg-[var(--fg)] px-3 py-1 text-[10px] font-bold tracking-wider text-[var(--bg)] uppercase transition-all hover:bg-[var(--fg)]/90 active:scale-[0.98]"
									>
										<span>BUKA DI TAB</span>
										<Globe class="h-3 w-3 stroke-[2.5]" />
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
