<script lang="ts">
	import { Sun, Moon, Search, Plus, Terminal } from 'lucide-svelte';

	let {
		activeTab = $bindable('search'),
		onOpenCommandPalette = () => {},
		onQuickAdd = () => {},
		theme = $bindable<'dark' | 'light'>('dark')
	}: {
		activeTab: string;
		onOpenCommandPalette?: () => void;
		onQuickAdd?: () => void;
		theme: 'dark' | 'light';
	} = $props();

	const navItems = [
		{ id: 'search', num: '01', label: 'SEARCH' },
		{ id: 'results', num: '02', label: 'RESULTS' },
		{ id: 'library', num: '03', label: 'LIBRARY' },
		{ id: 'sources', num: '04', label: 'SOURCES' },
		{ id: 'settings', num: '05', label: 'SETTINGS' }
	];

	function toggleTheme() {
		theme = theme === 'dark' ? 'light' : 'dark';
		document.documentElement.setAttribute('data-theme', theme);
	}
</script>

<header
	class="sticky top-0 z-40 w-full border-b border-[var(--line)] bg-[var(--bg)]/90 backdrop-blur-md"
>
	<div class="flex h-12 w-full items-stretch justify-between">
		<!-- Brand Mark / App Ident -->
		<button
			onclick={() => (activeTab = 'search')}
			class="group flex items-center gap-2 border-r border-[var(--line)] px-4 font-mono text-xs tracking-widest text-[var(--fg)] transition-colors hover:bg-[var(--surface-hover)]"
		>
			<span class="inline-block h-2 w-2 bg-[var(--accent-blue)]"></span>
			<span class="font-bold tracking-[0.2em] uppercase">PRIVATE SEARCH</span>
			<span class="text-[10px] text-[var(--fg-faint)]">// 100% LOCAL</span>
		</button>

		<!-- Navigation Tabs with Hairline Dividers -->
		<nav class="hidden items-stretch font-mono text-xs md:flex">
			{#each navItems as item}
				{@const isActive = activeTab === item.id}
				<button
					onclick={() => (activeTab = item.id)}
					class={`relative flex items-center gap-2 border-r border-[var(--line)] px-4 tracking-widest uppercase transition-all ${
						isActive
							? 'bg-[var(--fg)] font-bold text-[var(--bg)]'
							: 'text-[var(--fg-dim)] hover:bg-[var(--surface-hover)] hover:text-[var(--fg)]'
					}`}
				>
					<span class={isActive ? 'text-[var(--bg)]/70' : 'text-[var(--fg-faint)]'}>{item.num}</span
					>
					<span>{item.label}</span>
					{#if isActive}
						<span class="absolute right-0 bottom-0 left-0 h-[2px] bg-[var(--accent-blue)]"></span>
					{/if}
				</button>
			{/each}
		</nav>

		<!-- Right Controls -->
		<div class="flex items-stretch">
			<!-- Command Palette Shortcut Pill -->
			<button
				onclick={onOpenCommandPalette}
				class="hidden items-center gap-2 border-l border-[var(--line)] px-3 font-mono text-xs text-[var(--fg-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--fg)] sm:flex"
				title="Open Command Palette (Ctrl+K)"
			>
				<Search class="h-3.5 w-3.5" />
				<span class="tracking-widest">COMMAND</span>
				<kbd
					class="border border-[var(--line-strong)] bg-[var(--surface)] px-1 py-0.5 font-mono text-[10px] text-[var(--fg-faint)]"
				>
					⌘K
				</kbd>
			</button>

			<!-- Theme Switcher -->
			<button
				onclick={toggleTheme}
				aria-label="Toggle visual theme"
				class="flex items-center justify-center border-l border-[var(--line)] px-3 text-[var(--fg-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--fg)]"
				title={theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
			>
				{#if theme === 'dark'}
					<Sun class="h-4 w-4" />
				{:else}
					<Moon class="h-4 w-4" />
				{/if}
			</button>

			<!-- Primary Action CTA Button -->
			<button
				onclick={onQuickAdd}
				class="flex items-center gap-1.5 border-l border-[var(--line)] bg-[var(--fg)] px-4 font-mono text-xs font-bold tracking-wider text-[var(--bg)] uppercase transition-all hover:bg-[var(--fg)]/90 active:scale-[0.98]"
			>
				<Plus class="h-3.5 w-3.5 stroke-[2.5]" />
				<span class="hidden sm:inline">ADD SOURCE</span>
				<span class="sm:hidden">ADD</span>
			</button>
		</div>
	</div>
</header>
