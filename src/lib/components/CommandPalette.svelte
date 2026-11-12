<script lang="ts">
	import { onMount } from 'svelte';
	import {
		Search,
		Compass,
		Database,
		Globe,
		Settings,
		Plus,
		Download,
		Sun,
		Moon,
		Shield,
		ArrowRight,
		FileText
	} from 'lucide-svelte';

	let {
		isOpen = $bindable(false),
		onNavigate = (screen: 'search' | 'library' | 'sources' | 'settings') => {},
		onToggleTheme = () => {},
		onAddSource = () => {},
		onExport = () => {},
		onSearch = (query: string) => {},
		theme = 'dark'
	}: {
		isOpen: boolean;
		onNavigate: (screen: 'search' | 'library' | 'sources' | 'settings') => void;
		onToggleTheme: () => void;
		onAddSource: () => void;
		onExport: () => void;
		onSearch: (query: string) => void;
		theme: 'dark' | 'light';
	} = $props();

	let filterText = $state('');
	let selectedIndex = $state(0);
	let inputEl = $state<HTMLInputElement | null>(null);

	interface CommandItem {
		id: string;
		title: string;
		category: string;
		shortcut?: string;
		icon: any;
		action: () => void;
	}

	let allCommands = $derived.by<CommandItem[]>(() => [
		{
			id: 'nav-search',
			title: 'Navigate to Start Page',
			category: 'Navigation',
			shortcut: 'G S',
			icon: Compass,
			action: () => onNavigate('search')
		},
		{
			id: 'nav-library',
			title: 'Open Library & Vault',
			category: 'Navigation',
			shortcut: 'G L',
			icon: Database,
			action: () => onNavigate('library')
		},
		{
			id: 'nav-sources',
			title: 'Open Sources & Crawler',
			category: 'Navigation',
			shortcut: 'G C',
			icon: Globe,
			action: () => onNavigate('sources')
		},
		{
			id: 'nav-settings',
			title: 'Open Settings & Privacy',
			category: 'Navigation',
			shortcut: 'G P',
			icon: Settings,
			action: () => onNavigate('settings')
		},
		{
			id: 'act-add',
			title: 'Add Document or Source to Vault',
			category: 'Actions',
			shortcut: 'Ctrl N',
			icon: Plus,
			action: () => onAddSource()
		},
		{
			id: 'act-export',
			title: 'Export Library & Database (.zip)',
			category: 'Actions',
			shortcut: 'Ctrl E',
			icon: Download,
			action: () => onExport()
		},
		{
			id: 'act-theme',
			title: 'Toggle Visual Mode (Light / Dark)',
			category: 'System',
			shortcut: 'Ctrl D',
			icon: theme === 'dark' ? Sun : Moon,
			action: () => onToggleTheme()
		}
	]);

	let filteredCommands = $derived.by(() => {
		const q = filterText.trim().toLowerCase();
		if (!q) return allCommands;
		return allCommands.filter(
			(c) => c.title.toLowerCase().includes(q) || c.category.toLowerCase().includes(q)
		);
	});

	$effect(() => {
		if (isOpen) {
			filterText = '';
			selectedIndex = 0;
			setTimeout(() => inputEl?.focus(), 50);
		}
	});

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'ArrowDown') {
			e.preventDefault();
			selectedIndex = (selectedIndex + 1) % (filteredCommands.length || 1);
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			selectedIndex =
				(selectedIndex - 1 + (filteredCommands.length || 1)) % (filteredCommands.length || 1);
		} else if (e.key === 'Enter') {
			e.preventDefault();
			if (filteredCommands[selectedIndex]) {
				filteredCommands[selectedIndex].action();
				isOpen = false;
			} else if (filterText.trim()) {
				onSearch(filterText.trim());
				isOpen = false;
			}
		} else if (e.key === 'Escape') {
			isOpen = false;
		}
	}

	onMount(() => {
		function handleGlobalKey(e: KeyboardEvent) {
			if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
				e.preventDefault();
				isOpen = !isOpen;
			}
		}
		window.addEventListener('keydown', handleGlobalKey);
		return () => window.removeEventListener('keydown', handleGlobalKey);
	});
