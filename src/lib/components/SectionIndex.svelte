<script lang="ts">
	let {
		activeTab = $bindable('search'),
		onSelect = (tab: string) => {}
	}: {
		activeTab: string;
		onSelect?: (tab: string) => void;
	} = $props();

	const sections = [
		{ id: 'search', num: '01', title: 'SEARCH' },
		{ id: 'results', num: '02', title: 'RESULTS' },
		{ id: 'library', num: '03', title: 'LIBRARY' },
		{ id: 'sources', num: '04', title: 'SOURCES' },
		{ id: 'settings', num: '05', title: 'SETTINGS' }
	];

	function select(id: string) {
		activeTab = id;
		onSelect(id);
	}
</script>

<div
	class="fixed right-14 bottom-6 z-30 hidden flex-col items-end gap-1.5 font-mono text-[10px] tracking-widest uppercase select-none md:flex"
>
	<div class="mb-1 text-[9px] tracking-[0.25em] text-[var(--fg-faint)]">INDEX // NAVIGATION</div>
	{#each sections as sec}
		{@const isActive = activeTab === sec.id}
		<button
			onclick={() => select(sec.id)}
			class={`group flex items-center gap-2 py-0.5 text-right transition-all ${
				isActive
					? 'font-bold text-[var(--fg)]'
					: 'text-[var(--fg-faint)] hover:text-[var(--fg-dim)]'
			}`}
		>
			<span class={isActive ? 'text-[var(--accent-blue)]' : 'text-[var(--fg-faint)]'}>
				{sec.num}
			</span>
			<span>/</span>
			<span class="tracking-wider">{sec.title}</span>
			<span
				class={`inline-block h-[2px] transition-all ${
					isActive
						? 'w-4 bg-[var(--accent-blue)]'
						: 'w-1.5 bg-transparent group-hover:bg-[var(--line-strong)]'
				}`}
			></span>
		</button>
	{/each}
</div>
