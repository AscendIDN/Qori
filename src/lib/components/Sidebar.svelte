<script lang="ts">
	import { onMount } from 'svelte';
	import {
		Compass,
		Star,
		Heart,
		Clock,
		Database,
		Globe,
		Settings,
		MoreHorizontal,
		Plus,
		ChevronLeft,
		ChevronRight,
		X,
		Cpu,
		Layers
	} from 'lucide-svelte';
	import type { BrowserTab } from '../types';

	let {
		tabs = [],
		activeTabId = '',
		activePage = 'search',
		onSelectPage = (page: 'search' | 'library' | 'sources' | 'settings') => {},
		onSelectTab = (id: string) => {},
		onCloseTab = (id: string) => {},
		onNewTab = () => {},
		onOpenUrl = (url: string, title: string) => {}
	}: {
		tabs: BrowserTab[];
		activeTabId: string;
		activePage: 'search' | 'results' | 'settings' | 'web';
		onSelectPage: (page: 'search' | 'library' | 'sources' | 'settings') => void;
		onSelectTab: (id: string) => void;
		onCloseTab: (id: string) => void;
		onNewTab: () => void;
		onOpenUrl: (url: string, title: string) => void;
	} = $props();

	let isExpanded = $state(false);
	let activeWorkspace = $state(0);
	const workspaces = [
		{ name: 'Personal', color: 'bg-indigo-500' },
		{ name: 'Research', color: 'bg-emerald-500' },
		{ name: 'Work', color: 'bg-rose-500' }
	];

	// Pinned Web Apps
	const pinnedApps = [
		{ name: 'Reddit', url: 'https://reddit.com', icon: '🔴', bg: '#FF4500' },
		{ name: 'Twitch', url: 'https://twitch.tv', icon: '🟣', bg: '#9146FF' },
		{ name: 'X', url: 'https://x.com', icon: '⚫', bg: '#000000' }
	];

	function toggleSidebar() {
		isExpanded = !isExpanded;
	}

	onMount(() => {
		function handleKeydown(e: KeyboardEvent) {
			if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'b') {
				e.preventDefault();
				toggleSidebar();
			}
		}
		window.addEventListener('keydown', handleKeydown);
		return () => window.removeEventListener('keydown', handleKeydown);
	});
</script>

<aside
	class={`relative z-30 flex flex-col justify-between py-3 transition-all duration-200 ease-out glass-chrome border-r border-[var(--border)] select-none shrink-0 ${
		isExpanded ? 'w-[264px] px-3' : 'w-[56px] items-center px-1.5'
	}`}
