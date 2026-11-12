<script lang="ts">
	import { onMount } from 'svelte';
	import {
		Globe,
		Folder,
		Bookmark,
		Terminal,
		Play,
		Pause,
		Trash2,
		Plus,
		AlertCircle,
		CheckCircle,
		Clock,
		ExternalLink,
		Shield
	} from 'lucide-svelte';
	import type { Source, CrawlProgressEvent } from '../types';
	import { api } from '../api';

	let sources = $state<Source[]>([]);
	let isSubmitting = $state(false);
	let isCrawlPaused = $state(false);

	// Form State
	let addKind = $state<'web' | 'folder' | 'bookmark'>('web');
	let addUri = $state('');
	let addLabel = $state('');
	let addDepth = $state(2);
	let addRefreshHours = $state(24);

	// Mock Live Stream Event Log
	let crawlLogs = $state<CrawlProgressEvent[]>([
		{
			url: 'https://news.ycombinator.com',
			status: 'indexed',
			message: 'Parsed 30 stories, extracted 45 semantic chunks',
			timestamp: '09:14:22'
		},
		{
			url: 'https://en.wikipedia.org/wiki/Vector_database',
			status: 'indexed',
			message: 'Ingested 2,340 tokens, generated nomic-embed-text vector',
			timestamp: '09:15:02'
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

	onMount(() => {
		loadSources();
	});
</script>

<div class="mx-auto w-full max-w-6xl px-4 py-6 pb-24 md:px-8 select-none">
	<!-- Section Header -->
	<div class="mb-8 border-b border-[var(--border)] pb-5">
		<h1 class="text-2xl font-bold tracking-tight text-[var(--text)]">
			Sources & Crawler Daemon
		</h1>
		<p class="mt-1 text-xs text-[var(--text-dim)]">
			Polite local crawler honoring robots.txt, per-domain rate limits, and directory file watchers.
		</p>
	</div>

	<div class="grid grid-cols-1 gap-6 lg:grid-cols-12">
		<!-- Left Column: Add New Source Form -->
		<div class="flex flex-col gap-6 lg:col-span-5">
			<div class="rounded-[var(--radius-md)] bg-[var(--surface)] border border-[var(--border)] p-6 shadow-sm">
				<div class="mb-5 flex items-center gap-2 border-b border-[var(--border)] pb-3 text-xs font-bold text-[var(--text)] uppercase tracking-wider">
					<Plus class="h-4 w-4 text-[var(--accent)]" />
					<span>Add Monitored Source</span>
				</div>

				<!-- Kind Selector Buttons -->
				<div class="mb-5 flex items-center rounded-full bg-[var(--surface-hover)] p-1 border border-[var(--border)]">
					<button
						onclick={() => (addKind = 'web')}
						class={`flex flex-1 items-center justify-center gap-1.5 py-1.5 rounded-full text-xs font-semibold transition-all ${
							addKind === 'web'
								? 'bg-[var(--accent)] text-white shadow-xs'
								: 'text-[var(--text-dim)] hover:text-[var(--text)]'
						}`}
					>
						<Globe class="h-3.5 w-3.5" />
						<span>Web URL</span>
					</button>

					<button
						onclick={() => (addKind = 'folder')}
						class={`flex flex-1 items-center justify-center gap-1.5 py-1.5 rounded-full text-xs font-semibold transition-all ${
							addKind === 'folder'
								? 'bg-[var(--accent)] text-white shadow-xs'
								: 'text-[var(--text-dim)] hover:text-[var(--text)]'
						}`}
					>
						<Folder class="h-3.5 w-3.5" />
						<span>Folder</span>
					</button>

					<button
						onclick={() => (addKind = 'bookmark')}
						class={`flex flex-1 items-center justify-center gap-1.5 py-1.5 rounded-full text-xs font-semibold transition-all ${
							addKind === 'bookmark'
								? 'bg-[var(--accent)] text-white shadow-xs'
								: 'text-[var(--text-dim)] hover:text-[var(--text)]'
						}`}
					>
						<Bookmark class="h-3.5 w-3.5" />
						<span>Bookmarks</span>
					</button>
				</div>

				<form
					onsubmit={(e) => {
						e.preventDefault();
						handleAddSource();
					}}
					class="flex flex-col gap-4 text-xs"
				>
					<!-- URI Input -->
					<div>
						<label
							for="source-uri"
							class="mb-1.5 block text-[10px] font-bold tracking-wider text-[var(--text-faint)] uppercase"
						>
							{addKind === 'web'
								? 'Target Root URL'
								: addKind === 'folder'
									? 'Local Folder Path'
									: 'Bookmarks HTML / JSON Path'}
						</label>
						<input
							id="source-uri"
							bind:value={addUri}
							type="text"
							required
							placeholder={addKind === 'web'
								? 'https://example.com/docs'
								: addKind === 'folder'
									? 'C:/Users/name/Documents/Notes'
									: 'C:/Users/name/Downloads/bookmarks.html'}
							class="w-full rounded-[var(--radius-sm)] border border-[var(--border)] bg-[var(--surface-hover)] px-3 py-2 text-xs text-[var(--text)] focus:border-[var(--accent)] focus:outline-none"
						/>
					</div>

					<!-- Label Input -->
					<div>
						<label
							for="source-label"
							class="mb-1.5 block text-[10px] font-bold tracking-wider text-[var(--text-faint)] uppercase"
						>
							Friendly Label (Optional)
						</label>
						<input
							id="source-label"
							bind:value={addLabel}
							type="text"
							placeholder="e.g. Personal Engineering Vault"
							class="w-full rounded-[var(--radius-sm)] border border-[var(--border)] bg-[var(--surface-hover)] px-3 py-2 text-xs text-[var(--text)] focus:border-[var(--accent)] focus:outline-none"
						/>
					</div>

					<!-- Depth & Schedule (Grid) -->
					<div class="grid grid-cols-2 gap-3">
						<div>
							<label
								for="source-depth"
								class="mb-1.5 block text-[10px] font-bold tracking-wider text-[var(--text-faint)] uppercase"
							>
								Crawl Depth
							</label>
							<select
								id="source-depth"
								bind:value={addDepth}
								class="w-full rounded-[var(--radius-sm)] border border-[var(--border)] bg-[var(--surface-hover)] px-3 py-2 text-xs text-[var(--text)] focus:border-[var(--accent)] focus:outline-none"
							>
								<option value={1}>1 (Single page)</option>
								<option value={2}>2 (Linked pages)</option>
								<option value={3}>3 (Deep crawl)</option>
								<option value={5}>5 (Exhaustive)</option>
							</select>
						</div>

						<div>
							<label
								for="source-refresh"
								class="mb-1.5 block text-[10px] font-bold tracking-wider text-[var(--text-faint)] uppercase"
							>
								Refresh Schedule
							</label>
							<select
								id="source-refresh"
								bind:value={addRefreshHours}
								class="w-full rounded-[var(--radius-sm)] border border-[var(--border)] bg-[var(--surface-hover)] px-3 py-2 text-xs text-[var(--text)] focus:border-[var(--accent)] focus:outline-none"
							>
								<option value={24}>Every 24 Hours</option>
								<option value={168}>Weekly (168h)</option>
								<option value={720}>Monthly</option>
								<option value={0}>Manual only</option>
							</select>
						</div>
					</div>

					<!-- Compliance Note -->
					<div class="flex items-start gap-2 rounded-[var(--radius-sm)] bg-[var(--surface-hover)] border border-[var(--border)] p-2.5 text-[11px] text-[var(--text-dim)]">
						<Shield class="mt-0.5 h-3.5 w-3.5 shrink-0 text-emerald-400" />
						<span>
							The crawler identifies as <code class="font-mono text-[var(--text)]">PrivateSearchBot/1.0</code>, enforces 2s delays per domain, and honors <code class="font-mono text-[var(--text)]">robots.txt</code>.
						</span>
					</div>

					<!-- Submit Button -->
					<button
						type="submit"
						disabled={isSubmitting}
						class="mt-2 flex items-center justify-center gap-2 rounded-full bg-[var(--accent)] py-2.5 text-xs font-semibold text-white shadow-sm hover:bg-[var(--accent-hover)] transition-all disabled:opacity-50"
					>
						<Plus class="h-4 w-4" />
						<span>{isSubmitting ? 'Registering...' : 'Register Source'}</span>
					</button>
				</form>
			</div>
		</div>

		<!-- Right Column: Live Terminal Crawl Log -->
		<div class="flex flex-col gap-4 lg:col-span-7">
			<div class="flex h-[540px] flex-col overflow-hidden rounded-[var(--radius-md)] border border-[var(--border)] bg-[#0C0E16] shadow-sm">
				<!-- Terminal Titlebar -->
				<div class="flex items-center justify-between border-b border-[var(--border)] bg-[var(--surface)] px-4 py-2.5 text-xs select-none">
					<div class="flex items-center gap-2">
						<Terminal class="h-3.5 w-3.5 text-emerald-400" />
						<span class="font-bold tracking-wider text-[var(--text)] uppercase text-[11px]">
							Live Ingestion Stream // Tokio Daemon
						</span>
						<span class="inline-block h-2 w-2 animate-pulse rounded-full bg-emerald-400"></span>
					</div>

					<div class="flex items-center gap-1.5">
						<button
							onclick={() => (isCrawlPaused = !isCrawlPaused)}
							class="flex items-center gap-1 rounded-full px-2.5 py-1 text-[10px] font-semibold text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)] transition-colors"
						>
							{#if isCrawlPaused}
								<Play class="h-3 w-3 text-emerald-400" />
								<span>Resume</span>
							{:else}
								<Pause class="h-3 w-3 text-rose-400" />
								<span>Pause</span>
							{/if}
						</button>

						<button
							onclick={() => (crawlLogs = [])}
							class="flex items-center gap-1 rounded-full px-2.5 py-1 text-[10px] font-semibold text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)] transition-colors"
						>
							<Trash2 class="h-3 w-3" />
							<span>Clear</span>
						</button>
					</div>
				</div>

				<!-- Terminal Output Stream -->
				<div
					bind:this={terminalContainerEl}
					class="flex-1 space-y-1.5 overflow-y-auto p-4 font-mono text-[11px] leading-relaxed text-[#c0caf5]"
				>
					<div class="mb-2 border-b border-white/5 pb-2 text-[var(--text-faint)]">
						[SYS] Local crawler daemon initialized. Awaiting scheduled jobs or manual triggers.
					</div>
					{#each crawlLogs as log}
						<div class="flex items-start gap-2.5 px-1 py-0.5 hover:bg-white/[0.03] rounded">
							<span class="shrink-0 text-[var(--text-faint)]">[{log.timestamp}]</span>
							{#if log.status === 'indexed'}
								<span class="shrink-0 font-bold text-emerald-400">[OK]</span>
							{:else if log.status === 'fetching'}
								<span class="shrink-0 font-bold text-[var(--accent)]">[FETCH]</span>
							{:else if log.status === 'skipped'}
								<span class="shrink-0 font-bold text-purple-400">[SKIP]</span>
							{:else}
								<span class="shrink-0 font-bold text-rose-400">[ERR]</span>
							{/if}
							<span class="max-w-xs truncate text-[var(--text-dim)]">{log.url}</span>
							<span class="text-[var(--text-faint)]">—</span>
							<span class="text-[var(--text)]">{log.message}</span>
						</div>
					{/each}
				</div>

				<!-- Terminal Status Bar -->
				<div class="flex items-center justify-between border-t border-[var(--border)] bg-[var(--surface)] px-4 py-2 text-[10px] text-[var(--text-faint)]">
					<div class="flex items-center gap-3">
						<span>Workers: 4 Tokio Tasks</span>
						<span>Delay: 2000ms</span>
					</div>
					<span class="text-emerald-400 font-semibold">100% Robots.txt Compliant</span>
				</div>
			</div>
		</div>
	</div>
</div>
