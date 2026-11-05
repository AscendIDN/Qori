<script lang="ts">
	import { onMount } from 'svelte';
	import {
		Globe,
		Folder,
		Bookmark,
		History,
		RefreshCw,
		Plus,
		ExternalLink,
		Trash2,
		ToggleLeft,
		ToggleRight,
		SlidersHorizontal,
		FileText,
		CheckCircle2
	} from 'lucide-svelte';
	import type { Source } from '../types';
	import { api } from '../api';

	let {
		onNavigateSources = () => {},
		onManualAdd = () => {}
	}: {
		onNavigateSources?: () => void;
		onManualAdd?: () => void;
	} = $props();

	let sources = $state<Source[]>([]);
	let filterKind = $state<'all' | 'web' | 'folder' | 'bookmark' | 'history'>('all');
	let refreshingId = $state<number | null>(null);

	async function loadSources() {
		sources = await api.getSources();
	}

	async function triggerCrawl(s: Source) {
		refreshingId = s.id;
		await api.crawlNow(s.id);
		setTimeout(() => {
			if (refreshingId === s.id) refreshingId = null;
		}, 1500);
	}

	async function toggleSource(s: Source) {
		const next = !s.enabled;
		s.enabled = next;
		await api.toggleSource(s.id, next);
	}

	const filteredSources = $derived(
		filterKind === 'all' ? sources : sources.filter((s) => s.kind === filterKind)
	);

	const totalDocuments = $derived(sources.reduce((acc, s) => acc + (s.doc_count || 0), 0));

	onMount(() => {
		loadSources();
	});
</script>

