<script lang="ts">
	import type { AppStats } from '../types';

	let { stats }: { stats: AppStats } = $props();

	function formatBytes(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		const kb = bytes / 1024;
		if (kb < 1024) return `${kb.toFixed(1)} KB`;
		const mb = kb / 1024;
		return `${mb.toFixed(1)} MB`;
	}
</script>

<div
	class="relative w-full overflow-hidden border-y border-[var(--line)] bg-[var(--surface)] py-2 font-mono text-[11px] tracking-widest uppercase select-none"
>
	<div class="animate-marquee flex items-center whitespace-nowrap">
		{#each [1, 2] as _}
			<div class="flex items-center gap-6 pr-6">
				<span class="flex items-center gap-2 text-[var(--fg)]">
					<span class="text-[var(--fg-dim)]">DOCUMENTS INDEXED:</span>
					<span class="font-bold text-[var(--fg)]">{stats.total_documents.toLocaleString()}</span>
				</span>
				<span class="inline-block h-1.5 w-1.5 rounded-full bg-[var(--accent-blue)]"></span>

				<span class="flex items-center gap-2 text-[var(--fg)]">
					<span class="text-[var(--fg-dim)]">TANTIVY INDEX:</span>
					<span class="font-bold text-[var(--fg)]">{formatBytes(stats.index_size_bytes)}</span>
				</span>
				<span class="inline-block h-1.5 w-1.5 rounded-full bg-[var(--accent-blue)]"></span>

				<span class="flex items-center gap-2 text-[var(--fg)]">
					<span class="text-[var(--fg-dim)]">SOURCES MONITORED:</span>
					<span class="font-bold text-[var(--fg)]">{stats.total_sources} ACTIVE</span>
				</span>
				<span class="inline-block h-1.5 w-1.5 rounded-full bg-[var(--accent-blue)]"></span>

				<span class="flex items-center gap-2 text-[var(--fg)]">
					<span class="text-[var(--fg-dim)]">LAST CRAWL:</span>
					<span class="font-bold text-[var(--fg)]">{stats.last_crawl_at ?? 'JUST NOW'}</span>
				</span>
				<span class="inline-block h-1.5 w-1.5 rounded-full bg-[var(--accent-blue)]"></span>

				<span class="flex items-center gap-2 text-[var(--accent-green)]">
					<span>LOCAL ENGINE:</span>
					<span class="font-bold">ACTIVE (0 TELEMETRY)</span>
				</span>
				<span class="inline-block h-1.5 w-1.5 rounded-full bg-[var(--accent-blue)]"></span>

				<span class="flex items-center gap-2 text-[var(--fg)]">
					<span class="text-[var(--fg-dim)]">RETRIEVAL LATENCY:</span>
					<span class="font-bold text-[var(--accent-blue)]">&lt; 3.0 MS</span>
				</span>
				<span class="inline-block h-1.5 w-1.5 rounded-full bg-[var(--accent-blue)]"></span>

				<span class="flex items-center gap-2 text-[var(--fg-dim)]">
					<span>SEARCH PRIVACY:</span>
					<span class="font-bold text-[var(--fg)]">100% AIR-GAPPED READY</span>
				</span>
				<span class="inline-block h-1.5 w-1.5 rounded-full bg-[var(--accent-blue)]"></span>
			</div>
		{/each}
	</div>
</div>
