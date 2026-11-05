<script lang="ts">
	import { onMount } from 'svelte';
	import {
		Plus,
		Globe,
		Folder,
		Bookmark,
		Terminal,
		Play,
		Pause,
		RotateCcw,
		Trash2,
		CheckCircle2,
		AlertCircle,
		SlidersHorizontal,
		Shield
	} from 'lucide-svelte';
	import type { Source, CrawlProgressEvent } from '../types';
	import { api } from '../api';

	let sources = $state<Source[]>([]);
	let addKind = $state<'web' | 'folder' | 'bookmark'>('web');
	let addUri = $state('');
	let addLabel = $state('');
	let addDepth = $state(1);
	let addRefreshHours = $state(168);
	let isSubmitting = $state(false);

	// Live Terminal Crawl Logs
	let isCrawlPaused = $state(false);
	let crawlLogs = $state<CrawlProgressEvent[]>([
		{
			url: 'https://artcraft.com/sitemap.xml',
			status: 'indexed',
			message: 'Discovered 184 canonical URLs from sitemap',
			timestamp: '14:22:01'
		},
		{
			url: 'https://artcraft.com/robots.txt',
			status: 'indexed',
			message: 'Robots.txt parsed: Crawl-delay: 2s respected',
			timestamp: '14:22:02'
		},
		{
			url: 'https://artcraft.com/philosophy/craft-over-code',
			status: 'indexed',
			message: 'Readability extracted 2,480 words, sha256: 9f82c448a3',
			timestamp: '14:22:05'
		},
		{
			url: 'C:/Users/jathniel/Documents/Research/Local-First-Software.md',
			status: 'indexed',
			message: 'Markdown parsed, 5,120 words indexed into Tantivy',
			timestamp: '09:15:30'
		}
	]);

	let terminalContainerEl = $state<HTMLDivElement | null>(null);

	async function loadSources() {
		sources = await api.getSources();
	}

	async function handleAddSource() {
		if (!addUri.trim()) return;
		isSubmitting = true;
		try {
			const created = await api.addSource({
				kind: addKind,
				uri: addUri.trim(),
				label: addLabel.trim() || addUri.trim(),
				crawl_depth: Number(addDepth),
				refresh_hours: Number(addRefreshHours),
				enabled: true
			});
			sources = [created, ...sources];
			addUri = '';
			addLabel = '';

			// Append log entry
			addLog({
				url: created.uri,
				status: 'indexed',
				message: `Registered new ${created.kind} source: ${created.label}`,
				timestamp: new Date().toLocaleTimeString()
			});
		} catch (err) {
			console.error('Failed to add source:', err);
		} finally {
			isSubmitting = false;
		}
	}

	function addLog(entry: CrawlProgressEvent) {
		if (isCrawlPaused) return;
		crawlLogs = [...crawlLogs.slice(-150), entry];
		setTimeout(() => {
			if (terminalContainerEl) {
				terminalContainerEl.scrollTop = terminalContainerEl.scrollHeight;
			}
		}, 30);
	}

	async function triggerManualCrawl(s: Source) {
		addLog({
			url: s.uri,
			status: 'fetching',
			message: `Starting crawl job for ${s.label} (depth=${s.crawl_depth})...`,
			timestamp: new Date().toLocaleTimeString()
		});
		await api.crawlNow(s.id);
	}

	onMount(() => {
		loadSources();
	});
</script>

