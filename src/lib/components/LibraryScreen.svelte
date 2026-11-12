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
		SlidersHorizontal,
		FileText,
		CheckCircle2,
		Layers
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

<div class="mx-auto w-full max-w-6xl px-4 py-6 pb-24 md:px-8 select-none">
	<!-- Section Header -->
	<div class="mb-8 flex flex-col justify-between gap-4 border-b border-[var(--border)] pb-5 md:flex-row md:items-end">
		<div>
			<h1 class="text-2xl font-bold tracking-tight text-[var(--text)]">
				Library & Collections
			</h1>
			<p class="mt-1 text-xs text-[var(--text-dim)]">
				Indexed web domains, local directory vaults, and private browser bookmarks.
			</p>
		</div>

		<!-- Filter Pills & Action Controls -->
		<div class="flex flex-wrap items-center gap-2.5 text-xs">
			<div class="flex items-center gap-1 rounded-full bg-[var(--surface)] p-1 border border-[var(--border)]">
				{#each ['all', 'web', 'folder', 'bookmark'] as k}
					<button
						onclick={() => (filterKind = k as any)}
						class={`px-3 py-1 rounded-full text-xs font-semibold capitalize transition-all ${
							filterKind === k
								? 'bg-[var(--accent)] text-white shadow-xs'
								: 'text-[var(--text-dim)] hover:text-[var(--text)]'
						}`}
					>
						{k}
					</button>
				{/each}
			</div>

			<button
				onclick={onManualAdd}
				class="flex items-center gap-1.5 rounded-full bg-[var(--surface)] border border-[var(--border)] px-4 py-1.5 text-xs font-semibold text-[var(--text)] hover:bg-[var(--surface-hover)] transition-all"
			>
				<FileText class="h-3.5 w-3.5 text-emerald-400" />
				<span>Add Document</span>
			</button>

			<button
				onclick={onNavigateSources}
				class="flex items-center gap-1.5 rounded-full bg-[var(--accent)] px-4 py-1.5 text-xs font-semibold text-white shadow-sm hover:bg-[var(--accent-hover)] transition-all"
			>
				<Plus class="h-3.5 w-3.5" />
				<span>New Source</span>
			</button>
		</div>
	</div>

	<!-- Metric Summary Cards -->
	<div class="mb-8 grid grid-cols-2 gap-3.5 sm:grid-cols-4">
		<div class="rounded-[var(--radius-md)] bg-[var(--surface)] border border-[var(--border)] p-4 shadow-sm">
			<div class="text-[10px] font-bold uppercase tracking-wider text-[var(--text-faint)]">Total Sources</div>
			<div class="mt-1 text-xl font-bold text-[var(--text)]">{sources.length}</div>
		</div>

		<div class="rounded-[var(--radius-md)] bg-[var(--surface)] border border-[var(--border)] p-4 shadow-sm">
			<div class="text-[10px] font-bold uppercase tracking-wider text-[var(--text-faint)]">Documents Indexed</div>
			<div class="mt-1 text-xl font-bold text-[var(--accent)]">{totalDocuments.toLocaleString()}</div>
		</div>

		<div class="rounded-[var(--radius-md)] bg-[var(--surface)] border border-[var(--border)] p-4 shadow-sm">
			<div class="text-[10px] font-bold uppercase tracking-wider text-[var(--text-faint)]">Active Crawlers</div>
			<div class="mt-1 text-xl font-bold text-emerald-400">{sources.filter((s) => s.enabled).length} / {sources.length}</div>
		</div>

		<div class="rounded-[var(--radius-md)] bg-[var(--surface)] border border-[var(--border)] p-4 shadow-sm">
			<div class="text-[10px] font-bold uppercase tracking-wider text-[var(--text-faint)]">Storage Engine</div>
			<div class="mt-1 text-base font-bold text-[var(--text)]">SQLite + Tantivy</div>
		</div>
	</div>

	<!-- Sources Cards Grid (14px rounded cards) -->
	<div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
		{#each filteredSources as source (source.id)}
			<div class="group relative flex flex-col justify-between rounded-[var(--radius-md)] bg-[var(--surface)] border border-[var(--border)] hover:border-[var(--accent-soft)] p-5 shadow-sm hover:shadow-[var(--shadow-soft)] transition-all duration-150">
				<div>
					<!-- Card Header: Kind badge & toggle -->
					<div class="mb-3 flex items-center justify-between text-xs">
						<div class="flex items-center gap-2">
							<div class="h-7 w-7 rounded-[var(--radius-sm)] bg-[var(--surface-hover)] border border-[var(--border)] flex items-center justify-center text-[var(--accent)]">
								{#if source.kind === 'web'}
									<Globe class="h-3.5 w-3.5" />
								{:else if source.kind === 'folder'}
									<Folder class="h-3.5 w-3.5" />
								{:else}
									<Bookmark class="h-3.5 w-3.5" />
								{/if}
							</div>
							<span class="text-[10px] font-bold uppercase tracking-wider text-[var(--text-faint)]">
								{source.kind}
							</span>
						</div>

						<button
							onclick={() => toggleSource(source)}
							class={`px-2.5 py-0.5 rounded-full text-[10px] font-bold uppercase transition-all ${
								source.enabled
									? 'bg-emerald-500/15 text-emerald-400 border border-emerald-500/30'
									: 'bg-[var(--surface-hover)] text-[var(--text-faint)] border border-[var(--border)]'
							}`}
						>
							{source.enabled ? 'ACTIVE' : 'PAUSED'}
						</button>
					</div>

					<!-- Source Title / Label -->
					<h3 class="text-sm font-bold text-[var(--text)] truncate">
						{source.label}
					</h3>
					<p class="mt-0.5 truncate font-mono text-[11px] text-[var(--accent)]">
						{source.uri}
					</p>

					<!-- Details -->
					<div class="mt-3 flex items-center gap-3 text-[11px] text-[var(--text-dim)]">
						<div>
							<span class="font-bold text-[var(--text)]">{source.doc_count || 0}</span> docs
						</div>
						<span>•</span>
						<div>Depth: {source.crawl_depth}</div>
						<span>•</span>
						<div>Every {source.refresh_hours}h</div>
					</div>
				</div>

				<!-- Actions -->
				<div class="mt-4 pt-3 border-t border-[var(--border)] flex items-center justify-between">
					<span class="text-[10px] text-[var(--text-faint)]">
						Last: {source.last_run_at || 'Never'}
					</span>

					<button
						onclick={() => triggerCrawl(source)}
						class={`flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold text-[var(--accent)] hover:bg-[var(--accent-soft)] transition-colors ${
							refreshingId === source.id ? 'animate-spin' : ''
						}`}
						title="Crawl source now"
					>
						<RefreshCw class="h-3 w-3" />
						<span>Crawl</span>
					</button>
				</div>
			</div>
		{/each}
	</div>
</div>
