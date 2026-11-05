<script lang="ts">
	import { onMount } from 'svelte';
	import {
		Search,
		Compass,
		BookOpen,
		Globe,
		Settings,
		Plus,
		Download,
		Sun,
		Moon,
		Sparkles
	} from 'lucide-svelte';

	let {
		isOpen = $bindable(false),
		onSelectTab = (tab: string) => {},
		onQuickAdd = () => {},
		onExport = () => {},
		onToggleTheme = () => {}
	}: {
		isOpen: boolean;
		onSelectTab?: (tab: string) => void;
		onQuickAdd?: () => void;
		onExport?: () => void;
		onToggleTheme?: () => void;
	} = $props();

	let query = $state('');
	let selectedIndex = $state(0);
	let inputEl = $state<HTMLInputElement | null>(null);

	interface CommandItem {
		id: string;
		label: string;
		category: 'NAVIGATION' | 'ACTIONS' | 'SYSTEM';
		icon: any;
		shortcut?: string;
		action: () => void;
	}

	const allCommands: CommandItem[] = [
		{
			id: 'nav-search',
			label: 'Navigate: 01 / Search Console',
			category: 'NAVIGATION',
			icon: Search,
			shortcut: 'G S',
			action: () => {
				onSelectTab('search');
				close();
			}
		},
		{
			id: 'nav-results',
			label: 'Navigate: 02 / Search Results',
			category: 'NAVIGATION',
			icon: Compass,
			shortcut: 'G R',
			action: () => {
				onSelectTab('results');
				close();
			}
		},
		{
			id: 'nav-library',
			label: 'Navigate: 03 / Library Grid',
			category: 'NAVIGATION',
			icon: BookOpen,
			shortcut: 'G L',
			action: () => {
				onSelectTab('library');
				close();
			}
		},
		{
			id: 'nav-sources',
			label: 'Navigate: 04 / Sources & Crawler Logs',
			category: 'NAVIGATION',
			icon: Globe,
			shortcut: 'G C',
			action: () => {
				onSelectTab('sources');
				close();
			}
		},
		{
			id: 'nav-settings',
			label: 'Navigate: 05 / Settings & Privacy',
			category: 'NAVIGATION',
			icon: Settings,
			shortcut: 'G P',
			action: () => {
				onSelectTab('settings');
				close();
			}
		},
		{
			id: 'act-add',
			label: 'Add Source (URL, Local Folder, Bookmarks)',
			category: 'ACTIONS',
			icon: Plus,
			shortcut: '⌘N',
			action: () => {
				close();
				onQuickAdd();
			}
		},
		{
			id: 'act-export',
			label: 'Export Complete Index & Library (.zip)',
			category: 'ACTIONS',
			icon: Download,
			shortcut: '⌘E',
			action: () => {
				close();
				onExport();
			}
		},
		{
			id: 'act-theme',
			label: 'Toggle Visual Mode (Dark / Light Theme)',
			category: 'SYSTEM',
			icon: Sun,
			shortcut: '⌘T',
			action: () => {
				close();
				onToggleTheme();
			}
		}
	];

	let filteredCommands = $derived(
		allCommands.filter(
			(c) =>
				c.label.toLowerCase().includes(query.toLowerCase()) ||
				c.category.toLowerCase().includes(query.toLowerCase())
		)
	);

	function close() {
		isOpen = false;
		query = '';
		selectedIndex = 0;
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (!isOpen) {
			if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
				e.preventDefault();
				isOpen = true;
			}
			return;
		}

		if (e.key === 'Escape') {
			e.preventDefault();
			close();
		} else if (e.key === 'ArrowDown') {
			e.preventDefault();
			selectedIndex = (selectedIndex + 1) % (filteredCommands.length || 1);
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			selectedIndex =
				(selectedIndex - 1 + filteredCommands.length) % (filteredCommands.length || 1);
		} else if (e.key === 'Enter') {
			e.preventDefault();
			if (filteredCommands[selectedIndex]) {
				filteredCommands[selectedIndex].action();
			}
		}
	}

	onMount(() => {
		window.addEventListener('keydown', handleKeyDown);
		return () => window.removeEventListener('keydown', handleKeyDown);
	});

	$effect(() => {
		if (isOpen) {
			setTimeout(() => inputEl?.focus(), 50);
		}
	});
