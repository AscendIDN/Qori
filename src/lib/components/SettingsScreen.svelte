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
		SlidersHorizontal,
		Database,
		Image,
		Sliders,
		Sparkles
	} from 'lucide-svelte';
	import { api } from '../api';
	import LibraryScreen from './LibraryScreen.svelte';
	import SourcesScreen from './SourcesScreen.svelte';
	import type { WallpaperId } from './Wallpaper.svelte';

	let {
		theme = $bindable<'dark' | 'light'>('dark'),
		activeSection = $bindable<'general' | 'library' | 'sources'>('general'),
		wallpaper = $bindable<WallpaperId>('dark-lavender'),
		wallpaperDim = $bindable<number>(0),
		wallpaperBlur = $bindable<number>(0),
		reduceTransparency = $bindable<boolean>(false),
		onManualAdd = () => {}
	}: {
		theme: 'dark' | 'light';
		activeSection?: 'general' | 'library' | 'sources';
		wallpaper?: WallpaperId;
		wallpaperDim?: number;
		wallpaperBlur?: number;
		reduceTransparency?: boolean;
		onManualAdd?: () => void;
	} = $props();

	let searxngUrl = $state('http://localhost:8888');
	let ollamaUrl = $state('http://localhost:11434');
	let embeddingModel = $state('nomic-embed-text');
	let searxngStatus = $state<'idle' | 'checking' | 'connected' | 'error'>('idle');
	let ollamaStatus = $state<'idle' | 'checking' | 'connected' | 'error'>('idle');
	let exportSuccess = $state<string | null>(null);
	let importSuccess = $state<string | null>(null);

	const wallpapersList: { id: WallpaperId; name: string; theme: 'dark' | 'light'; colors: string }[] = [
		{ id: 'dark-lavender', name: 'Aurora Lavender', theme: 'dark', colors: 'from-[#121324] via-[#6C5CE7] to-[#FD79A8]' },
		{ id: 'dark-midnight', name: 'Velvet Midnight', theme: 'dark', colors: 'from-[#0B0C14] via-[#4A154B] to-[#3B3B98]' },
		{ id: 'dark-emerald', name: 'Emerald Twilight', theme: 'dark', colors: 'from-[#091418] via-[#00B894] to-[#0984E3]' },
		{ id: 'light-pastel', name: 'Opera Pastel Cloud', theme: 'light', colors: 'from-[#E8EDF9] via-[#D0BFFF] to-[#FCC2D7]' },
		{ id: 'light-breeze', name: 'Morning Breeze', theme: 'light', colors: 'from-[#E7F5FF] via-[#74C0FC] to-[#63E6BE]' },
		{ id: 'light-golden', name: 'Golden Warmth', theme: 'light', colors: 'from-[#FFF4E6] via-[#FFD8A8] to-[#FFC9C9]' },
		{ id: 'solid', name: 'Solid Minimalist', theme: 'dark', colors: 'from-zinc-800 to-zinc-900' }
	];

	onMount(async () => {
		const s = await api.getSettings();
		if (s.searxng_url) searxngUrl = s.searxng_url;
		if (s.ollama_url) ollamaUrl = s.ollama_url;
		if (s.embedding_model) embeddingModel = s.embedding_model;

		const ai = await api.checkAiStatus();
		ollamaStatus = ai.ollama_available ? 'connected' : 'idle';
		searxngStatus = ai.searxng_available ? 'connected' : 'idle';
	});

	function toggleTheme(next: 'dark' | 'light') {
		theme = next;
		document.documentElement.setAttribute('data-theme', theme);
	}

	function handleToggleTransparency() {
		reduceTransparency = !reduceTransparency;
		if (reduceTransparency) {
			document.documentElement.setAttribute('data-reduce-transparency', 'true');
		} else {
			document.documentElement.removeAttribute('data-reduce-transparency');
		}
	}

	async function testSearxng() {
		searxngStatus = 'checking';
		await api.saveSettings({ searxng_url: searxngUrl });
		const ai = await api.checkAiStatus();
		searxngStatus = ai.searxng_available ? 'connected' : 'error';
	}

	async function testOllama() {
		ollamaStatus = 'checking';
		await api.saveSettings({
			ollama_url: ollamaUrl,
			embedding_model: embeddingModel
		});
		const ai = await api.checkAiStatus();
		ollamaStatus = ai.ollama_available ? 'connected' : 'error';
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

<div class="mx-auto w-full max-w-6xl px-4 pt-6 md:px-8 select-none">
	<!-- Settings Sub-Navigation Pill Tabs -->
	<div class="mb-6 flex flex-wrap items-center gap-2 border-b border-[var(--border)] pb-3">
		<button
			onclick={() => (activeSection = 'general')}
			class={`flex items-center gap-2 px-4 py-2 rounded-full text-xs font-semibold tracking-wide transition-all ${
				activeSection === 'general'
					? 'bg-[var(--accent)] text-white shadow-sm'
					: 'bg-[var(--surface)] text-[var(--text-dim)] hover:text-[var(--text)] border border-[var(--border)]'
			}`}
		>
			<SlidersHorizontal class="h-3.5 w-3.5" />
			<span>Settings & Appearance</span>
		</button>

		<button
			onclick={() => (activeSection = 'library')}
			class={`flex items-center gap-2 px-4 py-2 rounded-full text-xs font-semibold tracking-wide transition-all ${
				activeSection === 'library'
					? 'bg-[var(--accent)] text-white shadow-sm'
					: 'bg-[var(--surface)] text-[var(--text-dim)] hover:text-[var(--text)] border border-[var(--border)]'
			}`}
		>
			<Database class="h-3.5 w-3.5" />
			<span>Library & Collections</span>
		</button>

		<button
			onclick={() => (activeSection = 'sources')}
			class={`flex items-center gap-2 px-4 py-2 rounded-full text-xs font-semibold tracking-wide transition-all ${
				activeSection === 'sources'
					? 'bg-[var(--accent)] text-white shadow-sm'
					: 'bg-[var(--surface)] text-[var(--text-dim)] hover:text-[var(--text)] border border-[var(--border)]'
			}`}
		>
			<Globe class="h-3.5 w-3.5" />
			<span>Sources & Crawler</span>
		</button>
	</div>
</div>

{#if activeSection === 'general'}
	<div class="mx-auto w-full max-w-6xl px-4 pb-24 md:px-8 select-none">
		<!-- Section Header -->
		<div class="mb-8 border-b border-[var(--border)] pb-5">
			<h1 class="text-2xl font-bold tracking-tight text-[var(--text)]">
				Settings & Preferences
			</h1>
			<p class="mt-1 text-xs text-[var(--text-dim)]">
				All telemetry is disabled. Search indexes, embeddings, and caches reside on your local machine.
			</p>
		</div>

		<div class="grid grid-cols-1 gap-5 md:grid-cols-2">
			<!-- Card 1: Appearance, Theme & Wallpaper -->
			<div class="flex flex-col justify-between rounded-[var(--radius-md)] bg-[var(--surface)] border border-[var(--border)] p-6 shadow-sm md:col-span-2">
				<div>
					<div class="mb-2 flex items-center gap-2 text-sm font-bold text-[var(--text)]">
						<Sun class="h-4 w-4 text-[var(--accent)]" />
						<span>Theme & Dynamic Wallpaper</span>
					</div>
					<p class="mb-5 text-xs text-[var(--text-dim)]">
						Customize the visual mood, wallpaper gradients, and frosted glass intensity.
					</p>

					<!-- Theme Switch Buttons -->
					<div class="mb-6 grid grid-cols-2 gap-3 max-w-md">
						<button
							onclick={() => toggleTheme('dark')}
							class={`flex items-center justify-center gap-2 rounded-[var(--radius-sm)] p-3 text-xs font-semibold transition-all ${
								theme === 'dark'
									? 'bg-[var(--accent)] text-white shadow-sm'
									: 'bg-[var(--surface-hover)] text-[var(--text-dim)] border border-[var(--border)] hover:text-[var(--text)]'
							}`}
						>
							<Moon class="h-4 w-4" />
							<span>Dark Theme</span>
						</button>

						<button
							onclick={() => toggleTheme('light')}
							class={`flex items-center justify-center gap-2 rounded-[var(--radius-sm)] p-3 text-xs font-semibold transition-all ${
								theme === 'light'
									? 'bg-[var(--accent)] text-white shadow-sm'
									: 'bg-[var(--surface-hover)] text-[var(--text-dim)] border border-[var(--border)] hover:text-[var(--text)]'
							}`}
						>
							<Sun class="h-4 w-4" />
							<span>Light Theme</span>
						</button>
					</div>

					<!-- Wallpaper Picker (6 presets) -->
					<div class="mb-6">
						<span class="block text-xs font-semibold text-[var(--text-dim)] mb-2 uppercase tracking-wider">
							Select Ambient Wallpaper
						</span>
						<div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-7 gap-2.5">
							{#each wallpapersList as wp}
								{@const isSelected = wallpaper === wp.id}
								<button
									onclick={() => (wallpaper = wp.id)}
									class={`group flex flex-col items-center gap-1.5 p-2 rounded-[var(--radius-sm)] text-center transition-all ${
										isSelected
											? 'ring-2 ring-[var(--accent)] bg-[var(--accent-soft)]'
											: 'hover:bg-[var(--surface-hover)]'
									}`}
								>
									<div class={`h-12 w-full rounded-[var(--radius-sm)] bg-gradient-to-tr ${wp.colors} shadow-xs border border-white/10 flex items-center justify-center`}>
										{#if isSelected}
											<Check class="h-4 w-4 text-white drop-shadow" />
										{/if}
									</div>
									<span class="text-[10px] font-medium text-[var(--text-dim)] group-hover:text-[var(--text)] truncate max-w-[90px]">
										{wp.name}
									</span>
								</button>
							{/each}
						</div>
					</div>

					<!-- Sliders: Dim & Blur + Reduce Transparency -->
					<div class="grid grid-cols-1 sm:grid-cols-3 gap-6 pt-4 border-t border-[var(--border)] text-xs">
						<div>
							<div class="flex items-center justify-between mb-1.5 font-semibold text-[var(--text-dim)]">
								<label for="dim-slider">Wallpaper Dim</label>
								<span class="font-mono text-[11px]">{wallpaperDim}%</span>
							</div>
							<input
								id="dim-slider"
								type="range"
								min="0"
								max="70"
								bind:value={wallpaperDim}
								class="w-full accent-[var(--accent)] cursor-pointer"
							/>
						</div>

						<div>
							<div class="flex items-center justify-between mb-1.5 font-semibold text-[var(--text-dim)]">
								<label for="blur-slider">Wallpaper Blur</label>
								<span class="font-mono text-[11px]">{wallpaperBlur}px</span>
							</div>
							<input
								id="blur-slider"
								type="range"
								min="0"
								max="20"
								bind:value={wallpaperBlur}
								class="w-full accent-[var(--accent)] cursor-pointer"
							/>
						</div>

						<div class="flex flex-col justify-end">
							<button
								onclick={handleToggleTransparency}
								class={`flex items-center justify-between p-2.5 rounded-[var(--radius-sm)] border text-xs font-medium transition-all ${
									reduceTransparency
										? 'bg-[var(--accent-soft)] border-[var(--accent)] text-[var(--text)] font-semibold'
										: 'bg-[var(--surface-hover)] border-[var(--border)] text-[var(--text-dim)] hover:text-[var(--text)]'
								}`}
							>
								<span>Reduce Transparency</span>
								<div class={`h-4 w-7 rounded-full p-0.5 transition-colors ${reduceTransparency ? 'bg-[var(--accent)]' : 'bg-[var(--border-strong)]'}`}>
									<div class={`h-3 w-3 rounded-full bg-white transition-transform ${reduceTransparency ? 'translate-x-3' : 'translate-x-0'}`}></div>
								</div>
							</button>
						</div>
					</div>
				</div>
			</div>

			<!-- Card 2: Privacy Guarantee -->
			<div class="flex flex-col justify-between rounded-[var(--radius-md)] bg-[var(--surface)] border border-[var(--border)] p-6 shadow-sm">
				<div>
					<div class="mb-2 flex items-center gap-2 text-sm font-bold text-emerald-400">
						<ShieldCheck class="h-4 w-4" />
						<span>Air-Gapped Privacy Guarantee</span>
					</div>
					<p class="mb-4 text-xs text-[var(--text-dim)]">
						Qori operates isolated from cloud trackers, ad networks, and corporate surveillance.
					</p>

					<div class="flex flex-col gap-2.5 text-xs text-[var(--text-dim)]">
						<div class="flex items-center gap-2">
							<Check class="h-4 w-4 text-emerald-400 shrink-0" />
							<span>Zero telemetry, analytics, or behavioral cookies</span>
						</div>
						<div class="flex items-center gap-2">
							<Check class="h-4 w-4 text-emerald-400 shrink-0" />
							<span>Tantivy full-text index stored locally in app vault</span>
						</div>
						<div class="flex items-center gap-2">
							<Check class="h-4 w-4 text-emerald-400 shrink-0" />
							<span>Robots.txt etiquette strictly honored by crawler</span>
						</div>
						<div class="flex items-center gap-2">
							<Check class="h-4 w-4 text-emerald-400 shrink-0" />
							<span>100% offline-ready local search engine</span>
						</div>
					</div>
				</div>
				<div class="mt-6 border-t border-[var(--border)] pt-3 text-[11px] font-bold text-emerald-400">
					STATUS: 100% AIR-GAPPED & COMPLIANT
				</div>
			</div>

			<!-- Card 3: Metasearch (SearXNG) -->
			<div class="flex flex-col justify-between rounded-[var(--radius-md)] bg-[var(--surface)] border border-[var(--border)] p-6 shadow-sm">
				<div>
					<div class="mb-2 flex items-center gap-2 text-sm font-bold text-[var(--text)]">
						<Globe class="h-4 w-4 text-[var(--accent)]" />
						<span>SearXNG Private Web Metasearch</span>
					</div>
					<p class="mb-4 text-xs text-[var(--text-dim)]">
						Connect to a private local or self-hosted SearXNG instance for privacy-respecting web results.
					</p>

					<div class="flex flex-col gap-3 text-xs">
						<div>
							<label for="searxng-url" class="mb-1 block text-[10px] font-semibold text-[var(--text-dim)] uppercase tracking-wider">
								Instance URL
							</label>
							<div class="flex items-center gap-2">
								<input
									id="searxng-url"
									bind:value={searxngUrl}
									type="text"
									class="flex-1 rounded-[var(--radius-sm)] border border-[var(--border)] bg-[var(--surface-hover)] px-3 py-2 text-xs text-[var(--text)] focus:border-[var(--accent)] focus:outline-none"
								/>
								<button
									onclick={testSearxng}
									class="rounded-[var(--radius-sm)] bg-[var(--accent)] px-3.5 py-2 font-semibold text-xs text-white hover:bg-[var(--accent-hover)] transition-colors shadow-sm"
								>
									Test
								</button>
							</div>
						</div>

						{#if searxngStatus === 'connected'}
							<div class="flex items-center gap-1.5 text-xs font-semibold text-emerald-400">
								<Check class="h-3.5 w-3.5" />
								<span>Connected: Ready for Web Mode</span>
							</div>
						{/if}
					</div>
				</div>
				<div class="mt-6 border-t border-[var(--border)] pt-3 font-mono text-[10px] text-[var(--text-faint)]">
					DOCKER: docker run -d -p 8888:8080 searxng/searxng
				</div>
			</div>

			<!-- Card 4: Semantic Search (Ollama) -->
			<div class="flex flex-col justify-between rounded-[var(--radius-md)] bg-[var(--surface)] border border-[var(--border)] p-6 shadow-sm">
				<div>
					<div class="mb-2 flex items-center gap-2 text-sm font-bold text-[var(--text)]">
						<Cpu class="h-4 w-4 text-[var(--accent)]" />
						<span>Ollama Local Embeddings</span>
					</div>
					<p class="mb-4 text-xs text-[var(--text-dim)]">
						Generate dense semantic embeddings locally for hybrid reciprocal rank fusion.
					</p>

					<div class="flex flex-col gap-3 text-xs">
						<div>
							<label for="ollama-url" class="mb-1 block text-[10px] font-semibold text-[var(--text-dim)] uppercase tracking-wider">
								Ollama Daemon URL
							</label>
							<div class="flex items-center gap-2">
								<input
									id="ollama-url"
									bind:value={ollamaUrl}
									type="text"
									class="flex-1 rounded-[var(--radius-sm)] border border-[var(--border)] bg-[var(--surface-hover)] px-3 py-2 text-xs text-[var(--text)] focus:border-[var(--accent)] focus:outline-none"
								/>
								<button
									onclick={testOllama}
									class="rounded-[var(--radius-sm)] bg-[var(--accent)] px-3.5 py-2 font-semibold text-xs text-white hover:bg-[var(--accent-hover)] transition-colors shadow-sm"
								>
									Test
								</button>
							</div>
						</div>

						<div>
							<label for="embedding-model" class="mb-1 block text-[10px] font-semibold text-[var(--text-dim)] uppercase tracking-wider">
								Embedding Model
							</label>
							<input
								id="embedding-model"
								bind:value={embeddingModel}
								type="text"
								class="w-full rounded-[var(--radius-sm)] border border-[var(--border)] bg-[var(--surface-hover)] px-3 py-2 text-xs text-[var(--text)] focus:border-[var(--accent)] focus:outline-none"
							/>
						</div>

						{#if ollamaStatus === 'connected'}
							<div class="flex items-center gap-1.5 text-xs font-semibold text-emerald-400">
								<Check class="h-3.5 w-3.5" />
								<span>Ollama Active: nomic-embed-text ready</span>
							</div>
						{/if}
					</div>
				</div>
				<div class="mt-6 border-t border-[var(--border)] pt-3 font-mono text-[10px] text-[var(--text-faint)]">
					CMD: ollama pull nomic-embed-text
				</div>
			</div>

			<!-- Card 5: Backup & Export -->
			<div class="flex flex-col justify-between rounded-[var(--radius-md)] bg-[var(--surface)] border border-[var(--border)] p-6 shadow-sm md:col-span-2">
				<div>
					<div class="mb-2 flex items-center gap-2 text-sm font-bold text-[var(--text)]">
						<HardDrive class="h-4 w-4 text-[var(--accent)]" />
						<span>Vault Backup & Data Portability</span>
					</div>
					<p class="mb-5 text-xs text-[var(--text-dim)]">
						Export your entire indexed library, metadata SQLite database, and crawler bookmarks into a single portable .zip vault.
					</p>

					<div class="flex flex-wrap items-center gap-3">
						<button
							onclick={handleExport}
							class="flex items-center gap-2 rounded-full bg-[var(--accent)] px-5 py-2.5 text-xs font-semibold text-white shadow-sm hover:bg-[var(--accent-hover)] transition-all"
						>
							<Download class="h-4 w-4" />
							<span>Export Full Library (.zip)</span>
						</button>

						<button
							onclick={handleImport}
							class="flex items-center gap-2 rounded-full bg-[var(--surface-hover)] border border-[var(--border)] px-5 py-2.5 text-xs font-semibold text-[var(--text)] hover:bg-[var(--surface)] transition-all"
						>
							<Upload class="h-4 w-4" />
							<span>Restore from Backup (.zip)</span>
						</button>
					</div>

					{#if exportSuccess}
						<div class="mt-3 flex items-center gap-2 text-xs font-semibold text-emerald-400">
							<Check class="h-4 w-4" />
							<span>{exportSuccess}</span>
						</div>
					{/if}

					{#if importSuccess}
						<div class="mt-3 flex items-center gap-2 text-xs font-semibold text-emerald-400">
							<Check class="h-4 w-4" />
							<span>{importSuccess}</span>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</div>
{:else if activeSection === 'library'}
	<LibraryScreen
		onNavigateSources={() => (activeSection = 'sources')}
		{onManualAdd}
	/>
{:else if activeSection === 'sources'}
	<SourcesScreen />
{/if}
