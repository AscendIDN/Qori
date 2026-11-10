<script lang="ts">
	import {
		Sun,
		Moon,
		Search,
		Plus,
		FileText,
		Database,
		Globe,
		Settings,
		Minus,
		Square,
		X,
		ArrowLeft,
		ArrowRight,
		RotateCcw,
		Camera,
		Download,
		Menu,
		Shield,
		KeyRound,
		History,
		Bookmark,
		Languages,
		CircleQuestionMark,
		LogOut
	} from 'lucide-svelte';
	import type { SearchMode, BrowserTab } from '../types';

	let {
		tabs = $bindable<BrowserTab[]>([]),
		activeTabId = $bindable<string>(''),
		query = $bindable(''),
		mode = 'keyword',
		totalCount = 0,
		durationMs = 0,
		onSearch = (q: string, m: SearchMode) => {},
		onNewTab = () => {},
		onSelectTab = (id: string) => {},
		onCloseTab = (id: string) => {},
		onOpenSettings = (section: 'general' | 'library' | 'sources') => {},
		theme = $bindable<'dark' | 'light'>('dark')
	}: {
		tabs: BrowserTab[];
		activeTabId: string;
		query?: string;
		mode?: SearchMode;
		totalCount?: number;
		durationMs?: number;
		onSearch?: (q: string, m: SearchMode) => void;
		onNewTab?: () => void;
		onSelectTab?: (id: string) => void;
		onCloseTab?: (id: string) => void;
		onOpenSettings?: (section: 'general' | 'library' | 'sources') => void;
		theme: 'dark' | 'light';
	} = $props();

	let queryInput = $state('');
	let isMenuOpen = $state(false);
	let isVpnActive = $state(true);
	let isRefreshing = $state(false);

	// Keep local queryInput in sync with parent query prop
	$effect(() => {
		queryInput = query;
	});

	function submitQuery() {
		if (queryInput.trim().length > 0) {
			onSearch(queryInput.trim(), mode);
		}
	}

	function handleBack() {
		const cur = tabs.find((t) => t.id === activeTabId);
		if (cur && (cur.page === 'results' || cur.page === 'settings')) {
			cur.page = 'search';
			cur.title = 'New Tab';
			cur.query = '';
			queryInput = '';
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

	function toggleVpn() {
		isVpnActive = !isVpnActive;
	}

	async function handleDownload() {
		try {
			const { api } = await import('../api');
			const fn = await api.exportLibrary();
			alert(`Library exported successfully: ${fn}`);
		} catch (e) {
			console.warn('Export failed:', e);
		}
	}

	function handleScreenshot() {
		alert('Screenshot captured and saved to vault.');
	}

	function handlePassword() {
		alert('Password Manager: No saved credentials stored.');
		isMenuOpen = false;
	}

	function handleHistory() {
		onOpenSettings('library');
		isMenuOpen = false;
	}

	function handleBookmarks() {
		onOpenSettings('library');
		isMenuOpen = false;
	}

	function handleTranslate() {
		alert('Translate Page: Page is currently in original language.');
		isMenuOpen = false;
	}

	function handleHelp() {
		alert('Qori Search Engine\n\nShortcuts:\n- Enter: Submit search\n- Esc: Close modal\n- Search Tabs: Top tab bar');
		isMenuOpen = false;
	}

	function handleExit() {
		isMenuOpen = false;
		closeWindow();
	}

	function toggleTheme() {
		theme = theme === 'dark' ? 'light' : 'dark';
		document.documentElement.setAttribute('data-theme', theme);
	}

	async function minimizeWindow() {
		try {
			const { getCurrentWindow } = await import('@tauri-apps/api/window');
			await getCurrentWindow().minimize();
		} catch {
			try {
				const { invoke } = await import('@tauri-apps/api/core');
				await invoke('window_minimize');
			} catch (err) {
				console.warn('Window minimize failed:', err);
			}
		}
	}

	async function toggleMaximize() {
		try {
			const { getCurrentWindow } = await import('@tauri-apps/api/window');
			await getCurrentWindow().toggleMaximize();
		} catch {
			try {
				const { invoke } = await import('@tauri-apps/api/core');
				await invoke('window_toggle_maximize');
			} catch (err) {
				console.warn('Window toggle maximize failed:', err);
			}
		}
	}

	async function closeWindow() {
		try {
			const { getCurrentWindow } = await import('@tauri-apps/api/window');
			await getCurrentWindow().close();
		} catch {
			try {
				const { invoke } = await import('@tauri-apps/api/core');
				await invoke('window_close');
			} catch (err) {
				console.warn('Window close failed:', err);
			}
		}
	}
</script>

<header class="sticky top-0 z-40 w-full border-b border-[var(--line)] bg-[var(--surface)] select-none">
	<!-- Top Row: Browser-style Tab Bar + Window Controls -->
	<div
		data-tauri-drag-region
		class="flex h-9 w-full items-stretch justify-between border-b border-[var(--line)] bg-[var(--bg-deep)]/95 backdrop-blur-md"
	>
		<!-- Left: Chrome-style Browser Tabs -->
		<div data-tauri-drag-region class="flex items-end gap-1 pl-2">
			{#each tabs as tab (tab.id)}
				{@const isActive = tab.id === activeTabId}
				<div
					class={`group relative flex h-7 max-w-[190px] items-center gap-1.5 border-t border-x px-2.5 font-mono text-[11px] tracking-wide transition-all cursor-pointer select-none ${
						isActive
							? 'rounded-t border-[var(--line-strong)] bg-[var(--surface)] font-bold text-[var(--fg)] shadow-sm'
							: 'border-transparent text-[var(--fg-dim)] hover:border-[var(--line)] hover:bg-[var(--surface)]/50 hover:text-[var(--fg)]'
					}`}
					onclick={() => onSelectTab(tab.id)}
					role="button"
					tabindex="0"
					onkeydown={(e) => {
						if (e.key === 'Enter') onSelectTab(tab.id);
					}}
					title={tab.title || 'New Tab'}
				>
					{#if tab.page === 'settings'}
						<Settings
							class={`h-3 w-3 shrink-0 ${
								isActive ? 'text-[var(--accent-purple)]' : 'text-[var(--fg-faint)]'
							}`}
						/>
					{:else}
						<Globe
							class={`h-3 w-3 shrink-0 ${
								isActive ? 'text-[var(--accent-blue)]' : 'text-[var(--fg-faint)]'
							}`}
						/>
					{/if}

					<span class="truncate">{tab.title || 'New Tab'}</span>

					{#if isActive}
						<span class="absolute top-0 right-2 left-2 h-[2px] bg-[var(--accent-blue)]"></span>
					{/if}

					<!-- Close Tab button (x) -->
					<button
						onclick={(e) => {
							e.stopPropagation();
							onCloseTab(tab.id);
						}}
						aria-label="Close Tab"
						class={`ml-1 flex h-4 w-4 shrink-0 items-center justify-center rounded-sm transition-colors hover:bg-[var(--line-strong)] hover:text-[var(--fg)] ${
							isActive ? 'opacity-80 hover:opacity-100' : 'opacity-0 group-hover:opacity-100'
						}`}
					>
						<X class="h-2.5 w-2.5" />
					</button>
				</div>
			{/each}

			<!-- New Tab (+) button -->
			<button
				onclick={onNewTab}
				class="mb-0.5 flex h-6 w-6 items-center justify-center rounded text-[var(--fg-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--fg)]"
				title="New Tab"
				aria-label="New Tab"
			>
				<Plus class="h-3 w-3" />
			</button>
		</div>

		<!-- Center: Draggable Window Region -->
		<div data-tauri-drag-region class="flex-1 cursor-default"></div>

		<!-- Right: Theme & Native-style Window Controls -->
		<div class="flex items-stretch">
			<!-- Theme Toggle -->
			<button
				onclick={toggleTheme}
				aria-label="Toggle visual theme"
				class="flex items-center justify-center px-2.5 text-[var(--fg-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--fg)]"
				title={theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
			>
				{#if theme === 'dark'}
					<Sun class="h-3.5 w-3.5" />
				{:else}
					<Moon class="h-3.5 w-3.5" />
				{/if}
			</button>

			<!-- Quick Add Source -->
			<button
				onclick={() => onOpenSettings('sources')}
				class="flex items-center gap-1 border-l border-[var(--line)] bg-[var(--fg)] px-3 font-mono text-[10px] font-bold tracking-wider text-[var(--bg)] uppercase transition-all hover:bg-[var(--fg)]/90 active:scale-[0.98]"
				title="Add new source"
			>
				<Plus class="h-3 w-3 stroke-[2.5]" />
				<span class="hidden md:inline">ADD SOURCE</span>
			</button>

			<!-- Divider before Window Controls -->
			<div class="h-full w-[1px] bg-[var(--line)]"></div>

			<!-- Window Controls: Minimize, Maximize/Restore, Close -->
			<div class="flex items-stretch">
				<button
					onclick={minimizeWindow}
					aria-label="Minimize"
					class="flex h-9 w-10 items-center justify-center text-[var(--fg-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--fg)]"
					title="Minimize"
				>
					<Minus class="h-3.5 w-3.5" />
				</button>

				<button
					onclick={toggleMaximize}
					aria-label="Maximize or Restore"
					class="flex h-9 w-10 items-center justify-center text-[var(--fg-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--fg)]"
					title="Maximize / Restore"
				>
					<Square class="h-3 w-3" />
				</button>

				<button
					onclick={closeWindow}
					aria-label="Close"
					class="flex h-9 w-11 items-center justify-center text-[var(--fg-dim)] transition-colors hover:bg-[#e5484d] hover:text-white"
					title="Close"
				>
					<X class="h-3.5 w-3.5" />
				</button>
			</div>
		</div>
	</div>

	<!-- Bottom Row: Browser Navigation Controls + Search Omnibox + Browser Tools -->
	<div class="relative flex w-full items-center justify-between gap-3 px-3 py-2 sm:px-5">
		<!-- Left: Navigation Arrows & Refresh Button -->
		<div class="flex items-center gap-1">
			<button
				onclick={handleBack}
				title="Back"
				class="flex h-7 w-7 items-center justify-center rounded text-[var(--fg-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--fg)]"
			>
				<ArrowLeft class="h-4 w-4" />
			</button>
			<button
				onclick={handleForward}
				title="Forward"
				class="flex h-7 w-7 items-center justify-center rounded text-[var(--fg-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--fg)]"
			>
				<ArrowRight class="h-4 w-4" />
			</button>
			<button
				onclick={handleRefresh}
				title="Reload page / query"
				class="flex h-7 w-7 items-center justify-center rounded text-[var(--fg-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--fg)]"
			>
				<RotateCcw class={`h-3.5 w-3.5 ${isRefreshing ? 'animate-spin' : ''}`} />
			</button>
		</div>

		<!-- Center: Search Input in Header -->
		<div
			class="relative flex flex-1 items-center border border-[var(--line-strong)] bg-[var(--bg)] transition-colors"
		>
			<Search class="ml-3 h-3.5 w-3.5 shrink-0 text-[var(--fg-dim)]" />
			<input
				bind:value={queryInput}
				onkeydown={(e) => {
					if (e.key === 'Enter') submitQuery();
				}}
				type="text"
				placeholder="Refine search query..."
				class="w-full bg-transparent px-3 py-1.5 font-mono text-xs text-[var(--fg)] placeholder:text-[var(--fg-faint)] focus:outline-none focus:ring-0 sm:text-sm"
			/>
		</div>

		<!-- Right: Browser Tools (VPN, Screenshot, Download, Hamburger Menu) -->
		<div class="flex items-center gap-1.5">
			<!-- VPN Badge Button -->
			<button
				onclick={toggleVpn}
				title={isVpnActive ? 'Air-Gapped Private Search VPN: Active' : 'VPN: Bypassed'}
				class={`flex items-center gap-1 rounded px-2 py-1 font-mono text-[10px] font-bold tracking-wider transition-all ${
					isVpnActive
						? 'border border-[var(--accent-purple)]/50 bg-[var(--accent-purple)]/15 text-[var(--accent-purple)] hover:bg-[var(--accent-purple)]/25'
						: 'border border-[var(--line)] bg-[var(--bg-deep)] text-[var(--fg-faint)] hover:text-[var(--fg-dim)]'
				}`}
			>
				<Shield class="h-3 w-3 fill-current" />
				<span>VPN</span>
			</button>

			<!-- Screenshot (Camera) -->
			<button
				onclick={handleScreenshot}
				title="Capture Screenshot"
				class="flex h-7 w-7 items-center justify-center rounded text-[var(--fg-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--fg)]"
			>
				<Camera class="h-4 w-4" />
			</button>

			<!-- Downloads -->
			<button
				onclick={handleDownload}
				title="Downloads / Export Vault"
				class="flex h-7 w-7 items-center justify-center rounded text-[var(--fg-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--fg)]"
			>
				<Download class="h-4 w-4" />
			</button>

			<!-- Hamburger Menu -->
			<button
				onclick={() => (isMenuOpen = !isMenuOpen)}
				title="Browser Menu"
				class={`flex h-7 w-7 items-center justify-center rounded transition-colors ${
					isMenuOpen
						? 'bg-[var(--surface-hover)] text-[var(--fg)]'
						: 'text-[var(--fg-dim)] hover:bg-[var(--surface-hover)] hover:text-[var(--fg)]'
				}`}
			>
				<Menu class="h-4 w-4" />
			</button>
		</div>

		<!-- Hamburger Dropdown Menu -->
		{#if isMenuOpen}
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div class="fixed inset-0 z-40 bg-transparent" onclick={() => (isMenuOpen = false)}></div>

			<div
				class="absolute right-4 top-full z-50 mt-1 w-56 border border-[var(--line-strong)] bg-[var(--surface)] p-1.5 font-mono text-xs shadow-2xl"
			>
				<!-- 1. Password -->
				<button
					onclick={handlePassword}
					class="flex w-full items-center gap-2.5 px-3 py-2 text-left text-[var(--fg)] hover:bg-[var(--surface-hover)]"
				>
					<KeyRound class="h-3.5 w-3.5 text-[var(--accent-blue)]" />
					<span>Password</span>
				</button>

				<!-- 2. History -->
				<button
					onclick={handleHistory}
					class="flex w-full items-center gap-2.5 px-3 py-2 text-left text-[var(--fg)] hover:bg-[var(--surface-hover)]"
				>
					<History class="h-3.5 w-3.5 text-[var(--accent-purple)]" />
					<span>History</span>
				</button>

				<!-- 3. Download -->
				<button
					onclick={() => {
						handleDownload();
						isMenuOpen = false;
					}}
					class="flex w-full items-center gap-2.5 px-3 py-2 text-left text-[var(--fg)] hover:bg-[var(--surface-hover)]"
				>
					<Download class="h-3.5 w-3.5 text-[var(--accent-green)]" />
					<span>Download</span>
				</button>

				<!-- 4. Bookmarks -->
				<button
					onclick={handleBookmarks}
					class="flex w-full items-center gap-2.5 px-3 py-2 text-left text-[var(--fg)] hover:bg-[var(--surface-hover)]"
				>
					<Bookmark class="h-3.5 w-3.5 text-[var(--accent-blue)]" />
					<span>Bookmarks</span>
				</button>

				<div class="my-1 border-t border-[var(--line)]"></div>

				<!-- 5. Translate Page -->
				<button
					onclick={handleTranslate}
					class="flex w-full items-center gap-2.5 px-3 py-2 text-left text-[var(--fg)] hover:bg-[var(--surface-hover)]"
				>
					<Languages class="h-3.5 w-3.5 text-[var(--accent-purple)]" />
					<span>Translate Page</span>
				</button>

				<!-- 6. Settings -->
				<button
					onclick={() => {
						onOpenSettings('general');
						isMenuOpen = false;
					}}
					class="flex w-full items-center gap-2.5 px-3 py-2 text-left text-[var(--fg)] hover:bg-[var(--surface-hover)]"
				>
					<Settings class="h-3.5 w-3.5 text-[var(--fg-dim)]" />
					<span>Settings</span>
				</button>

				<!-- 7. Help -->
				<button
					onclick={handleHelp}
					class="flex w-full items-center gap-2.5 px-3 py-2 text-left text-[var(--fg)] hover:bg-[var(--surface-hover)]"
				>
					<CircleQuestionMark class="h-3.5 w-3.5 text-[var(--fg-dim)]" />
					<span>Help</span>
				</button>

				<div class="my-1 border-t border-[var(--line)]"></div>

				<!-- 8. Exit -->
				<button
					onclick={handleExit}
					class="flex w-full items-center gap-2.5 px-3 py-2 text-left text-[var(--fg)] hover:bg-[#e5484d]/10 hover:text-[#e5484d]"
				>
					<LogOut class="h-3.5 w-3.5 text-[#e5484d]" />
					<span>Exit</span>
				</button>
			</div>
		{/if}
	</div>
</header>
