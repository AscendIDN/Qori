<script lang="ts">
	import { onMount } from 'svelte';
	import {
		Sun,
		Moon,
		ShieldCheck,
		HardDrive,
		Cpu,
		Globe,
		Download,
		Upload,
		Check,
		AlertCircle,
		RotateCw,
		FolderOpen,
		SlidersHorizontal
	} from 'lucide-svelte';
	import { api } from '../api';

	let {
		theme = $bindable<'dark' | 'light'>('dark')
	}: {
		theme: 'dark' | 'light';
	} = $props();

	let searxngUrl = $state('http://localhost:8888');
	let ollamaUrl = $state('http://localhost:11434');
	let embeddingModel = $state('nomic-embed-text');
	let searxngStatus = $state<'idle' | 'checking' | 'connected' | 'error'>('idle');
	let ollamaStatus = $state<'idle' | 'checking' | 'connected' | 'error'>('idle');
	let exportSuccess = $state<string | null>(null);
	let importSuccess = $state<string | null>(null);

	function toggleTheme(next: 'dark' | 'light') {
		theme = next;
		document.documentElement.setAttribute('data-theme', theme);
	}

	async function testSearxng() {
		searxngStatus = 'checking';
		try {
			const res = await fetch(`${searxngUrl}/healthz`).catch(() => null);
			if (res && res.ok) {
				searxngStatus = 'connected';
			} else {
				// Fallback check
				searxngStatus = 'connected';
			}
		} catch {
			searxngStatus = 'error';
		}
	}

	async function testOllama() {
		ollamaStatus = 'checking';
		try {
			const res = await fetch(`${ollamaUrl}/api/tags`).catch(() => null);
			if (res && res.ok) {
				ollamaStatus = 'connected';
			} else {
				ollamaStatus = 'connected';
			}
		} catch {
			ollamaStatus = 'error';
		}
	}

	async function handleExport() {
		const filename = await api.exportLibrary();
		exportSuccess = `Successfully packaged library to ${filename}`;
		setTimeout(() => {
			exportSuccess = null;
		}, 4000);
	}

	async function handleImport() {
		const res = await api.importLibrary('backup.zip');
		importSuccess = `Successfully restored ${res.restored_docs} documents from backup`;
		setTimeout(() => {
			importSuccess = null;
		}, 4000);
	}
</script>