<div class="mx-auto w-full max-w-7xl px-4 py-8 pb-24 md:px-8">
	<!-- Section Header -->
	<div
		class="mb-8 flex flex-col justify-between gap-4 border-b border-[var(--line)] pb-6 md:flex-row md:items-end"
	>
		<div>
			<div
				class="mb-2 flex items-center gap-3 font-mono text-[11px] tracking-widest text-[var(--fg-dim)] uppercase"
			>
				<span class="font-bold text-[var(--accent-blue)]">03</span>
				<span>/</span>
				<span>INDEX ARCHIVE</span>
			</div>
			<h1
				class="text-3xl font-[var(--font-display)] font-extrabold tracking-tight text-[var(--fg)] uppercase sm:text-4xl"
			>
				LIBRARY <span class="font-[var(--font-serif)] text-[var(--fg-dim)] lowercase italic">&</span
				> COLLECTIONS
			</h1>
			<p class="mt-1 font-mono text-xs text-[var(--fg-dim)]">
				Hairline matrix of monitored domains, local directory vaults, and browser bookmarks.
			</p>
		</div>

		<!-- Filter & Action Controls -->
		<div class="flex flex-wrap items-center gap-3 font-mono text-xs">
			<div class="flex items-center border border-[var(--line)] bg-[var(--surface)] p-0.5">
				{#each ['all', 'web', 'folder', 'bookmark'] as k}
					<button
						onclick={() => (filterKind = k as any)}
						class={`px-3 py-1 text-[10px] tracking-wider uppercase transition-all ${
							filterKind === k
								? 'bg-[var(--fg)] font-bold text-[var(--bg)]'
								: 'text-[var(--fg-dim)] hover:text-[var(--fg)]'
						}`}
					>
						{k}
					</button>
				{/each}
			</div>

			<button
				onclick={onManualAdd}
				class="flex items-center gap-1.5 border border-[var(--line-strong)] bg-[var(--surface)] px-3 py-1.5 font-mono text-xs font-bold tracking-wider text-[var(--fg)] uppercase transition-all hover:border-[var(--fg)]"
			>
				<FileText class="h-3.5 w-3.5 text-[var(--accent-green)]" />
				<span>MANUAL DOC</span>
			</button>

			<button
				onclick={onNavigateSources}
				class="flex items-center gap-1.5 bg-[var(--fg)] px-3.5 py-1.5 font-mono text-xs font-bold tracking-wider text-[var(--bg)] uppercase transition-all hover:bg-[var(--fg)]/90 active:scale-[0.98]"
			>
				<Plus class="h-3.5 w-3.5 stroke-[2.5]" />
				<span>NEW SOURCE</span>
			</button>
		</div>
	</div>

	<!-- Summary Metric Strip -->
	<div
		class="mb-8 grid grid-cols-2 gap-[-1px] border border-[var(--line)] bg-[var(--bg-deep)] font-mono text-xs sm:grid-cols-4"
	>
		<div class="border-r border-b border-[var(--line)] p-4 sm:border-b-0">
			<div class="mb-1 text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">
				TOTAL SOURCES
			</div>
			<div class="text-xl font-bold text-[var(--fg)] sm:text-2xl">{sources.length}</div>
		</div>
		<div class="border-b border-[var(--line)] p-4 sm:border-r sm:border-b-0">
			<div class="mb-1 text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">
				DOCUMENTS INDEXED
			</div>
			<div class="text-xl font-bold text-[var(--accent-blue)] sm:text-2xl">
				{totalDocuments.toLocaleString()}
			</div>
		</div>
		<div class="border-r border-[var(--line)] p-4">
			<div class="mb-1 text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">
				ACTIVE CRAWLERS
			</div>
			<div class="text-xl font-bold text-[var(--accent-green)] sm:text-2xl">
				{sources.filter((s) => s.enabled).length} / {sources.length}
			</div>
		</div>
		<div class="p-4">
			<div class="mb-1 text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">
				STORAGE ENGINE
			</div>
			<div class="text-xl font-bold text-[var(--fg)] sm:text-2xl">SQLITE + TANTIVY</div>
		</div>
	</div>

	<!-- 1px Hairline Grid Layout (ArtCraft Style) -->
	<div
		class="grid grid-cols-1 gap-[-1px] border-t border-l border-[var(--line)] bg-[var(--line)] md:grid-cols-2 xl:grid-cols-3"
	>
		{#each filteredSources as source (source.id)}
			<div
				class="group relative flex flex-col justify-between border-r border-b border-[var(--line)] bg-[var(--bg)] p-6 transition-all duration-200 hover:border-[var(--line-strong)] hover:bg-[var(--surface)]"
			>
				<!-- Top bar: Source Kind Icon, Type, Enabled Toggle -->
				<div>
					<div class="mb-4 flex items-center justify-between gap-3 font-mono text-xs">
						<div class="flex items-center gap-2">
							{#if source.kind === 'web'}
								<div
									class="border border-[var(--line)] bg-[var(--surface)] p-1.5 text-[var(--accent-blue)] transition-colors group-hover:border-[var(--accent-blue)]"
								>
									<Globe class="h-4 w-4" />
								</div>
							{:else if source.kind === 'folder'}
								<div
									class="border border-[var(--line)] bg-[var(--surface)] p-1.5 text-[var(--accent-green)] transition-colors group-hover:border-[var(--accent-green)]"
								>
									<Folder class="h-4 w-4" />
								</div>
							{:else}
								<div
									class="border border-[var(--line)] bg-[var(--surface)] p-1.5 text-[var(--accent-purple)] transition-colors group-hover:border-[var(--accent-purple)]"
								>
									<Bookmark class="h-4 w-4" />
								</div>
							{/if}
							<span class="text-[10px] font-bold tracking-widest text-[var(--fg-dim)] uppercase">
								{source.kind}
							</span>
						</div>

						<button
							onclick={() => toggleSource(source)}
							class={`flex items-center gap-1.5 border px-2 py-0.5 font-mono text-[9px] tracking-wider uppercase transition-all ${
								source.enabled
									? 'border-[var(--accent-green)]/40 bg-[var(--accent-green)]/10 text-[var(--accent-green)]'
									: 'border-[var(--line)] text-[var(--fg-faint)]'
							}`}
							title={source.enabled ? 'Source active' : 'Source disabled'}
						>
							<span
								class={`h-1.5 w-1.5 rounded-full ${source.enabled ? 'bg-[var(--accent-green)]' : 'bg-[var(--fg-faint)]'}`}
							></span>
							<span>{source.enabled ? 'ENABLED' : 'PAUSED'}</span>
						</button>
					</div>

					<!-- Title and URI -->
					<h2
						class="mb-1 line-clamp-1 text-base font-bold text-[var(--fg)] transition-colors group-hover:text-[var(--accent-blue)] sm:text-lg"
					>
						{source.label || source.uri}
					</h2>
					<div class="mb-4 truncate font-mono text-[11px] text-[var(--fg-faint)] select-all">
						{source.uri}
					</div>
				</div>

				<!-- Middle stats -->
				<div
					class="my-4 grid grid-cols-2 gap-2 border-y border-[var(--line)] py-3 font-mono text-[10px]"
				>
					<div>
						<span class="block text-[var(--fg-faint)] uppercase">DOCS</span>
						<span class="text-sm font-bold text-[var(--fg)]">
							{(source.doc_count || 0).toLocaleString()}
						</span>
					</div>
					<div>
						<span class="block text-[var(--fg-faint)] uppercase">SCHEDULE</span>
						<span class="text-sm font-bold text-[var(--fg-dim)]">
							{source.refresh_hours ? `EVERY ${source.refresh_hours}H` : 'MANUAL'}
						</span>
					</div>
				</div>

				<!-- Bottom Actions Row -->
				<div class="flex items-center justify-between pt-1 font-mono text-xs">
					<span class="text-[9px] text-[var(--fg-faint)]">
						LAST RUN: {source.last_run_at?.slice(0, 10) || 'NEVER'}
					</span>

					<div class="flex items-center gap-2">
						<button
							onclick={() => triggerCrawl(source)}
							class="flex items-center gap-1 border border-[var(--line)] px-2.5 py-1 text-[10px] font-bold tracking-wider text-[var(--fg-dim)] uppercase transition-all hover:border-[var(--fg)] hover:bg-[var(--fg)] hover:text-[var(--bg)] hover:text-[var(--fg)]"
							title="Trigger immediate re-crawl"
						>
							<RefreshCw class={`h-3 w-3 ${refreshingId === source.id ? 'animate-spin' : ''}`} />
							<span>CRAWL</span>
						</button>
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>