>
	<!-- Top Section: Logo & Primary Navigation -->
	<div class="flex flex-col gap-2 w-full">
		<!-- Qori Logo Mark + Collapse Toggle Button -->
		<div class={`flex items-center ${isExpanded ? 'justify-between px-2' : 'justify-center'} h-9`}>
			<button
				onclick={() => onSelectPage('search')}
				class="flex items-center gap-2.5 rounded-[var(--radius-sm)] focus-visible:outline-none"
				title="Qori Home"
			>
				<div
					class="h-7 w-7 rounded-[var(--radius-sm)] bg-gradient-to-tr from-[var(--accent)] to-[#C084FC] flex items-center justify-center text-white font-bold text-xs shadow-sm shadow-[var(--accent)]/30"
				>
					Q
				</div>
				{#if isExpanded}
					<span class="font-extrabold tracking-tight text-sm text-[var(--text)]">QORI</span>
				{/if}
			</button>

			<button
				onclick={toggleSidebar}
				class="h-7 w-7 rounded-[var(--radius-sm)] flex items-center justify-center text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--accent-soft)] transition-colors"
				title={isExpanded ? 'Collapse sidebar (Ctrl+B)' : 'Expand sidebar (Ctrl+B)'}
				aria-label="Toggle Sidebar"
			>
				{#if isExpanded}
					<ChevronLeft class="h-4 w-4" />
				{:else}
					<ChevronRight class="h-4 w-4" />
				{/if}
			</button>
		</div>

		<!-- Divider -->
		<div class="h-[1px] bg-[var(--border)] my-1 w-full"></div>

		<!-- Main Nav Items (Start Page / Bookmarks) -->
		<div class="flex flex-col gap-1 w-full">
			<!-- Start Page (Home) -->
			<button
				onclick={() => onSelectPage('search')}
				class={`flex items-center gap-3 w-full rounded-[var(--radius-sm)] transition-all ${
					isExpanded ? 'px-3 py-2 text-xs font-semibold' : 'h-9 w-9 justify-center mx-auto'
				} ${
					activePage === 'search'
						? 'bg-[var(--accent)] text-white shadow-sm shadow-[var(--accent)]/25'
						: 'text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--accent-soft)]'
				}`}
				title="Start Page"
			>
				<Compass class="h-4 w-4 shrink-0" />
				{#if isExpanded}
					<span class="truncate">Start Page</span>
				{/if}
			</button>

			<!-- Bookmarks -->
			<button
				onclick={() => onSelectPage('library')}
				class={`flex items-center gap-3 w-full rounded-[var(--radius-sm)] transition-all ${
					isExpanded ? 'px-3 py-2 text-xs font-medium' : 'h-9 w-9 justify-center mx-auto'
				} text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--accent-soft)]`}
				title="Bookmarks"
			>
				<Star class="h-4 w-4 shrink-0" />
				{#if isExpanded}
					<span class="truncate">Bookmarks</span>
				{/if}
			</button>
		</div>

		<!-- Workspaces Stack -->
		<div class="flex flex-col gap-1.5 w-full my-1">
			{#if isExpanded}
				<div class="px-2 pt-1 flex items-center justify-between text-[10px] uppercase font-bold tracking-wider text-[var(--text-faint)]">
					<span>Workspaces</span>
					<button class="hover:text-[var(--text)] transition-colors" title="Add workspace">
						<Plus class="h-3.5 w-3.5" />
					</button>
				</div>
			{/if}

			<div class={`flex ${isExpanded ? 'flex-col gap-1' : 'flex-col items-center gap-1.5'}`}>
				{#each workspaces as ws, idx}
					<button
						onclick={() => (activeWorkspace = idx)}
						class={`flex items-center gap-2.5 rounded-[var(--radius-sm)] transition-all ${
							isExpanded ? 'px-2.5 py-1.5 text-xs' : 'p-1'
						} ${
							activeWorkspace === idx
								? isExpanded
									? 'bg-[var(--accent-soft)] text-[var(--text)] font-semibold'
									: 'ring-2 ring-[var(--accent)] ring-offset-2 ring-offset-[var(--surface)]'
								: 'text-[var(--text-dim)] hover:bg-[var(--accent-soft)]'
						}`}
						title={ws.name}
					>
						<div class={`h-3.5 w-3.5 rounded-full ${ws.color} shrink-0`}></div>
						{#if isExpanded}
							<span class="truncate">{ws.name}</span>
						{/if}
					</button>
				{/each}
			</div>
		</div>

		<!-- Pinned Shortcuts -->
		<div class="flex flex-col gap-1 w-full my-1">
			{#if isExpanded}
				<div class="px-2 pt-1 text-[10px] uppercase font-bold tracking-wider text-[var(--text-faint)]">
					<span>Pinned Apps</span>
				</div>
			{/if}

			<div class={`flex ${isExpanded ? 'flex-col gap-1' : 'flex-col items-center gap-1.5'}`}>
				{#each pinnedApps as app}
					<button
						onclick={() => onOpenUrl(app.url, app.name)}
						class={`flex items-center gap-2.5 rounded-[var(--radius-sm)] transition-all ${
							isExpanded
								? 'px-2.5 py-1.5 text-xs text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--accent-soft)]'
								: 'h-8 w-8 justify-center rounded-[var(--radius-sm)] bg-[var(--surface-hover)] hover:scale-105 border border-[var(--border)]'
						}`}
						title={`Open ${app.name}`}
					>
						<span class="text-xs">{app.icon}</span>
						{#if isExpanded}
							<span class="truncate">{app.name}</span>
						{/if}
					</button>
				{/each}
			</div>
		</div>

		<!-- Expanded Mode: Vertical Tabs list preview -->
		{#if isExpanded}
			<div class="flex flex-col gap-1 w-full my-2 border-t border-[var(--border)] pt-2 max-h-[220px] overflow-y-auto">
				<div class="px-2 pb-1 flex items-center justify-between text-[10px] uppercase font-bold tracking-wider text-[var(--text-faint)]">
					<span>Tabs ({tabs.length})</span>
					<button onclick={onNewTab} class="hover:text-[var(--text)] transition-colors" title="New Tab">
						<Plus class="h-3.5 w-3.5" />
					</button>
				</div>
				{#each tabs as tab}
					{@const isActive = tab.id === activeTabId}
					<div
						class={`group flex items-center justify-between px-2.5 py-1.5 rounded-[var(--radius-sm)] text-xs cursor-pointer transition-all ${
							isActive
								? 'bg-[var(--surface)] text-[var(--text)] font-semibold shadow-sm'
								: 'text-[var(--text-dim)] hover:bg-[var(--accent-soft)] hover:text-[var(--text)]'
						}`}
						onclick={() => onSelectTab(tab.id)}
						role="button"
						tabindex="0"
						onkeydown={(e) => {
							if (e.key === 'Enter') onSelectTab(tab.id);
						}}
					>
						<span class="truncate pr-1">{tab.title || 'New Tab'}</span>
						<button
							onclick={(e) => {
								e.stopPropagation();
								onCloseTab(tab.id);
							}}
							class="opacity-0 group-hover:opacity-100 p-0.5 rounded hover:bg-[var(--accent-soft)] text-[var(--text-dim)] hover:text-[var(--text)]"
							title="Close tab"
						>
							<X class="h-3 w-3" />
						</button>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<!-- Bottom Section: Library, Sources, Settings, etc. -->
	<div class="flex flex-col gap-1 w-full border-t border-[var(--border)] pt-2">
		<!-- Library (Qori Index) -->
		<button
			onclick={() => onSelectPage('library')}
			class={`flex items-center gap-3 w-full rounded-[var(--radius-sm)] transition-all ${
				isExpanded ? 'px-3 py-2 text-xs' : 'h-9 w-9 justify-center mx-auto'
			} ${
				activePage === 'settings'
					? 'bg-[var(--accent-soft)] text-[var(--text)] font-semibold'
					: 'text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--accent-soft)]'
			}`}
			title="Library (Indexed Documents)"
		>
			<Database class="h-4 w-4 shrink-0" />
			{#if isExpanded}
				<span class="truncate">Library</span>
			{/if}
		</button>

		<!-- Sources (Crawler) -->
		<button
			onclick={() => onSelectPage('sources')}
			class={`flex items-center gap-3 w-full rounded-[var(--radius-sm)] transition-all ${
				isExpanded ? 'px-3 py-2 text-xs' : 'h-9 w-9 justify-center mx-auto'
			} text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--accent-soft)]`}
			title="Sources & Crawler"
		>
			<Globe class="h-4 w-4 shrink-0" />
			{#if isExpanded}
				<span class="truncate">Sources</span>
			{/if}
		</button>

		<!-- Settings -->
		<button
			onclick={() => onSelectPage('settings')}
			class={`flex items-center gap-3 w-full rounded-[var(--radius-sm)] transition-all ${
				isExpanded ? 'px-3 py-2 text-xs' : 'h-9 w-9 justify-center mx-auto'
			} text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--accent-soft)]`}
			title="Settings & Privacy"
		>
			<Settings class="h-4 w-4 shrink-0" />
			{#if isExpanded}
				<span class="truncate">Settings</span>
			{/if}
		</button>

		{#if isExpanded}
			<!-- Memory Saver / Resource Meter Footer -->
			<div class="mt-2 pt-2 border-t border-[var(--border)] px-2 flex items-center justify-between text-[11px] text-[var(--text-faint)]">
				<div class="flex items-center gap-1.5">
					<Cpu class="h-3 w-3 text-emerald-400" />
					<span>128 MB RAM</span>
				</div>
				<span class="text-emerald-400 font-semibold text-[10px]">Optimal</span>
			</div>
		{/if}
	</div>
</aside>