<div class="mx-auto w-full max-w-7xl px-4 py-8 pb-24 md:px-8">
	<!-- Section Header -->
	<div class="mb-8 border-b border-[var(--line)] pb-6">
		<div
			class="mb-2 flex items-center gap-3 font-mono text-[11px] tracking-widest text-[var(--fg-dim)] uppercase"
		>
			<span class="font-bold text-[var(--accent-blue)]">04</span>
			<span>/</span>
			<span>SOURCES & CRAWLER ENGINE</span>
		</div>
		<h1
			class="text-3xl font-[var(--font-display)] font-extrabold tracking-tight text-[var(--fg)] uppercase sm:text-4xl"
		>
			INSPECTION <span class="font-[var(--font-serif)] text-[var(--fg-dim)] lowercase italic"
				>&</span
			> INGESTION
		</h1>
		<p class="mt-1 font-mono text-xs text-[var(--fg-dim)]">
			Polite local crawler obeying robots.txt, per-domain rate limits, and directory file watchers.
		</p>
	</div>

	<div class="grid grid-cols-1 gap-8 lg:grid-cols-12">
		<!-- Left Column: Add New Source Form -->
		<div class="flex flex-col gap-6 lg:col-span-5">
			<div class="border border-[var(--line-strong)] bg-[var(--surface)] p-6">
				<div
					class="mb-5 flex items-center gap-2 border-b border-[var(--line)] pb-3 font-mono text-xs font-bold tracking-widest text-[var(--fg)] uppercase"
				>
					<Plus class="h-4 w-4 text-[var(--accent-blue)]" />
					<span>ADD MONITORED SOURCE</span>
				</div>

				<!-- Kind Selector Buttons -->
				<div class="mb-5 flex items-center border border-[var(--line)] bg-[var(--bg-deep)] p-0.5">
					<button
						onclick={() => (addKind = 'web')}
						class={`flex flex-1 items-center justify-center gap-1.5 py-2 font-mono text-xs font-bold tracking-wider uppercase transition-all ${
							addKind === 'web'
								? 'bg-[var(--fg)] text-[var(--bg)]'
								: 'text-[var(--fg-dim)] hover:text-[var(--fg)]'
						}`}
					>
						<Globe class="h-3.5 w-3.5" />
						<span>WEB URL</span>
					</button>

					<button
						onclick={() => (addKind = 'folder')}
						class={`flex flex-1 items-center justify-center gap-1.5 py-2 font-mono text-xs font-bold tracking-wider uppercase transition-all ${
							addKind === 'folder'
								? 'bg-[var(--fg)] text-[var(--bg)]'
								: 'text-[var(--fg-dim)] hover:text-[var(--fg)]'
						}`}
					>
						<Folder class="h-3.5 w-3.5" />
						<span>FOLDER</span>
					</button>

					<button
						onclick={() => (addKind = 'bookmark')}
						class={`flex flex-1 items-center justify-center gap-1.5 py-2 font-mono text-xs font-bold tracking-wider uppercase transition-all ${
							addKind === 'bookmark'
								? 'bg-[var(--fg)] text-[var(--bg)]'
								: 'text-[var(--fg-dim)] hover:text-[var(--fg)]'
						}`}
					>
						<Bookmark class="h-3.5 w-3.5" />
						<span>BOOKMARKS</span>
					</button>
				</div>

				<form
					onsubmit={(e) => {
						e.preventDefault();
						handleAddSource();
					}}
					class="flex flex-col gap-4 font-mono text-xs"
				>
					<!-- URI Input -->
					<div>
						<label
							class="mb-1.5 block text-[10px] tracking-widest text-[var(--fg-faint)] uppercase"
						>
							{addKind === 'web'
								? 'TARGET ROOT URL'
								: addKind === 'folder'
									? 'LOCAL FOLDER PATH'
									: 'BOOKMARKS HTML / JSON PATH'}
						</label>
						<input
							bind:value={addUri}
							type="text"
							required
							placeholder={addKind === 'web'
								? 'https://example.com/docs'
								: addKind === 'folder'
									? 'C:/Users/name/Documents/Notes'
									: 'C:/Users/name/Downloads/bookmarks.html'}
							class="w-full border border-[var(--line)] bg-[var(--bg)] px-3 py-2 text-[var(--fg)] placeholder:text-[var(--fg-faint)] focus:border-[var(--fg)] focus:outline-none"
						/>
					</div>

					<!-- Label Input -->
					<div>
						<label
							class="mb-1.5 block text-[10px] tracking-widest text-[var(--fg-faint)] uppercase"
						>
							FRIENDLY LABEL (OPTIONAL)
						</label>
						<input
							bind:value={addLabel}
							type="text"
							placeholder="e.g. Personal Engineering Vault"
							class="w-full border border-[var(--line)] bg-[var(--bg)] px-3 py-2 text-[var(--fg)] placeholder:text-[var(--fg-faint)] focus:border-[var(--fg)] focus:outline-none"
						/>
					</div>

					<!-- Depth & Schedule (Grid) -->
					<div class="grid grid-cols-2 gap-3">
						<div>
							<label
								class="mb-1.5 block text-[10px] tracking-widest text-[var(--fg-faint)] uppercase"
							>
								CRAWL DEPTH
							</label>
							<select
								bind:value={addDepth}
								class="w-full border border-[var(--line)] bg-[var(--bg)] px-3 py-2 text-[var(--fg)] focus:border-[var(--fg)] focus:outline-none"
							>
								<option value={1}>1 (Single page / Flat)</option>
								<option value={2}>2 (Linked pages)</option>
								<option value={3}>3 (Deep crawl)</option>
								<option value={5}>5 (Exhaustive)</option>
							</select>
						</div>

						<div>
							<label
								class="mb-1.5 block text-[10px] tracking-widest text-[var(--fg-faint)] uppercase"
							>
								REFRESH SCHEDULE
							</label>
							<select
								bind:value={addRefreshHours}
								class="w-full border border-[var(--line)] bg-[var(--bg)] px-3 py-2 text-[var(--fg)] focus:border-[var(--fg)] focus:outline-none"
							>
								<option value={24}>Every 24 Hours</option>
								<option value={168}>Weekly (168h)</option>
								<option value={720}>Monthly</option>
								<option value={0}>Manual only</option>
							</select>
						</div>
					</div>

					<!-- Compliance Note -->
					<div
						class="flex items-start gap-2 border border-[var(--line)] bg-[var(--bg-deep)] p-2.5 text-[10px] text-[var(--fg-faint)]"
					>
						<Shield class="mt-0.5 h-3.5 w-3.5 shrink-0 text-[var(--accent-green)]" />
						<span>
							The crawler identifies as <code class="text-[var(--fg-dim)]"
								>PrivateSearchBot/1.0</code
							>, enforces 2s delays per domain, and strictly skips paths disallowed by
							<code class="text-[var(--fg-dim)]">robots.txt</code>.
						</span>
					</div>

					<!-- Submit Button -->
					<button
						type="submit"
						disabled={isSubmitting}
						class="mt-2 flex items-center justify-center gap-2 bg-[var(--fg)] py-2.5 font-mono text-xs font-bold tracking-wider text-[var(--bg)] uppercase transition-all hover:bg-[var(--fg)]/90 active:scale-[0.98] disabled:opacity-50"
					>
						<Plus class="h-4 w-4 stroke-[2.5]" />
						<span>{isSubmitting ? 'REGISTERING...' : 'REGISTER SOURCE'}</span>
					</button>
				</form>
			</div>
		</div>

		<!-- Right Column: Live Terminal Crawl Log -->
		<div class="flex flex-col gap-4 lg:col-span-7">
			<div
				class="flex h-[540px] flex-col overflow-hidden border border-[var(--line-strong)] bg-[#08090b] shadow-2xl"
			>
				<!-- Terminal Titlebar -->
				<div
					class="flex items-center justify-between border-b border-[var(--line)] bg-[var(--surface)] px-4 py-2.5 font-mono text-xs select-none"
				>
					<div class="flex items-center gap-2">
						<Terminal class="h-3.5 w-3.5 text-[var(--accent-green)]" />
						<span class="font-bold tracking-widest text-[var(--fg)] uppercase">
							LIVE CRAWL LOG // TOKIO DAEMON
						</span>
						<span class="inline-block h-2 w-2 animate-pulse rounded-full bg-[var(--accent-green)]"
						></span>
					</div>

					<div class="flex items-center gap-2">
						<button
							onclick={() => (isCrawlPaused = !isCrawlPaused)}
							class="flex items-center gap-1 border border-[var(--line)] bg-[var(--bg)] px-2 py-0.5 text-[10px] text-[var(--fg-dim)] transition-colors hover:text-[var(--fg)]"
						>
							{#if isCrawlPaused}
								<Play class="h-3 w-3 text-[var(--accent-green)]" />
								<span>RESUME</span>
							{:else}
								<Pause class="h-3 w-3 text-[var(--accent-red)]" />
								<span>PAUSE</span>
							{/if}
						</button>

						<button
							onclick={() => (crawlLogs = [])}
							class="flex items-center gap-1 border border-[var(--line)] bg-[var(--bg)] px-2 py-0.5 text-[10px] text-[var(--fg-dim)] transition-colors hover:text-[var(--fg)]"
						>
							<Trash2 class="h-3 w-3" />
							<span>CLEAR</span>
						</button>
					</div>
				</div>

				<!-- Terminal Output Stream -->
				<div
					bind:this={terminalContainerEl}
					class="flex-1 space-y-1.5 overflow-y-auto p-4 font-mono text-[11px] leading-relaxed text-[#c0caf5]"
				>
					<div class="mb-2 border-b border-[var(--line)] pb-2 text-[var(--fg-faint)]">
						[SYS] Local crawler runtime started. Ready to ingest documents.
					</div>
					{#each crawlLogs as log}
						<div class="flex items-start gap-2.5 px-1 py-0.5 hover:bg-white/[0.02]">
							<span class="shrink-0 text-[var(--fg-faint)]">[{log.timestamp}]</span>
							{#if log.status === 'indexed'}
								<span class="shrink-0 font-bold text-[var(--accent-green)]">[OK]</span>
							{:else if log.status === 'fetching'}
								<span class="shrink-0 font-bold text-[var(--accent-blue)]">[FETCH]</span>
							{:else if log.status === 'skipped'}
								<span class="shrink-0 font-bold text-[var(--accent-purple)]">[SKIP]</span>
							{:else}
								<span class="shrink-0 font-bold text-[var(--accent-red)]">[ERR]</span>
							{/if}
							<span class="max-w-xs truncate text-[var(--fg-dim)]">{log.url}</span>
							<span class="text-[var(--fg-faint)]">—</span>
							<span class="text-[var(--fg)]">{log.message}</span>
						</div>
					{/each}
				</div>

				<!-- Terminal Status Bar -->
				<div
					class="flex items-center justify-between border-t border-[var(--line)] bg-[var(--surface)] px-4 py-2 font-mono text-[10px] text-[var(--fg-faint)]"
				>
					<div class="flex items-center gap-4">
						<span>WORKERS: 4 TOKIO TASKS</span>
						<span>POLITENESS: 2000MS DELAY</span>
					</div>
					<span class="text-[var(--accent-green)]">ALL ROBOTS.TXT OBEYED</span>
				</div>
			</div>
		</div>
	</div>
</div>
