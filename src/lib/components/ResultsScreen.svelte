<script lang="ts">
	import { onMount } from 'svelte';
	import {
		ExternalLink,
		Bookmark,
		Clock,
		Globe,
		ChevronRight,
		Image,
		Film,
		Newspaper,
		Filter,
		Share2,
		Check,
		Search,
		Sparkles
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
	let copiedId = $state<number | null>(null);
	let activeCategory = $state<'Semua' | 'Gambar' | 'Video' | 'Berita'>('Semua');

	function switchCategory(cat: 'Semua' | 'Gambar' | 'Video' | 'Berita') {
		if (activeCategory === cat) return;
		activeCategory = cat;
		const categoryMap: Record<'Semua' | 'Gambar' | 'Video' | 'Berita', 'general' | 'images' | 'videos' | 'news'> = {
			Semua: 'general',
			Gambar: 'images',
			Video: 'videos',
			Berita: 'news'
		};
		onSearchAgain(query, mode, { category: categoryMap[cat] });
	}

	const filteredResults = $derived.by(() => {
		return results;
	});

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

<div class="w-full pb-20 select-none">
	<div class="mr-auto w-full max-w-4xl px-4 sm:px-8 py-6">
		<main class="w-full">
			<!-- Category Filter Pills Bar (Semua, Gambar, Video, Berita) -->
			<div class="mb-5 flex items-center gap-2 overflow-x-auto pb-1 text-xs select-none">
				{#each [
					{ id: 'Semua', label: 'All Results', icon: Globe },
					{ id: 'Gambar', label: 'Images', icon: Image },
					{ id: 'Video', label: 'Videos', icon: Film },
					{ id: 'Berita', label: 'News', icon: Newspaper }
				] as cat}
					{@const isActive = activeCategory === cat.id}
					<button
						onclick={() => switchCategory(cat.id as any)}
						class={`flex items-center gap-2 px-3.5 py-1.5 rounded-full text-xs font-semibold tracking-wide transition-all ${
							isActive
								? 'bg-[var(--accent)] text-white shadow-xs font-bold'
								: 'bg-[var(--surface)] text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)] border border-[var(--border)]'
						}`}
					>
						<cat.icon class="h-3.5 w-3.5" />
						<span>{cat.label}</span>
					</button>
				{/each}
			</div>

			<!-- Results Meta Bar -->
			<div class="mb-6 flex flex-wrap items-center justify-between gap-3 border-b border-[var(--border)] pb-3 text-xs">
				<div class="flex items-center gap-2 text-xs text-[var(--text-dim)]">
					<span>Search results for <span class="font-bold text-[var(--text)]">"{query}"</span></span>
					{#if modeUsed && modeUsed !== mode}
						<span class="rounded-full bg-[var(--accent-soft)] px-2 py-0.5 text-[10px] text-[var(--accent)] font-medium">
							Fallback: {modeUsed}
						</span>
					{/if}
				</div>

				<div class="flex items-center gap-2 text-[11px] text-[var(--text-faint)]">
					<span>{filteredResults.length} items found</span>
					<span>•</span>
					<span>{durationMs} ms</span>
				</div>
			</div>

			<!-- Fallback Warning Notification if SearXNG / Ollama was offline -->
			{#if fallbackMessage}
				<div class="mb-6 rounded-[var(--radius-md)] bg-amber-500/10 border border-amber-500/20 p-3.5 text-xs text-amber-300">
					{fallbackMessage}
				</div>
			{/if}

			{#if filteredResults.length === 0}
				<!-- Empty Results State -->
				<div class="rounded-[var(--radius-lg)] bg-[var(--surface)] border border-[var(--border)] p-12 text-center shadow-sm">
					<p class="mb-2 text-sm font-bold text-[var(--text)]">
						NO RESULTS FOUND FOR THIS QUERY
					</p>
					<p class="mb-6 text-xs text-[var(--text-dim)]">
						Try adjusting your search terms, verifying local sources in Crawler, or check your internet connection.
					</p>
					<button
						onclick={onBackToHome}
						class="rounded-full bg-[var(--accent)] px-5 py-2 text-xs font-semibold text-white shadow-sm hover:bg-[var(--accent-hover)] transition-all"
					>
						Return to Start Page
					</button>
				</div>
			{:else if activeCategory === 'Gambar'}
				<!-- IMAGE GRID VIEW -->
				<div class="grid grid-cols-2 gap-3.5 sm:grid-cols-3">
					{#each filteredResults as doc, i}
						{@const isSelected = i === selectedIndex}
						{@const imageSrc = doc.img_src || doc.thumbnail}
						<div
							id={`result-item-${i}`}
							onclick={() => {
								selectedIndex = i;
								openDoc(doc);
							}}
							role="button"
							tabindex="0"
							onkeydown={(e) => {
								if (e.key === 'Enter') openDoc(doc);
							}}
							class={`group relative cursor-pointer overflow-hidden rounded-[var(--radius-md)] border transition-all ${
								isSelected
									? 'border-[var(--accent)] bg-[var(--surface)] shadow-[var(--shadow-soft)] ring-2 ring-[var(--accent-soft)]'
									: 'border-[var(--border)] bg-[var(--surface)] hover:border-[var(--accent-soft)] hover:shadow-sm'
							}`}
						>
							<div class="relative aspect-video w-full overflow-hidden bg-[var(--surface-hover)]">
								{#if imageSrc}
									<img
										src={imageSrc}
										alt={doc.title}
										loading="lazy"
										class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
									/>
								{:else}
									<div class="flex h-full w-full items-center justify-center text-[var(--text-faint)]">
										<Image class="h-8 w-8" />
									</div>
								{/if}
							</div>
							<div class="p-3">
								<div class="truncate font-mono text-[10px] text-[var(--accent)]">
									{doc.source_label || doc.url}
								</div>
								<h3 class="truncate text-xs font-bold text-[var(--text)] group-hover:text-[var(--accent)] mt-0.5">
									{doc.title || doc.url}
								</h3>
							</div>
						</div>
					{/each}
				</div>
			{:else if activeCategory === 'Video'}
				<!-- VIDEO CARDS VIEW -->
				<div class="flex flex-col gap-3.5">
					{#each filteredResults as doc, i}
						{@const isSelected = i === selectedIndex}
						{@const videoThumb = doc.thumbnail || doc.img_src}
						<div
							id={`result-item-${i}`}
							onclick={() => {
								selectedIndex = i;
								openDoc(doc);
							}}
							role="button"
							tabindex="0"
							onkeydown={(e) => {
								if (e.key === 'Enter') openDoc(doc);
							}}
							class={`group relative flex cursor-pointer flex-col gap-4 rounded-[var(--radius-md)] border p-4 transition-all sm:flex-row ${
								isSelected
									? 'border-[var(--accent)] bg-[var(--surface)] shadow-[var(--shadow-soft)] ring-2 ring-[var(--accent-soft)]'
									: 'border-[var(--border)] bg-[var(--surface)] hover:border-[var(--accent-soft)] hover:shadow-sm'
							}`}
						>
							<div class="relative aspect-video w-full shrink-0 overflow-hidden rounded-[var(--radius-sm)] bg-[var(--surface-hover)] sm:w-48">
								{#if videoThumb}
									<img
										src={videoThumb}
										alt={doc.title}
										loading="lazy"
										class="h-full w-full object-cover"
									/>
								{:else}
									<div class="flex h-full w-full items-center justify-center text-[var(--text-faint)]">
										<Film class="h-8 w-8" />
									</div>
								{/if}
							</div>

							<div class="flex min-w-0 flex-1 flex-col justify-between">
								<div>
									<span class="font-mono text-[10px] text-[var(--accent)]">
										{doc.source_label || doc.url}
									</span>
									<h3 class="mt-1 text-sm font-bold text-[var(--text)] group-hover:text-[var(--accent)]">
										{doc.title || doc.url}
									</h3>
									{#if doc.description}
										<p class="mt-1 line-clamp-2 text-xs text-[var(--text-dim)]">
											{doc.description}
										</p>
									{/if}
								</div>
								{#if doc.published_date}
									<span class="mt-2 text-[10px] text-[var(--text-faint)]">
										{doc.published_date}
									</span>
								{/if}
							</div>
						</div>
					{/each}
				</div>
			{:else}
				<!-- STANDARD WEB & LIBRARY RESULT CARDS -->
				<div class="flex flex-col gap-4">
					{#each filteredResults as doc, i}
						{@const isSelected = i === selectedIndex}
						<article
							id={`result-item-${i}`}
							onclick={() => (selectedIndex = i)}
							role="presentation"
							class={`group relative rounded-[var(--radius-md)] bg-[var(--surface)] border p-5 transition-all duration-150 ${
								isSelected
									? 'border-[var(--accent)] shadow-[var(--shadow-soft)] ring-2 ring-[var(--accent-soft)]'
									: 'border-[var(--border)] hover:border-[var(--border-strong)] hover:shadow-sm'
							}`}
						>
							<!-- Top Row: Index number, URL, Engine/Source Badge, Actions -->
							<div class="mb-2 flex items-center justify-between gap-3 text-xs">
								<div class="flex min-w-0 items-center gap-2">
									<span class="font-mono text-xs font-semibold text-[var(--text-faint)]">
										{(i + 1).toString().padStart(2, '0')}
									</span>
									<span class="text-[var(--border-strong)]">/</span>
									<button
										onclick={() => openDoc(doc)}
										class="cursor-pointer truncate font-mono text-[11px] text-[var(--accent)] hover:underline text-left max-w-sm"
									>
										{doc.url}
									</button>
									<button
										onclick={(e) => {
											e.stopPropagation();
											openExternal(doc);
										}}
										class="text-[var(--text-faint)] hover:text-[var(--text)] transition-colors p-1"
										title="Open in default browser"
										aria-label="Open in browser"
									>
										<ExternalLink class="h-3 w-3" />
									</button>
								</div>

								<div class="flex shrink-0 items-center gap-2">
									<span class="rounded-full bg-[var(--surface-hover)] border border-[var(--border)] px-2 py-0.5 font-mono text-[9px] uppercase tracking-wider text-[var(--text-dim)]">
										{doc.engine || doc.source_label || doc.source_kind || 'WEB'}
									</span>
									<button
										onclick={(e) => {
											e.stopPropagation();
											togglePin(doc);
										}}
										class={`p-1.5 rounded-full transition-colors ${
											doc.pinned
												? 'text-[var(--accent)] bg-[var(--accent-soft)]'
												: 'text-[var(--text-faint)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)]'
										}`}
										title={doc.pinned ? 'Pinned (Click to unpin)' : 'Pin document'}
										aria-label="Pin document"
									>
										<Bookmark class="h-3.5 w-3.5 fill-current" />
									</button>
								</div>
							</div>

							<!-- Large Readable Title -->
							<button
								class="mb-2 block w-full text-left text-lg font-bold tracking-tight text-[var(--text)] hover:text-[var(--accent)] transition-colors"
								onclick={() => openDoc(doc)}
							>
								{doc.title || doc.url}
							</button>

							<!-- Highlighted Snippet -->
							{#if doc.snippet}
								<div class="snippet-content mb-3.5 text-xs sm:text-sm leading-relaxed text-[var(--text-dim)]">
									{@html doc.snippet}
								</div>
							{:else if doc.description}
								<p class="mb-3.5 text-xs sm:text-sm leading-relaxed text-[var(--text-dim)]">
									{doc.description}
								</p>
							{/if}

							<!-- Subpage branches if available -->
							{#if doc.sub_pages && doc.sub_pages.length > 0}
								<div class="mt-3 pt-3 border-t border-[var(--border)] flex flex-col gap-2">
									<span class="text-[10px] uppercase font-bold tracking-wider text-[var(--text-faint)]">
										Related Pages ({doc.sub_pages.length})
									</span>
									<div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
										{#each doc.sub_pages as branch}
											<button
												onclick={() => onOpenWebTab(branch.url, branch.title)}
												class="flex items-center justify-between p-2 rounded-[var(--radius-sm)] bg-[var(--surface-hover)] hover:bg-[var(--accent-soft)] text-left transition-colors text-xs"
											>
												<span class="truncate font-semibold text-[var(--text)]">{branch.title}</span>
												<ChevronRight class="h-3.5 w-3.5 text-[var(--text-faint)] shrink-0 ml-1" />
											</button>
										{/each}
									</div>
								</div>
							{/if}

							<!-- Card Footer: Tags as soft pill chips & Copy URL -->
							<div class="mt-3 pt-2.5 border-t border-[var(--border)] flex items-center justify-between text-xs">
								<div class="flex items-center gap-1.5 flex-wrap">
									{#if doc.tags && doc.tags.length > 0}
										{#each doc.tags as tag}
											<span class="rounded-full bg-[var(--accent-soft)] px-2 py-0.5 text-[10px] font-medium text-[var(--accent)]">
												#{tag}
											</span>
										{/each}
									{/if}
								</div>

								<button
									onclick={() => copyUrl(doc)}
									class="flex items-center gap-1 text-[11px] text-[var(--text-faint)] hover:text-[var(--text)] transition-colors p-1"
								>
									{#if copiedId === doc.id}
										<Check class="h-3 w-3 text-emerald-400" />
										<span class="text-emerald-400">Copied!</span>
									{:else}
										<Share2 class="h-3 w-3" />
										<span>Copy link</span>
									{/if}
								</button>
							</div>
						</article>
					{/each}
				</div>
			{/if}
		</main>
	</div>
</div>
