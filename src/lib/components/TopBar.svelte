<script lang="ts">
	import {
		Plus,
		X,
		Search,
		Minus,
		Square,
		Compass,
		Globe,
		Database,
		Settings as SettingsIcon,
		Layers
	} from 'lucide-svelte';
	import type { BrowserTab } from '../types';

	let {
		tabs = [],
		activeTabId = '',
		onSelectTab = (id: string) => {},
		onCloseTab = (id: string) => {},
		onNewTab = () => {},
		onOpenCommandPalette = () => {}
	}: {
		tabs: BrowserTab[];
		activeTabId: string;
		onSelectTab: (id: string) => void;
		onCloseTab: (id: string) => void;
		onNewTab: () => void;
		onOpenCommandPalette: () => void;
	} = $props();

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

<header
	data-tauri-drag-region
	class="relative z-30 flex h-[44px] w-full items-center justify-between px-3 border-b border-[var(--border)] glass-chrome select-none"
>
	<!-- Left Side: Rounded Pill Tabs + '+' Button -->
	<div data-tauri-drag-region class="flex items-center gap-1.5 flex-1 overflow-x-auto py-1 pr-4">
		{#each tabs as tab (tab.id)}
			{@const isActive = tab.id === activeTabId}
			<div
				class={`group relative flex h-[32px] max-w-[210px] min-w-[130px] items-center gap-2 rounded-full px-3 text-xs font-medium cursor-pointer transition-all duration-150 select-none ${
					isActive
						? 'bg-[var(--surface)] text-[var(--text)] shadow-sm font-semibold border border-[var(--border)]'
						: 'text-[var(--text-dim)] hover:bg-[var(--accent-soft)] hover:text-[var(--text)]'
				}`}
				onclick={() => onSelectTab(tab.id)}
				role="button"
				tabindex="0"
				onkeydown={(e) => {
					if (e.key === 'Enter') onSelectTab(tab.id);
				}}
				title={tab.title || 'New Tab'}
			>
				<!-- Favicon / Tab Icon -->
				<div class="shrink-0 flex items-center justify-center">
					{#if tab.page === 'settings'}
						<SettingsIcon class="h-3.5 w-3.5 text-[var(--accent)]" />
					{:else if tab.page === 'web'}
						<Globe class="h-3.5 w-3.5 text-blue-400" />
					{:else if tab.page === 'results'}
						<Search class="h-3.5 w-3.5 text-[var(--accent)]" />
					{:else}
						<Compass class="h-3.5 w-3.5 text-[var(--text-dim)]" />
					{/if}
				</div>

				<!-- Tab Title -->
				<span class="truncate flex-1 text-left text-xs">
					{tab.title || 'Start Page'}
				</span>

				<!-- Close Button (Pill style on hover) -->
				<button
					onclick={(e) => {
						e.stopPropagation();
						onCloseTab(tab.id);
					}}
					class="opacity-0 group-hover:opacity-100 h-4 w-4 rounded-full flex items-center justify-center text-[var(--text-faint)] hover:text-[var(--text)] hover:bg-[var(--accent-soft)] transition-opacity"
					title="Close tab (Ctrl+W)"
					aria-label="Close tab"
				>
					<X class="h-3 w-3" />
				</button>
			</div>
		{/each}

		<!-- New Tab Button: Circular Pill -->
		<button
			onclick={onNewTab}
			class="h-[30px] w-[30px] shrink-0 rounded-full flex items-center justify-center text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--surface)] hover:shadow-sm border border-transparent hover:border-[var(--border)] transition-all"
			title="Open new tab (Ctrl+T)"
			aria-label="New Tab"
		>
			<Plus class="h-4 w-4" />
		</button>
	</div>

	<!-- Right Side: Command Search Button + Window Controls -->
	<div class="flex items-center gap-1 shrink-0">
		<!-- Command Search Palette Button -->
		<button
			onclick={onOpenCommandPalette}
			class="flex items-center gap-2 h-7 px-2.5 rounded-full bg-[var(--surface)]/70 hover:bg-[var(--surface)] border border-[var(--border)] text-xs text-[var(--text-dim)] hover:text-[var(--text)] transition-colors shadow-sm mr-2"
			title="Search commands (Ctrl+K)"
		>
			<Search class="h-3.5 w-3.5" />
			<span class="font-mono text-[10px] text-[var(--text-faint)] bg-[var(--accent-soft)] px-1.5 py-0.5 rounded-full font-medium">Ctrl K</span>
		</button>

		<!-- Window Controls: Soft styled minimize, maximize, close -->
		<div class="flex items-center gap-0.5 ml-1">
			<button
				onclick={minimizeWindow}
				class="h-7 w-8 rounded-[var(--radius-sm)] flex items-center justify-center text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--accent-soft)] transition-colors"
				title="Minimize"
				aria-label="Minimize"
			>
				<Minus class="h-3.5 w-3.5" />
			</button>
			<button
				onclick={toggleMaximize}
				class="h-7 w-8 rounded-[var(--radius-sm)] flex items-center justify-center text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--accent-soft)] transition-colors"
				title="Maximize"
				aria-label="Maximize"
			>
				<Square class="h-3 w-3" />
			</button>
			<button
				onclick={closeWindow}
				class="h-7 w-8 rounded-[var(--radius-sm)] flex items-center justify-center text-[var(--text-dim)] hover:text-white hover:bg-rose-500/80 transition-colors"
				title="Close"
				aria-label="Close"
			>
				<X class="h-3.5 w-3.5" />
			</button>
		</div>
	</div>
</header>