</script>

{#if isOpen}
	<div
		class="fixed inset-0 z-50 flex items-start justify-center pt-24 bg-black/45 backdrop-blur-md p-4 transition-all"
		onclick={() => (isOpen = false)}
		role="presentation"
	>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div
			class="relative w-full max-w-[640px] rounded-[var(--radius-lg)] bg-[var(--surface)] border border-[var(--border)] shadow-[var(--shadow-popover)] overflow-hidden text-[var(--text)]"
			onclick={(e) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<!-- Large Input Header -->
			<div class="flex items-center gap-3 px-4 py-3.5 border-b border-[var(--border)] bg-[var(--surface-hover)]">
				<Search class="h-4 w-4 text-[var(--text-dim)] shrink-0" />
				<input
					bind:this={inputEl}
					type="text"
					bind:value={filterText}
					onkeydown={handleKeydown}
					placeholder="Type a command or jump to screen..."
					class="flex-1 bg-transparent text-sm font-medium text-[var(--text)] placeholder-[var(--text-faint)] focus:outline-none"
				/>
				<span class="font-mono text-[10px] text-[var(--text-faint)] bg-[var(--border)] px-1.5 py-0.5 rounded">ESC</span>
			</div>

			<!-- Commands List -->
			<div class="max-h-[340px] overflow-y-auto p-2 flex flex-col gap-1">
				{#each filteredCommands as cmd, i}
					{@const isSelected = selectedIndex === i}
					{@const IconComponent = cmd.icon}
					<button
						onclick={() => {
							cmd.action();
							isOpen = false;
						}}
						class={`flex items-center justify-between px-3 py-2.5 rounded-[var(--radius-sm)] text-xs text-left transition-all cursor-pointer ${
							isSelected
								? 'bg-[var(--accent-soft)] text-[var(--text)] font-semibold'
								: 'text-[var(--text-dim)] hover:bg-[var(--surface-hover)] hover:text-[var(--text)]'
						}`}
					>
						<div class="flex items-center gap-3">
							<IconComponent class={`h-4 w-4 shrink-0 ${isSelected ? 'text-[var(--accent)]' : 'text-[var(--text-dim)]'}`} />
							<span>{cmd.title}</span>
						</div>
						<div class="flex items-center gap-2">
							<span class="text-[10px] text-[var(--text-faint)] uppercase tracking-wider">{cmd.category}</span>
							{#if cmd.shortcut}
								<span class="font-mono text-[10px] text-[var(--text-faint)] bg-[var(--border)] px-1.5 py-0.5 rounded">
									{cmd.shortcut}
								</span>
							{/if}
						</div>
					</button>
				{:else}
					<div class="py-8 text-center text-xs text-[var(--text-dim)]">
						<p>No matching commands found.</p>
						<button
							onclick={() => {
								onSearch(filterText.trim());
								isOpen = false;
							}}
							class="mt-2 text-[var(--accent)] hover:underline inline-flex items-center gap-1 font-semibold"
						>
							<span>Search "{filterText}" with Qori</span>
							<ArrowRight class="h-3 w-3" />
						</button>
					</div>
				{/each}
			</div>

			<!-- Footer -->
			<div class="px-4 py-2 border-t border-[var(--border)] flex items-center justify-between text-[11px] text-[var(--text-faint)] bg-[var(--surface-hover)]">
				<div class="flex items-center gap-2 font-mono text-[10px]">
					<span>[↑↓] NAVIGATE</span>
					<span>[↵] SELECT</span>
					<span>[ESC] CLOSE</span>
				</div>
				<span class="font-bold text-emerald-400 font-mono text-[10px]">AIR-GAPPED SYSTEM</span>
			</div>
		</div>
	</div>
{/if}
