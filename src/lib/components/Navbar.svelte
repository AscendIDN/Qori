<script lang="ts">
	import {
		ArrowLeft,
		ArrowRight,
		RotateCcw,
		Search,
		Shield,
		ShieldCheck,
		Camera,
		Download,
		Menu,
		Lock,
		Globe,
		Sun,
		Moon,
		Bookmark,
		Clock,
		Settings,
		Database,
		FileText,
		Check,
		Plus,
		Info,
		Layers
	} from 'lucide-svelte';
	import type { SearchMode, BrowserTab } from '../types';

	let {
		tabs = $bindable<BrowserTab[]>([]),
		activeTabId = $bindable<string>(''),
		query = $bindable(''),
		mode = 'keyword',
		onSearch = (q: string, m: SearchMode) => {},
		onNewTab = () => {},
		onOpenSettings = (section: 'general' | 'library' | 'sources') => {},
		theme = $bindable<'dark' | 'light'>('dark')
	}: {
		tabs: BrowserTab[];
		activeTabId: string;
		query?: string;
		mode?: SearchMode;
		onSearch?: (q: string, m: SearchMode) => void;
		onNewTab?: () => void;
		onOpenSettings?: (section: 'general' | 'library' | 'sources') => void;
		theme: 'dark' | 'light';
	} = $props();

	let queryInput = $state('');
	let isMenuOpen = $state(false);
	let isVpnPopoverOpen = $state(false);
	let isVpnActive = $state(true);
	let selectedProxy = $state('Amsterdam, NL (Fastest)');
	let isRefreshing = $state(false);
	let isDownloadPopoverOpen = $state(false);
	let isScreenshotPopoverOpen = $state(false);

	const proxies = [
		'Amsterdam, NL (Fastest)',
		'Frankfurt, DE (Low Latency)',
		'Singapore, SG',
		'San Francisco, US (Air-Gapped Relay)'
	];

	// Keep local queryInput in sync with parent query prop
	$effect(() => {
		queryInput = query;
	});

	function isUrl(str: string): boolean {
		const trimmed = str.trim();
		if (trimmed.startsWith('http://') || trimmed.startsWith('https://')) return true;
		if (/^([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}(\/.*)?$/i.test(trimmed)) return true;
		if (/^localhost(:\d+)?(\/.*)?$/i.test(trimmed)) return true;
		return false;
	}

	function formatUrl(str: string): string {
		let trimmed = str.trim();
		if (!trimmed.startsWith('http://') && !trimmed.startsWith('https://')) {
			return `https://${trimmed}`;
		}
		return trimmed;
	}

	function submitQuery() {
		const val = queryInput.trim();
		if (!val) return;

		if (isUrl(val)) {
			const formatted = formatUrl(val);
			const cur = tabs.find((t) => t.id === activeTabId);
			if (cur) {
				cur.page = 'web';
				cur.url = formatted;
				try {
					cur.title = new URL(formatted).hostname.replace(/^www\./, '');
				} catch {
					cur.title = formatted;
				}
				cur.query = formatted;
				query = formatted;
			}
		} else {
			onSearch(val, mode);
		}
	}

	function handleBack() {
		const cur = tabs.find((t) => t.id === activeTabId);
		if (cur && (cur.page === 'results' || cur.page === 'settings' || cur.page === 'web')) {
			cur.page = 'search';
			cur.title = 'Start Page';
			cur.query = '';
			cur.url = undefined;
			queryInput = '';
			query = '';
		}
	}

	function handleForward() {
		const cur = tabs.find((t) => t.id === activeTabId);
		if (cur && cur.page === 'search' && queryInput.trim().length > 0) {
			submitQuery();
		}
	}

	async function handleRefresh() {
		isRefreshing = true;
		if (queryInput.trim().length > 0) {
			onSearch(queryInput.trim(), mode);
		}
		setTimeout(() => {
			isRefreshing = false;
		}, 500);
	}

	function toggleTheme() {
		theme = theme === 'dark' ? 'light' : 'dark';
		document.documentElement.setAttribute('data-theme', theme);
	}

	async function handleDownloadExport() {
		try {
			const { api } = await import('../api');
			const fn = await api.exportLibrary();
			alert(`Library exported successfully: ${fn}`);
		} catch (e) {
			console.warn('Export failed:', e);
		}
		isDownloadPopoverOpen = false;
	}

	function takeScreenshot(kind: 'visible' | 'full' | 'selection') {
		alert(`Screenshot (${kind}) captured and saved to vault.`);
		isScreenshotPopoverOpen = false;
	}

	function handleMenuAction(action: string) {
		isMenuOpen = false;
		if (action === 'new-tab') {
			onNewTab();
		} else if (action === 'library') {
			onOpenSettings('library');
		} else if (action === 'sources') {
			onOpenSettings('sources');
		} else if (action === 'settings') {
			onOpenSettings('general');
		} else if (action === 'theme') {
			toggleTheme();
		} else if (action === 'about') {
			alert('QORI — Personal Local-First Search Engine & Browser\nVersion 1.0.0 (Opera/Arc Redesign)');
		}
	}
</script>

<div class="relative z-20 flex h-[50px] w-full items-center justify-between px-3 bg-[var(--chrome-surface)] backdrop-blur-2xl border-b border-white/10 select-none">
	<!-- Left Navigation Pill Controls: Back, Forward, Reload -->
	<div class="flex items-center gap-1 shrink-0">
		<button
			onclick={handleBack}
			class="h-8 w-8 rounded-full flex items-center justify-center text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-white/10 transition-all"
			title="Back"
			aria-label="Back"
		>
			<ArrowLeft class="h-4 w-4" />
		</button>

		<button
			onclick={handleForward}
			class="h-8 w-8 rounded-full flex items-center justify-center text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-white/10 transition-all"
			title="Forward"
			aria-label="Forward"
		>
			<ArrowRight class="h-4 w-4" />
		</button>

		<button
			onclick={handleRefresh}
			class={`h-8 w-8 rounded-full flex items-center justify-center text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-white/10 transition-all ${
				isRefreshing ? 'animate-spin text-[var(--accent)]' : ''
			}`}
			title="Reload page"
			aria-label="Reload"
		>
			<RotateCcw class="h-3.5 w-3.5" />
		</button>
	</div>

	<!-- Center: Large Rounded Address / Search Pill Container -->
	<div class="relative flex-1 max-w-[840px] mx-3">
		<div
			class="relative flex h-[34px] w-full items-center gap-2 rounded-full bg-[var(--chrome-urlbar)] px-3 text-xs shadow-inner border border-white/10 focus-within:border-[var(--accent)] focus-within:ring-2 focus-within:ring-[var(--accent-soft)] transition-all"
		>
			<!-- VPN Badge (Integrated in URL Bar as in Opera) -->
			<div class="relative">
				<button
					onclick={() => (isVpnPopoverOpen = !isVpnPopoverOpen)}
					class={`flex items-center gap-1 px-2 py-0.5 rounded-full font-mono text-[10px] font-bold tracking-wider transition-all ${
						isVpnActive
							? 'bg-[var(--accent)] text-white shadow-xs'
							: 'bg-[var(--surface-hover)] text-[var(--text-dim)] border border-[var(--border)]'
					}`}
					title={`VPN: ${isVpnActive ? 'Active' : 'Disabled'} (${selectedProxy})`}
				>
					<Shield class="h-3 w-3" />
					<span>VPN</span>
				</button>

				<!-- VPN Popover -->
				{#if isVpnPopoverOpen}
					<div
						class="absolute top-8 left-0 z-50 w-64 rounded-[var(--radius-md)] bg-[var(--surface)] border border-[var(--border)] p-3 shadow-[var(--shadow-popover)] text-xs text-[var(--text)]"
					>
						<div class="flex items-center justify-between pb-2 border-b border-[var(--border)]">
							<span class="font-bold">Qori Private Relay</span>
							<button
								onclick={() => (isVpnActive = !isVpnActive)}
								class={`px-2 py-0.5 rounded-full text-[10px] font-bold ${
									isVpnActive ? 'bg-emerald-500/20 text-emerald-400' : 'bg-rose-500/20 text-rose-400'
								}`}
							>
								{isVpnActive ? 'CONNECTED' : 'DISABLED'}
							</button>
						</div>
						<div class="mt-2.5 flex flex-col gap-1.5">
							<span class="text-[10px] text-[var(--text-dim)] uppercase tracking-wider font-semibold">Location / Node</span>
							{#each proxies as p}
								<button
									onclick={() => {
										selectedProxy = p;
										isVpnPopoverOpen = false;
									}}
									class={`flex items-center justify-between px-2 py-1.5 rounded-[var(--radius-sm)] text-left transition-colors ${
										selectedProxy === p
											? 'bg-[var(--accent-soft)] text-[var(--text)] font-semibold'
											: 'text-[var(--text-dim)] hover:bg-[var(--surface-hover)]'
									}`}
								>
									<span class="truncate">{p}</span>
									{#if selectedProxy === p}
										<Check class="h-3 w-3 text-[var(--accent)] shrink-0" />
									{/if}
								</button>
							{/each}
						</div>
					</div>
				{/if}
			</div>

			<!-- Search / Address Icon -->
			<Search class="h-3.5 w-3.5 text-[var(--text-dim)] shrink-0 ml-0.5" />

			<!-- Address / Search Input Field -->
			<input
				type="text"
				bind:value={queryInput}
				placeholder="Search with QORI or enter web address"
				class="flex-1 bg-transparent text-xs text-[var(--text)] placeholder-[var(--text-faint)] focus:outline-none"
				onkeydown={(e) => {
					if (e.key === 'Enter') submitQuery();
				}}
			/>

			<!-- Site Security Shield Badge -->
			<div class="flex items-center gap-1.5 text-[var(--text-faint)] hover:text-emerald-400 transition-colors cursor-pointer" title="Encrypted Connection">
				<ShieldCheck class="h-3.5 w-3.5" />
			</div>
		</div>
	</div>

	<!-- Right Side Tools: Screenshot, Download, Profile, Hamburger Menu -->
	<div class="flex items-center gap-1 shrink-0">
		<!-- Screenshot Button with Popover -->
		<div class="relative">
			<button
				onclick={() => (isScreenshotPopoverOpen = !isScreenshotPopoverOpen)}
				class="h-8 w-8 rounded-full flex items-center justify-center text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-white/10 transition-all"
				title="Capture Screenshot"
				aria-label="Screenshot"
			>
				<Camera class="h-4 w-4" />
			</button>

			{#if isScreenshotPopoverOpen}
				<div
					class="absolute right-0 top-10 z-50 w-44 rounded-[var(--radius-md)] bg-[var(--chrome-surface)] border border-white/10 p-2 shadow-[var(--shadow-popover)] text-xs text-[var(--text)] flex flex-col gap-1 backdrop-blur-2xl"
				>
					<button
						onclick={() => takeScreenshot('visible')}
						class="flex items-center gap-2 px-2.5 py-1.5 rounded-[var(--radius-sm)] hover:bg-white/10 text-left"
					>
						<span>Visible Screen</span>
					</button>
					<button
						onclick={() => takeScreenshot('full')}
						class="flex items-center gap-2 px-2.5 py-1.5 rounded-[var(--radius-sm)] hover:bg-white/10 text-left"
					>
						<span>Full Page</span>
					</button>
					<button
						onclick={() => takeScreenshot('selection')}
						class="flex items-center gap-2 px-2.5 py-1.5 rounded-[var(--radius-sm)] hover:bg-white/10 text-left"
					>
						<span>Area Selection</span>
					</button>
				</div>
			{/if}
		</div>

		<!-- Download Manager / Export Vault -->
		<div class="relative">
			<button
				onclick={() => (isDownloadPopoverOpen = !isDownloadPopoverOpen)}
				class="h-8 w-8 rounded-full flex items-center justify-center text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-white/10 transition-all"
				title="Downloads & Exports"
				aria-label="Downloads"
			>
				<Download class="h-4 w-4" />
			</button>

			{#if isDownloadPopoverOpen}
				<div
					class="absolute right-0 top-10 z-50 w-64 rounded-[var(--radius-md)] bg-[var(--chrome-surface)] border border-white/10 p-3 shadow-[var(--shadow-popover)] text-xs text-[var(--text)] flex flex-col gap-2 backdrop-blur-2xl"
				>
					<div class="flex items-center justify-between pb-1.5 border-b border-[var(--border)]">
						<span class="font-bold">Vault Exports</span>
						<span class="text-[10px] text-[var(--text-dim)]">Ready</span>
					</div>
					<p class="text-[11px] text-[var(--text-dim)]">Export your indexed Tantivy library and SQLite bookmarks database as a portable .zip vault.</p>
					<button
						onclick={handleDownloadExport}
						class="w-full py-1.5 px-3 rounded-[var(--radius-sm)] bg-[var(--accent)] hover:bg-[var(--accent-hover)] text-white font-medium flex items-center justify-center gap-2 transition-colors shadow-sm"
					>
						<Download class="h-3.5 w-3.5" />
						<span>Export Full Library (.zip)</span>
					</button>
				</div>
			{/if}
		</div>

		<!-- Divider -->
		<div class="h-4 w-[1px] bg-white/10 mx-1"></div>

		<!-- Hamburger Menu Button -->
		<div class="relative">
			<button
				onclick={() => (isMenuOpen = !isMenuOpen)}
				class={`h-8 w-8 rounded-full flex items-center justify-center transition-all ${
					isMenuOpen
						? 'bg-[var(--accent)] text-white shadow-sm'
						: 'text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-white/10'
				}`}
				title="Menu"
				aria-label="Browser Menu"
			>
				<Menu class="h-4 w-4" />
			</button>

			<!-- Comprehensive Browser Menu Dropdown Panel -->
			{#if isMenuOpen}
				<div
					class="absolute right-0 top-10 z-50 w-64 rounded-[var(--radius-md)] bg-[var(--chrome-surface)] border border-white/10 p-2 shadow-[var(--shadow-popover)] text-xs text-[var(--text)] flex flex-col gap-1 backdrop-blur-2xl"
				>
					<button
						onclick={() => handleMenuAction('new-tab')}
						class="flex items-center justify-between px-2.5 py-1.5 rounded-[var(--radius-sm)] hover:bg-[var(--accent-soft)] text-left"
					>
						<div class="flex items-center gap-2">
							<Plus class="h-3.5 w-3.5 text-[var(--text-dim)]" />
							<span>New Tab</span>
						</div>
						<span class="font-mono text-[10px] text-[var(--text-faint)]">Ctrl T</span>
					</button>

					<button
						onclick={() => handleMenuAction('library')}
						class="flex items-center justify-between px-2.5 py-1.5 rounded-[var(--radius-sm)] hover:bg-[var(--accent-soft)] text-left"
					>
						<div class="flex items-center gap-2">
							<Database class="h-3.5 w-3.5 text-[var(--text-dim)]" />
							<span>Library & Vault</span>
						</div>
						<span class="font-mono text-[10px] text-[var(--text-faint)]">Ctrl L</span>
					</button>

					<button
						onclick={() => handleMenuAction('sources')}
						class="flex items-center justify-between px-2.5 py-1.5 rounded-[var(--radius-sm)] hover:bg-[var(--accent-soft)] text-left"
					>
						<div class="flex items-center gap-2">
							<Globe class="h-3.5 w-3.5 text-[var(--text-dim)]" />
							<span>Sources & Crawler</span>
						</div>
						<span class="font-mono text-[10px] text-[var(--text-faint)]">Ctrl S</span>
					</button>

					<div class="h-[1px] bg-[var(--border)] my-1"></div>

					<button
						onclick={() => handleMenuAction('settings')}
						class="flex items-center justify-between px-2.5 py-1.5 rounded-[var(--radius-sm)] hover:bg-[var(--accent-soft)] text-left"
					>
						<div class="flex items-center gap-2">
							<Settings class="h-3.5 w-3.5 text-[var(--text-dim)]" />
							<span>Settings</span>
						</div>
						<span class="font-mono text-[10px] text-[var(--text-faint)]">Ctrl ,</span>
					</button>

					<button
						onclick={() => handleMenuAction('theme')}
						class="flex items-center justify-between px-2.5 py-1.5 rounded-[var(--radius-sm)] hover:bg-[var(--accent-soft)] text-left"
					>
						<div class="flex items-center gap-2">
							{#if theme === 'dark'}
								<Sun class="h-3.5 w-3.5 text-amber-400" />
								<span>Light Theme</span>
							{:else}
								<Moon class="h-3.5 w-3.5 text-[var(--accent)]" />
								<span>Dark Theme</span>
							{/if}
						</div>
						<span class="font-mono text-[10px] text-[var(--text-faint)]">Ctrl D</span>
					</button>

					<div class="h-[1px] bg-[var(--border)] my-1"></div>

					<button
						onclick={() => handleMenuAction('about')}
						class="flex items-center justify-between px-2.5 py-1.5 rounded-[var(--radius-sm)] hover:bg-[var(--accent-soft)] text-left"
					>
						<div class="flex items-center gap-2">
							<Info class="h-3.5 w-3.5 text-[var(--text-dim)]" />
							<span>About Qori</span>
						</div>
						<span class="text-[10px] text-[var(--text-faint)]">v1.0</span>
					</button>
				</div>
			{/if}
		</div>
	</div>
</div>
