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
	class="relative z-30 flex h-[42px] w-full items-end justify-between px-2 bg-[var(--chrome-tabstrip)] backdrop-blur-2xl border-b border-white/10 select-none overflow-y-hidden"
>
	<!-- Left Side: Chrome Tabs (Active Tab Merged with Navbar) + '+' Button -->
	<div
		data-tauri-drag-region
		class="flex items-end h-full gap-0 flex-1 overflow-x-auto overflow-y-hidden no-scrollbar pl-1 pr-3"
	>
		{#each tabs as tab, i (tab.id)}
			{@const isActive = tab.id === activeTabId}
			{@const nextActive = tabs[i + 1]?.id === activeTabId}

			<div
				class={`group relative flex items-center gap-2 px-3 text-xs cursor-pointer select-none transition-colors duration-150 ${
					isActive
						? 'z-20 h-[36px] max-w-[220px] min-w-[140px] text-[var(--text)] font-semibold rounded-t-[10px] bg-[var(--chrome-surface)] -mb-[1px]'
						: 'z-10 h-[32px] max-w-[210px] min-w-[130px] text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-white/[0.08] rounded-t-[8px] mb-[2px]'
				}`}
				onclick={() => onSelectTab(tab.id)}
				role="button"
				tabindex="0"
				onkeydown={(e) => {
					if (e.key === 'Enter') onSelectTab(tab.id);
				}}
				title={tab.title || 'New Tab'}
			>
				{#if isActive}
					<!-- Left Inverted Corner Fillet (seamless curve into navbar) -->
					<svg
						class="absolute -bottom-[1px] -left-[8px] w-[8px] h-[8px] text-[var(--chrome-surface)] pointer-events-none z-20"
						viewBox="0 0 8 8"
						fill="currentColor"
						aria-hidden="true"
					>
						<path d="M 0 8 A 8 8 0 0 0 8 0 L 8 8 Z" />
					</svg>

					<!-- Right Inverted Corner Fillet (seamless curve into navbar) -->
					<svg
						class="absolute -bottom-[1px] -right-[8px] w-[8px] h-[8px] text-[var(--chrome-surface)] pointer-events-none z-20"
						viewBox="0 0 8 8"
						fill="currentColor"
						aria-hidden="true"
					>
						<path d="M 0 0 A 8 8 0 0 0 8 8 L 0 8 Z" />
					</svg>

					<!-- High-DPI subpixel seam cover -->
					<div
						class="absolute -bottom-[1px] left-0 right-0 h-[1px] bg-[var(--chrome-surface)] pointer-events-none z-20"
						aria-hidden="true"
					></div>
				{/if}

				<!-- Favicon / Tab Icon -->
				<div class="shrink-0 flex items-center justify-center">
					{#if tab.page === 'settings'}
						<SettingsIcon class="h-3.5 w-3.5 text-[var(--accent)]" />
					{:else if tab.page === 'web'}
						<Globe class="h-3.5 w-3.5 text-blue-400" />
					{:else if tab.page === 'results'}
						<Search class="h-3.5 w-3.5 text-[var(--accent)]" />
					{:else}
						<Compass class="h-3.5 w-3.5 text-[var(--accent)]" />
					{/if}
				</div>

				<!-- Tab Title -->
				<span class="truncate flex-1 text-left text-xs font-medium">
					{tab.title || 'Start Page'}
				</span>

				<!-- Close Button (Chrome style round hover button) -->
				<button
					onclick={(e) => {
						e.stopPropagation();
						onCloseTab(tab.id);
					}}
					class={`h-4 w-4 rounded-full flex items-center justify-center text-[var(--text-faint)] hover:text-white hover:bg-white/20 transition-all ${
						isActive ? 'opacity-80 hover:opacity-100' : 'opacity-0 group-hover:opacity-100'
					}`}
					title="Close tab (Ctrl+W)"
					aria-label="Close tab"
				>
					<X class="h-3 w-3" />
				</button>
			</div>

			<!-- Divider between inactive tabs -->
			{#if !isActive && !nextActive && i < tabs.length - 1}
				<div class="w-[1px] h-3.5 bg-white/15 my-auto mb-2 shrink-0 -mx-[1px]" aria-hidden="true"></div>
			{/if}
		{/each}

		<!-- New Tab Button: Circular '+' button -->
		<button
			onclick={onNewTab}
			class="h-7 w-7 shrink-0 rounded-full flex items-center justify-center text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-white/10 transition-colors ml-1.5 mb-1.5"
			title="Open new tab (Ctrl+T)"
			aria-label="New Tab"
		>
			<Plus class="h-4 w-4" />
		</button>
	</div>

	<!-- Right Side: Window Controls -->
	<div class="flex items-center gap-1 shrink-0 h-full pb-1">
		<div class="flex items-center gap-0.5 ml-1">
			<button
				onclick={minimizeWindow}
				class="h-7 w-8 rounded-[var(--radius-sm)] flex items-center justify-center text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-white/10 transition-colors"
				title="Minimize"
				aria-label="Minimize"
			>
				<Minus class="h-3.5 w-3.5" />
			</button>
			<button
				onclick={toggleMaximize}
				class="h-7 w-8 rounded-[var(--radius-sm)] flex items-center justify-center text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-white/10 transition-colors"
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