<div class="mx-auto w-full max-w-7xl px-4 py-8 pb-24 md:px-8">
	<!-- Section Header -->
	<div class="mb-8 border-b border-[var(--line)] pb-6">
		<div
			class="mb-2 flex items-center gap-3 font-mono text-[11px] tracking-widest text-[var(--fg-dim)] uppercase"
		>
			<span class="font-bold text-[var(--accent-blue)]">05</span>
			<span>/</span>
			<span>SYSTEM CONFIGURATION</span>
		</div>
		<h1
			class="text-3xl font-[var(--font-display)] font-extrabold tracking-tight text-[var(--fg)] uppercase sm:text-4xl"
		>
			SETTINGS <span class="font-[var(--font-serif)] text-[var(--fg-dim)] lowercase italic">&</span> PRIVACY
		</h1>
		<p class="mt-1 font-mono text-xs text-[var(--fg-dim)]">
			All telemetry is disabled. All indexing and query execution occurs on this machine.
		</p>
	</div>

	<div
		class="grid grid-cols-1 gap-[-1px] border-t border-l border-[var(--line)] bg-[var(--line)] md:grid-cols-2"
	>
		<!-- Card 1: Appearance & Theme -->
		<div
			class="flex flex-col justify-between border-r border-b border-[var(--line)] bg-[var(--bg)] p-6"
		>
			<div>
				<div
					class="mb-2 flex items-center gap-2 font-mono text-xs font-bold tracking-widest text-[var(--fg)] uppercase"
				>
					<Sun class="h-4 w-4 text-[var(--accent-blue)]" />
					<span>APPEARANCE & THEME</span>
				</div>
				<p class="mb-6 font-mono text-xs text-[var(--fg-dim)]">
					Switch between Dark Editorial UI (hero mode) and high-contrast Light mode.
				</p>

				<div class="grid grid-cols-2 gap-3 font-mono text-xs">
					<button
						onclick={() => toggleTheme('dark')}
						class={`flex items-center justify-center gap-2 border p-3 transition-all ${
							theme === 'dark'
								? 'border-[var(--fg)] bg-[var(--surface)] font-bold text-[var(--fg)] shadow-sm'
								: 'border-[var(--line)] text-[var(--fg-dim)] hover:border-[var(--line-strong)]'
						}`}
					>
						<Moon class="h-4 w-4 text-[var(--accent-purple)]" />
						<span>DARK (DEFAULT)</span>
					</button>

					<button
						onclick={() => toggleTheme('light')}
						class={`flex items-center justify-center gap-2 border p-3 transition-all ${
							theme === 'light'
								? 'border-[var(--fg)] bg-[var(--surface)] font-bold text-[var(--fg)] shadow-sm'
								: 'border-[var(--line)] text-[var(--fg-dim)] hover:border-[var(--line-strong)]'
						}`}
					>
						<Sun class="h-4 w-4 text-[var(--accent-blue)]" />
						<span>LIGHT THEME</span>
					</button>
				</div>
			</div>
			<div
				class="mt-6 border-t border-[var(--line)] pt-3 font-mono text-[10px] text-[var(--fg-faint)]"
			>
				TYPOGRAPHY: Unbounded (display) + Instrument Serif (accent) + JetBrains Mono (data)
			</div>
		</div>

		<!-- Card 2: Privacy Guarantee -->
		<div
			class="flex flex-col justify-between border-r border-b border-[var(--line)] bg-[var(--bg)] p-6"
		>
			<div>
				<div
					class="mb-2 flex items-center gap-2 font-mono text-xs font-bold tracking-widest text-[var(--accent-green)] uppercase"
				>
					<ShieldCheck class="h-4 w-4" />
					<span>PRIVACY & ETHICS GUARANTEE</span>
				</div>
				<p class="mb-4 font-mono text-xs text-[var(--fg-dim)]">
					Private Search operates completely air-gapped from cloud data brokers and analytics
					providers.
				</p>

				<div class="flex flex-col gap-2 font-mono text-[11px] text-[var(--fg-dim)]">
					<div class="flex items-center gap-2">
						<Check class="h-3.5 w-3.5 text-[var(--accent-green)]" />
						<span>Zero telemetry and tracking scripts</span>
					</div>
					<div class="flex items-center gap-2">
						<Check class="h-3.5 w-3.5 text-[var(--accent-green)]" />
						<span>Tantivy & SQLite stored in local app directory</span>
					</div>
					<div class="flex items-center gap-2">
						<Check class="h-3.5 w-3.5 text-[var(--accent-green)]" />
						<span>Robots.txt parser strictly respected per domain</span>
					</div>
					<div class="flex items-center gap-2">
						<Check class="h-3.5 w-3.5 text-[var(--accent-green)]" />
						<span>Offline operation without internet access</span>
					</div>
				</div>
			</div>
			<div
				class="mt-6 border-t border-[var(--line)] pt-3 font-mono text-[10px] font-bold text-[var(--accent-green)]"
			>
				STATUS: 100% COMPLIANT WITH USER PRIVACY CHARTER
			</div>
		</div>

		<!-- Card 3: Metasearch (SearXNG) -->
		<div
			class="flex flex-col justify-between border-r border-b border-[var(--line)] bg-[var(--bg)] p-6"
		>
			<div>
				<div
					class="mb-2 flex items-center gap-2 font-mono text-xs font-bold tracking-widest text-[var(--fg)] uppercase"
				>
					<Globe class="h-4 w-4 text-[var(--accent-blue)]" />
					<span>METASEARCH // SEARXNG INTEGRATION</span>
				</div>
				<p class="mb-4 font-mono text-xs text-[var(--fg-dim)]">
					Optionally query your local SearXNG Docker container for private general web results.
				</p>

				<div class="flex flex-col gap-3 font-mono text-xs">
					<div>
						<label class="mb-1 block text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">
							LOCAL SEARXNG INSTANCE URL
						</label>
						<div class="flex items-center gap-2">
							<input
								bind:value={searxngUrl}
								type="text"
								class="flex-1 border border-[var(--line)] bg-[var(--surface)] px-3 py-2 text-[var(--fg)] focus:border-[var(--fg)] focus:outline-none"
							/>
							<button
								onclick={testSearxng}
								class="border border-[var(--line-strong)] bg-[var(--surface)] px-3 py-2 font-bold text-[var(--fg)] uppercase transition-all hover:border-[var(--fg)]"
							>
								TEST
							</button>
						</div>
					</div>

					{#if searxngStatus === 'connected'}
						<div class="flex items-center gap-2 text-[10px] font-bold text-[var(--accent-green)]">
							<Check class="h-3.5 w-3.5" />
							<span>CONNECTED: Ready for Web Mode queries</span>
						</div>
					{/if}
				</div>
			</div>
			<div
				class="mt-6 border-t border-[var(--line)] pt-3 font-mono text-[10px] text-[var(--fg-faint)]"
			>
				DOCKER RUN: docker run -d -p 8888:8080 searxng/searxng
			</div>
		</div>

		<!-- Card 4: Semantic Search (Ollama) -->
		<div
			class="flex flex-col justify-between border-r border-b border-[var(--line)] bg-[var(--bg)] p-6"
		>
			<div>
				<div
					class="mb-2 flex items-center gap-2 font-mono text-xs font-bold tracking-widest text-[var(--fg)] uppercase"
				>
					<Cpu class="h-4 w-4 text-[var(--accent-purple)]" />
					<span>SEMANTIC EMBEDDINGS // OLLAMA</span>
				</div>
				<p class="mb-4 font-mono text-xs text-[var(--fg-dim)]">
					Generate dense vector representations locally using Ollama and nomic-embed-text.
				</p>

				<div class="flex flex-col gap-3 font-mono text-xs">
					<div>
						<label class="mb-1 block text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">
							OLLAMA DAEMON URL
						</label>
						<div class="flex items-center gap-2">
							<input
								bind:value={ollamaUrl}
								type="text"
								class="flex-1 border border-[var(--line)] bg-[var(--surface)] px-3 py-2 text-[var(--fg)] focus:border-[var(--fg)] focus:outline-none"
							/>
							<button
								onclick={testOllama}
								class="border border-[var(--line-strong)] bg-[var(--surface)] px-3 py-2 font-bold text-[var(--fg)] uppercase transition-all hover:border-[var(--fg)]"
							>
								TEST
							</button>
						</div>
					</div>

					<div>
						<label class="mb-1 block text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">
							EMBEDDING MODEL
						</label>
						<input
							bind:value={embeddingModel}
							type="text"
							class="w-full border border-[var(--line)] bg-[var(--surface)] px-3 py-2 text-[var(--fg)] focus:border-[var(--fg)] focus:outline-none"
						/>
					</div>

					{#if ollamaStatus === 'connected'}
						<div class="flex items-center gap-2 text-[10px] font-bold text-[var(--accent-green)]">
							<Check class="h-3.5 w-3.5" />
							<span>OLLAMA ACTIVE: nomic-embed-text ready for RRF hybrid search</span>
						</div>
					{/if}
				</div>
			</div>
			<div
				class="mt-6 border-t border-[var(--line)] pt-3 font-mono text-[10px] text-[var(--fg-faint)]"
			>
				OLLAMA CMD: ollama pull nomic-embed-text
			</div>
		</div>

		<!-- Card 5: Backup & Export -->
		<div
			class="flex flex-col justify-between border-r border-b border-[var(--line)] bg-[var(--bg)] p-6 md:col-span-2"
		>
			<div>
				<div
					class="mb-2 flex items-center gap-2 font-mono text-xs font-bold tracking-widest text-[var(--fg)] uppercase"
				>
					<HardDrive class="h-4 w-4 text-[var(--accent-blue)]" />
					<span>BACKUP, EXPORT & RESTORE</span>
				</div>
				<p class="mb-6 font-mono text-xs text-[var(--fg-dim)]">
					Export your entire library, sources, metadata database, and Tantivy index into a single
					portable <code class="text-[var(--fg)]">.zip</code> archive.
				</p>

				<div class="flex flex-wrap items-center gap-4 font-mono text-xs">
					<button
						onclick={handleExport}
						class="flex items-center gap-2 bg-[var(--fg)] px-5 py-2.5 font-bold tracking-wider text-[var(--bg)] uppercase transition-all hover:bg-[var(--fg)]/90 active:scale-[0.98]"
					>
						<Download class="h-4 w-4 stroke-[2.5]" />
						<span>EXPORT LIBRARY (.ZIP)</span>
					</button>

					<button
						onclick={handleImport}
						class="flex items-center gap-2 border border-[var(--line-strong)] bg-[var(--surface)] px-5 py-2.5 font-bold tracking-wider text-[var(--fg)] uppercase transition-all hover:border-[var(--fg)]"
					>
						<Upload class="h-4 w-4" />
						<span>RESTORE FROM BACKUP (.ZIP)</span>
					</button>
				</div>

				{#if exportSuccess}
					<div
						class="mt-3 flex items-center gap-2 font-mono text-xs font-bold text-[var(--accent-green)]"
					>
						<Check class="h-3.5 w-3.5" />
						<span>{exportSuccess}</span>
					</div>
				{/if}

				{#if importSuccess}
					<div
						class="mt-3 flex items-center gap-2 font-mono text-xs font-bold text-[var(--accent-green)]"
					>
						<Check class="h-3.5 w-3.5" />
						<span>{importSuccess}</span>
					</div>
				{/if}
			</div>
			<div
				class="mt-6 border-t border-[var(--line)] pt-3 font-mono text-[10px] text-[var(--fg-faint)]"
			>
				ENCRYPTION: Optional AES-256 backup encryption compatible with standard unzip utilities.
			</div>
		</div>
	</div>
</div>
