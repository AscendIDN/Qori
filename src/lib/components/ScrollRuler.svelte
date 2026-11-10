<script lang="ts">
	import { onMount } from 'svelte';

	let isScrollable = $state(false);
	let maxRulerTick = $state(100);
	let scrollFraction = $state(0);
	let currentTickVal = $state(0);
	let scrollPercent = $state('000%');
	let isHovered = $state(false);
	let isScrolling = $state(false);
	let isDragging = $state(false);
	let mouseFraction = $state(0);

	let rulerTrackEl = $state<HTMLDivElement | null>(null);
	let scrollTimeout: ReturnType<typeof setTimeout> | null = null;

	function getScrollTop(): number {
		if (typeof window === 'undefined') return 0;
		return window.scrollY || document.documentElement.scrollTop || document.body.scrollTop || 0;
	}

	function getScrollMetrics() {
		if (typeof window === 'undefined') return { maxScroll: 0, clientHeight: 1, scrollHeight: 1 };
		const clientHeight = window.innerHeight || document.documentElement.clientHeight || 1;
		const scrollHeight = Math.max(
			document.documentElement.scrollHeight,
			document.body.scrollHeight,
			document.documentElement.offsetHeight,
			document.body.offsetHeight
		);
		const maxScroll = Math.max(0, scrollHeight - clientHeight);
		return { maxScroll, clientHeight, scrollHeight };
	}

	function setScrollTop(top: number, smooth: boolean = false) {
		if (typeof window === 'undefined') return;
		const behavior = smooth ? 'smooth' : 'instant';
		window.scrollTo({ top, behavior });
		if (document.documentElement) document.documentElement.scrollTop = top;
		if (document.body) document.body.scrollTop = top;
	}

	function updateScroll() {
		if (typeof window === 'undefined') return;
		const { maxScroll, clientHeight } = getScrollMetrics();

		// If page is not scrollable (e.g. HomeScreen), hide ruler completely
		if (maxScroll <= 20) {
			isScrollable = false;
			scrollFraction = 0;
			currentTickVal = 0;
			scrollPercent = '000%';
			return;
		}

		isScrollable = true;

		// 100 is the maximal limit. Scale down dynamically if scroll capacity is shorter (e.g. 50, 75).
		const scrollScreens = maxScroll / clientHeight;
		if (scrollScreens >= 2.0) {
			maxRulerTick = 100;
		} else {
			const raw = (scrollScreens / 2.0) * 100;
			const stepped = Math.round(raw / 5) * 5;
			maxRulerTick = Math.min(100, Math.max(20, stepped));
		}

		const currentScroll = getScrollTop();
		const fraction = Math.min(1, Math.max(0, currentScroll / maxScroll));
		scrollFraction = fraction;
		currentTickVal = Math.round(fraction * maxRulerTick);
		scrollPercent = currentTickVal.toString().padStart(3, '0') + '%';

		// Trigger active scrolling state
		isScrolling = true;
		if (scrollTimeout) clearTimeout(scrollTimeout);
		scrollTimeout = setTimeout(() => {
			isScrolling = false;
		}, 1100);
	}

	function handleTrackScrub(clientY: number, smooth: boolean = false) {
		if (!rulerTrackEl) return;
		const rect = rulerTrackEl.getBoundingClientRect();
		if (rect.height <= 0) return;
		const offsetY = Math.max(0, Math.min(rect.height, clientY - rect.top));
		const fraction = offsetY / rect.height;
		const { maxScroll } = getScrollMetrics();
		if (maxScroll > 0) {
			setScrollTop(fraction * maxScroll, smooth);
		}
		updateScroll();
	}

	function onTrackPointerDown(e: PointerEvent) {
		isDragging = true;
		handleTrackScrub(e.clientY, false);
		window.addEventListener('pointermove', onWindowPointerMove);
		window.addEventListener('pointerup', onWindowPointerUp);
	}

	function onWindowPointerMove(e: PointerEvent) {
		if (!isDragging) return;
		handleTrackScrub(e.clientY, false);
	}

	function onWindowPointerUp() {
		isDragging = false;
		window.removeEventListener('pointermove', onWindowPointerMove);
		window.removeEventListener('pointerup', onWindowPointerUp);
	}

	function onTrackPointerMove(e: PointerEvent) {
		if (!rulerTrackEl) return;
		const rect = rulerTrackEl.getBoundingClientRect();
		if (rect.height <= 0) return;
		const offsetY = Math.max(0, Math.min(rect.height, e.clientY - rect.top));
		mouseFraction = offsetY / rect.height;
	}

	onMount(() => {
		updateScroll();

		// Listen to scroll events with capture to intercept any sub-container scrolls
		window.addEventListener('scroll', updateScroll, { passive: true, capture: true });
		document.addEventListener('scroll', updateScroll, { passive: true, capture: true });
		window.addEventListener('resize', updateScroll);

		// Observe body and root element resize
		const resizeObserver = new ResizeObserver(() => {
			updateScroll();
		});
		if (document.body) {
			resizeObserver.observe(document.body);
		}
		if (document.documentElement) {
			resizeObserver.observe(document.documentElement);
		}

		// Observe DOM tree changes (such as tab transitions or search result rendering)
		const mutationObserver = new MutationObserver(() => {
			updateScroll();
		});
		if (document.body) {
			mutationObserver.observe(document.body, { childList: true, subtree: true });
		}

		return () => {
			window.removeEventListener('scroll', updateScroll, { capture: true });
			document.removeEventListener('scroll', updateScroll, { capture: true });
			window.removeEventListener('resize', updateScroll);
			resizeObserver.disconnect();
			mutationObserver.disconnect();
			if (scrollTimeout) clearTimeout(scrollTimeout);
			window.removeEventListener('pointermove', onWindowPointerMove);
			window.removeEventListener('pointerup', onWindowPointerUp);
		};
	});

	// Dynamic ticks generated up to maxRulerTick (e.g. 0 to 50, or 0 to 100)
	const ticks = $derived(Array.from({ length: maxRulerTick + 1 }, (_, i) => i));