</script>

{#if isOpen}
	<div
		class="fixed inset-0 z-50 flex items-start justify-center bg-black/70 px-4 pt-20 backdrop-blur-sm"
		onclick={(e) => {
			if (e.target === e.currentTarget) close();
		}}
		role="presentation"
	>
		<div
			class="animate-in fade-in zoom-in-95 w-full max-w-xl border border-[var(--line-strong)] bg-[var(--surface)] shadow-2xl duration-150"
		>
			<!-- Top header / search input -->
			<div class="flex items-center gap-3 border-b border-[var(--line)] px-4 py-3">
				<Search class="h-4 w-4 text-[var(--fg-dim)]" />
				<input
					bind:this={inputEl}
					bind:value={query}
					type="text"
					placeholder="Type a command or jump to screen..."
					class="flex-1 bg-transparent font-mono text-sm text-[var(--fg)] placeholder:text-[var(--fg-faint)] focus:outline-none"
				/>
				<kbd
					class="border border-[var(--line)] bg-[var(--bg)] px-1.5 py-0.5 font-mono text-[10px] text-[var(--fg-faint)]"
				>
					ESC
				</kbd>
			</div>

			<!-- Command list -->
			<div class="max-h-80 overflow-y-auto py-2">
				{#if filteredCommands.length === 0}
					<div class="px-4 py-8 text-center font-mono text-xs text-[var(--fg-faint)]">
						NO MATCHING COMMANDS FOUND
					</div>
				{:else}
					{#each filteredCommands as cmd, i}
						{@const isSelected = i === selectedIndex}
						{@const Icon = cmd.icon}
						<button
							onclick={cmd.action}
							onmouseenter={() => (selectedIndex = i)}
							class={`flex w-full items-center justify-between px-4 py-2.5 text-left font-mono text-xs transition-colors ${
								isSelected
									? 'bg-[var(--fg)] font-bold text-[var(--bg)]'
									: 'text-[var(--fg)] hover:bg-[var(--surface-hover)]'
							}`}
						>
							<div class="flex items-center gap-3">
								<Icon
									class={`h-4 w-4 ${isSelected ? 'text-[var(--bg)]' : 'text-[var(--accent-blue)]'}`}
								/>
								<span>{cmd.label}</span>
							</div>
							<div class="flex items-center gap-2">
								<span
									class={`text-[10px] tracking-wider uppercase ${isSelected ? 'text-[var(--bg)]/70' : 'text-[var(--fg-faint)]'}`}
								>
									{cmd.category}
								</span>
								{#if cmd.shortcut}
									<kbd
										class={`border px-1 py-0.5 text-[9px] ${isSelected ? 'border-[var(--bg)]/30 bg-[var(--bg)]/10 text-[var(--bg)]' : 'border-[var(--line)] bg-[var(--bg)] text-[var(--fg-faint)]'}`}
									>
										{cmd.shortcut}
									</kbd>
								{/if}
							</div>
						</button>
					{/each}
				{/if}
			</div>

			<!-- Footer bar -->
			<div
				class="flex items-center justify-between border-t border-[var(--line)] bg-[var(--bg-deep)] px-4 py-2 font-mono text-[10px] text-[var(--fg-faint)]"
			>
				<div class="flex items-center gap-3">
					<span>[↑↓] NAVIGATE</span>
					<span>[↵] SELECT</span>
					<span>[ESC] CLOSE</span>
				</div>
				<span class="text-[var(--accent-green)]">AIR-GAPPED SYSTEM</span>
			</div>
		</div>
	</div>
{/if}
