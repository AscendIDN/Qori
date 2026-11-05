<script lang="ts">
	import { onMount } from 'svelte';

	let scrollPercent = $state('000%');
	let rawPercent = $state(0);

	function updateScroll() {
		if (typeof window === 'undefined') return;
		const docHeight = document.documentElement.scrollHeight - window.innerHeight;
		if (docHeight <= 0) {
			rawPercent = 0;
			scrollPercent = '000%';
			return;
		}
		const current = Math.min(100, Math.max(0, (window.scrollY / docHeight) * 100));
		rawPercent = current;
		scrollPercent = Math.round(current).toString().padStart(3, '0') + '%';
	}

	onMount(() => {
		updateScroll();
		window.addEventListener('scroll', updateScroll, { passive: true });
		window.addEventListener('resize', updateScroll);
		return () => {
			window.removeEventListener('scroll', updateScroll);
			window.removeEventListener('resize', updateScroll);
		};
	});

	const ticks = [0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100];
</script>

<aside
	class="pointer-events-none fixed top-12 right-0 bottom-0 z-30 hidden w-10 flex-col items-center justify-between border-l border-[var(--line)] bg-[var(--bg)]/80 px-1.5 py-4 backdrop-blur-sm select-none lg:flex"
>
	<!-- Top readout -->
	<div class="font-mono text-[9px] font-bold tracking-widest text-[var(--accent-blue)]">
		{scrollPercent}
	</div>

	<!-- Ruler Marks -->
	<div class="relative my-2 flex h-[75%] w-full flex-col items-center justify-between">
		<!-- Active scroll needle indicator -->
		<div
			class="absolute right-0 left-0 h-[2px] bg-[var(--accent-blue)] shadow-[0_0_8px_var(--accent-blue)] transition-all duration-75"
			style={`top: ${rawPercent}%;`}
		></div>

		{#each ticks as tick}
			{@const isMajor = tick % 20 === 0}
			<div class="flex w-full items-center justify-end pr-0.5">
				{#if isMajor}
					<span class="mr-1 font-mono text-[8px] tracking-tighter text-[var(--fg-faint)]">
						{tick.toString().padStart(2, '0')}
					</span>
					<div class="h-[1px] w-3 bg-[var(--fg-dim)]"></div>
				{:else if tick % 10 === 0}
					<div class="h-[1px] w-2 bg-[var(--line-strong)]"></div>
				{:else}
					<div class="h-[1px] w-1 bg-[var(--line)]"></div>
				{/if}
			</div>
		{/each}
	</div>

	<!-- Bottom marker -->
	<div class="font-mono text-[8px] tracking-widest text-[var(--fg-faint)] uppercase">RULER</div>
</aside>