</script>

{#if isScrollable}
	<!-- Interactive Precision Scroll Ruler (Neo-Brutalist Micrometer Instrument) -->
	<aside
		onpointerenter={() => (isHovered = true)}
		onpointerleave={() => (isHovered = false)}
		onwheel={(e) => {
			e.preventDefault();
			window.scrollBy({ top: e.deltaY, behavior: 'instant' });
			if (document.documentElement) document.documentElement.scrollTop += e.deltaY;
			if (document.body) document.body.scrollTop += e.deltaY;
			updateScroll();
		}}
		class={`fixed top-[84px] bottom-3 right-0 z-40 flex select-none transition-all duration-200 ${
			isHovered || isDragging
				? 'w-16 bg-[#0a0b0e]/95 backdrop-blur-md shadow-2xl border-l border-[var(--line-strong)]'
				: isScrolling
					? 'w-14 bg-[#0a0b0e]/80 backdrop-blur-sm border-l border-[var(--line)] opacity-95'
					: 'w-10 bg-transparent border-l border-[var(--line)]/40 opacity-40 hover:opacity-100'
		}`}
	>
		<!-- Ruler Track Container (Scaled dynamically to maxRulerTick%) -->
		<div
			bind:this={rulerTrackEl}
			onpointerdown={onTrackPointerDown}
			onpointermove={onTrackPointerMove}
			class="relative w-full cursor-pointer py-3 select-none transition-[height] duration-300"
			style={`height: ${maxRulerTick}%;`}
		>
			<!-- Hover Guideline (White Needle following cursor on ruler - Gambar 2) -->
			{#if (isHovered || isDragging) && !isScrolling}
				<div
					class="pointer-events-none absolute left-0 right-0 z-10 transition-opacity"
					style={`top: ${mouseFraction * 100}%; transform: translateY(-0.5px);`}
				>
					<div class="h-[1px] w-full bg-white/75 shadow-[0_0_4px_rgba(255,255,255,0.7)]"></div>
				</div>
			{/if}

			<!-- Active Blue Scroll Needle & Percent Badge (Gambar 3 - Appears on Scroll / Hover / Drag) -->
			<div
				class={`pointer-events-none absolute left-0 right-0 z-30 flex flex-col transition-opacity duration-150 ${
					isScrolling || isHovered || isDragging ? 'opacity-100' : 'opacity-0'
				}`}
				style={`top: ${scrollFraction * 100}%; transform: translateY(-1px);`}
			>
				<!-- Glowing Neon Blue Line -->
				<div
					class="h-[2px] w-full bg-[#2f6bff] shadow-[0_0_8px_#2f6bff,0_0_2px_#2f6bff]"
				></div>

				<!-- Percentage Readout (e.g. 013%, 050%, 100%) -->
				<div
					class={`pl-1 font-mono text-[10px] font-bold tracking-wider text-[#3b82f6] select-none transition-transform ${
						scrollFraction > 0.88 ? '-translate-y-5' : 'mt-0.5'
					}`}
					style="text-shadow: 0 0 8px rgba(47, 107, 255, 0.6);"
				>
					{scrollPercent}
				</div>
			</div>

			<!-- Dynamic Tick Marks Scale (0 to maxRulerTick) -->
			{#each ticks as tick}
				{@const isMajor = tick % 5 === 0}
				{@const isTenth = tick % 10 === 0}
				{@const topPercent = (tick / maxRulerTick) * 100}

				<div
					class="pointer-events-none absolute right-0 flex items-center justify-end"
					style={`top: ${topPercent}%; transform: translateY(-50%); width: 100%;`}
				>
					{#if isMajor}
						<!-- Major Tick Number (0, 5, 10, ... maxRulerTick) -->
						<span
							class={`mr-1.5 font-mono tracking-tighter transition-all ${
								isHovered || isDragging || isScrolling
									? isTenth
										? 'text-[9px] font-bold text-[var(--fg)]'
										: 'text-[8px] font-medium text-[var(--fg-dim)]'
									: 'text-[8px] text-[var(--fg-faint)]'
							}`}
						>
							{tick}
						</span>

						<!-- Major Horizontal Line -->
						<div
							class={`h-[1px] transition-all ${
								isHovered || isDragging
									? isTenth
										? 'w-5 bg-[var(--fg)]'
										: 'w-3.5 bg-[var(--fg-dim)]'
									: isTenth
										? 'w-3.5 bg-[var(--line-strong)]'
										: 'w-2 bg-[var(--line)]'
							}`}
						></div>
					{:else}
						<!-- Minor Sub-Tick (1, 2, 3, 4 ...) - Micrometer style -->
						<div
							class={`h-[1px] transition-all ${
								isHovered || isDragging
									? 'w-2 bg-[var(--line-strong)] opacity-90'
									: isScrolling
										? 'w-1.5 bg-[var(--line)] opacity-40'
										: 'w-1 bg-[var(--line)]/30 opacity-0'
							}`}
						></div>
					{/if}
				</div>
			{/each}
		</div>
	</aside>
{/if}
