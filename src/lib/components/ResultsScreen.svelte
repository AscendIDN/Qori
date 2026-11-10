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
		Newspaper
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
	let activeCategory = $state<'Semua' | 'Gambar' | 'Video' | 'Berita'>('Semua');

	// Sync searchQuery when query prop changes
	$effect(() => {
		searchQuery = query;
	});

	const filteredResults = $derived.by(() => {
		if (activeCategory === 'Gambar') {
			return results.filter(
				(d) =>
					d.url.match(/\.(png|jpe?g|webp|gif|svg)/i) ||
					d.tags.some((t) => t.toLowerCase().includes('image') || t.toLowerCase().includes('gambar'))
			);
		}
		if (activeCategory === 'Video') {
			return results.filter(
				(d) =>
					d.url.includes('youtube.com') ||
					d.url.includes('vimeo.com') ||
					d.url.includes('video') ||
					d.tags.some((t) => t.toLowerCase().includes('video'))
			);
		}
		if (activeCategory === 'Berita') {
			return results.filter(
				(d) =>
					d.url.includes('news') ||
					d.url.includes('berita') ||
					d.tags.some((t) => t.toLowerCase().includes('news') || t.toLowerCase().includes('berita'))
			);
		}
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
	<!-- Left-Aligned Webpage Results Container (Google layout reference) -->
	<div class="mr-auto w-full max-w-3xl px-4 sm:px-6 md:pl-10 lg:pl-16 py-6">
		<main class="w-full">
			<!-- Category Bar (Semua, Gambar, Video, Berita) -->
			<div class="mb-5 flex items-center gap-1 border-b border-[var(--line)] font-mono text-xs select-none">
				{#each [
					{ id: 'Semua', label: 'Semua', icon: Globe },
					{ id: 'Gambar', label: 'Gambar', icon: Image },
					{ id: 'Video', label: 'Video', icon: Film },
					{ id: 'Berita', label: 'Berita', icon: Newspaper }
				] as cat}
					<button
						onclick={() => (activeCategory = cat.id as any)}
						class={`flex items-center gap-2 border-b-2 px-3.5 py-2.5 text-xs font-semibold tracking-wider transition-all ${
							activeCategory === cat.id
								? 'border-[var(--fg)] text-[var(--fg)] bg-[var(--surface)]/80'
								: 'border-transparent text-[var(--fg-dim)] hover:text-[var(--fg)] hover:bg-[var(--surface)]/40'
						}`}
					>
						<cat.icon class="h-3.5 w-3.5" />
						<span>{cat.label}</span>
					</button>
				{/each}
			</div>

			<!-- Results Meta Bar (Google-like clean stats) -->
			<div class="mb-6 flex flex-wrap items-center justify-between gap-3 border-b border-[var(--line)]/60 pb-3 font-mono text-xs select-none">
				<div class="flex items-center gap-2 text-xs tracking-wider text-[var(--fg-dim)]">
					<span>Hasil pencarian untuk <span class="font-bold text-[var(--fg)]">"{query}"</span></span>
				</div>

				<div class="flex items-center gap-2 text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">
					<span>{filteredResults.length} HASIL {activeCategory.toUpperCase()}</span>
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
						{@const hasBranches = doc.sub_pages && doc.sub_pages.length >= 4}

						{#if hasBranches}
							<!-- HERO RESULT CARD DENGAN CABANG SUB-HALAMAN (SITELINKS BRANCHES) -->
							<article
								id={`result-item-${i}`}
								onclick={() => (selectedIndex = i)}
								class={`group relative border transition-all ${
									isSelected
										? 'border-[var(--fg)] bg-[var(--surface)] shadow-lg'
										: 'border-[var(--line)] bg-[var(--surface)]/70 hover:border-[var(--line-strong)] hover:bg-[var(--surface)]'
								}`}
								style={`animation: resultEnter 0.35s cubic-bezier(0.16, 1, 0.3, 1) ${i * 30}ms backwards;`}
							>
								<!-- Main Card Content -->
								<div class="p-5">
									<!-- Top Domain & Breadcrumb -->
									<div class="mb-2 flex items-center justify-between gap-3 font-mono text-xs">
										<div class="flex min-w-0 items-center gap-2">
											<span class="flex h-5 w-5 shrink-0 items-center justify-center border border-[var(--line)] bg-[var(--bg-deep)] text-[10px] font-bold text-[var(--accent-blue)]">
												<Globe class="h-3 w-3" />
											</span>
											<span class="font-bold text-[var(--fg)]">
												{doc.source_label || 'WEB'}
											</span>
											<span class="text-[var(--line-strong)]">•</span>
											<span
												onclick={() => openDoc(doc)}
												class="cursor-pointer truncate text-[11px] text-[var(--accent-blue)] underline-offset-2 hover:underline"
											>
												{doc.url}
											</span>
										</div>

										<div class="flex shrink-0 items-center gap-2">
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

									<!-- Large Hero Title -->
									<h2
										class="mb-2 cursor-pointer text-xl font-bold tracking-tight text-[var(--fg)] transition-colors hover:text-[var(--accent-blue)] sm:text-2xl"
										onclick={() => openDoc(doc)}
									>
										{doc.title || doc.url}
									</h2>

									<!-- Snippet -->
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

									<!-- Action Buttons -->
									<div class="flex flex-wrap items-center gap-2 border-t border-[var(--line)]/60 pt-3 font-mono text-[10px]">
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

								<!-- Cabang Sub-Halaman Terhubung (Branches Container) -->
								<div class="border-t border-[var(--line)] bg-[var(--bg-deep)]/60">
									<div class="flex items-center justify-between border-b border-[var(--line)]/50 px-5 py-2 font-mono text-[10px] tracking-wider text-[var(--fg-faint)] uppercase">
										<span>CABANG HALAMAN TERKAIT ({doc.sub_pages?.length} HALAMAN)</span>
										<span class="font-bold text-[var(--accent-blue)]">PRIORITAS UTAMA</span>
									</div>

									<div class="divide-y divide-[var(--line)]/40">
										{#each doc.sub_pages || [] as branch}
											<div
												onclick={(e) => {
													e.stopPropagation();
													onOpenWebTab(branch.url, branch.title);
												}}
												class="group/branch flex cursor-pointer items-center justify-between px-5 py-3.5 transition-colors hover:bg-[var(--surface)]"
											>
												<div class="min-w-0 pr-4">
													<div class="flex items-center gap-2">
														<span class="text-sm font-bold text-[var(--fg)] transition-colors group-hover/branch:text-[var(--accent-blue)]">
															{branch.title}
														</span>
													</div>
													{#if branch.snippet}
														<p class="mt-0.5 line-clamp-1 text-xs text-[var(--fg-dim)]">
															{branch.snippet}
														</p>
													{/if}
													<span class="mt-0.5 block truncate font-mono text-[10px] text-[var(--accent-blue)]/75">
														{branch.url}
													</span>
												</div>

												<div class="flex shrink-0 items-center text-[var(--fg-faint)] transition-all group-hover/branch:translate-x-1 group-hover/branch:text-[var(--fg)]">
													<ChevronRight class="h-4 w-4" />
												</div>
											</div>
										{/each}
									</div>

									<!-- Footer Link: Jelajahi halaman lainnya -->
									<div class="border-t border-[var(--line)]/50 px-5 py-3">
										<button
											onclick={(e) => {
												e.stopPropagation();
												openDoc(doc);
											}}
											class="flex cursor-pointer items-center gap-1.5 font-mono text-xs font-semibold text-[var(--accent-blue)] transition-colors hover:underline"
										>
											<span>Jelajahi halaman lainnya dari {doc.source_label || 'situs ini'} »</span>
										</button>
									</div>
								</div>
							</article>
						{:else}
							<!-- STANDARD RESULT CARD -->
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
						{/if}
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
