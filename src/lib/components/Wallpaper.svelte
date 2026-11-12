<script lang="ts">
	export type WallpaperId =
		| 'dark-lavender'
		| 'dark-midnight'
		| 'dark-emerald'
		| 'light-pastel'
		| 'light-breeze'
		| 'light-golden'
		| 'solid';

	let {
		wallpaper = 'dark-lavender',
		theme = 'dark',
		dim = 0,
		blur = 0,
		customColor = ''
	}: {
		wallpaper?: WallpaperId;
		theme?: 'dark' | 'light';
		dim?: number; // 0 - 80%
		blur?: number; // 0 - 24px
		customColor?: string;
	} = $props();

	// If wallpaper is not explicitly chosen, choose default based on theme
	let activeWallpaper = $derived.by(() => {
		if (wallpaper === 'solid') return 'solid';
		if (theme === 'light' && wallpaper.startsWith('dark-')) {
			return 'light-pastel';
		}
		if (theme === 'dark' && wallpaper.startsWith('light-')) {
			return 'dark-lavender';
		}
		return wallpaper;
	});
</script>

<div class="pointer-events-none fixed inset-0 z-0 overflow-hidden select-none" aria-hidden="true">
	<!-- Background Wallpaper Base -->
	<div
		class="absolute inset-0 transition-all duration-700 ease-out"
		style={`filter: blur(${blur}px); transform: scale(1.05);`}
	>
		{#if activeWallpaper === 'solid'}
			<div
				class="h-full w-full"
				style={`background-color: ${customColor || (theme === 'dark' ? '#12131C' : '#F4F6FC')};`}
			></div>
		{:else if activeWallpaper === 'dark-lavender'}
			<!-- Reference Dark: Opera-inspired Soft Lavender, Indigo & Twilight Pink -->
			<div
				class="h-full w-full bg-gradient-to-br from-[#121324] via-[#1A1936] to-[#0E101D] relative"
			>
				<div
					class="absolute -top-[25%] -left-[15%] h-[75vw] w-[75vw] rounded-full bg-gradient-to-br from-[#6C5CE7]/25 via-[#A29BFE]/15 to-transparent blur-[120px]"
				></div>
				<div
					class="absolute top-[35%] -right-[20%] h-[70vw] w-[70vw] rounded-full bg-gradient-to-bl from-[#FD79A8]/18 via-[#8C7AE6]/18 to-transparent blur-[140px]"
				></div>
				<div
					class="absolute -bottom-[20%] left-[20%] h-[60vw] w-[60vw] rounded-full bg-gradient-to-tr from-[#0984E3]/15 via-[#6C5CE7]/12 to-transparent blur-[130px]"
				></div>
			</div>
		{:else if activeWallpaper === 'dark-midnight'}
			<!-- Dark Velvet Midnight & Plum -->
			<div
				class="h-full w-full bg-gradient-to-tr from-[#0B0C14] via-[#151324] to-[#0C1220] relative"
			>
				<div
					class="absolute top-[10%] left-[10%] h-[65vw] w-[65vw] rounded-full bg-gradient-to-r from-[#4A154B]/22 to-[#3B3B98]/18 blur-[130px]"
				></div>
				<div
					class="absolute bottom-[5%] right-[10%] h-[60vw] w-[60vw] rounded-full bg-gradient-to-l from-[#1B1464]/25 to-[#5758BB]/15 blur-[120px]"
				></div>
			</div>
		{:else if activeWallpaper === 'dark-emerald'}
			<!-- Dark Emerald & Twilight Slate -->
			<div
				class="h-full w-full bg-gradient-to-br from-[#091418] via-[#0E1F24] to-[#071013] relative"
			>
				<div
					class="absolute -top-[10%] right-[10%] h-[70vw] w-[70vw] rounded-full bg-gradient-to-b from-[#00B894]/18 to-[#0984E3]/12 blur-[140px]"
				></div>
				<div
					class="absolute bottom-[10%] left-[5%] h-[60vw] w-[60vw] rounded-full bg-gradient-to-tr from-[#00CEC9]/15 to-transparent blur-[120px]"
				></div>
			</div>
		{:else if activeWallpaper === 'light-pastel'}
			<!-- Reference Light: Opera One Palette (Lavender, Sky Blue, Soft Rose) -->
			<div
				class="h-full w-full bg-gradient-to-br from-[#E8EDF9] via-[#E2E6F8] to-[#E9DEF5] relative"
			>
				<div
					class="absolute -top-[15%] left-[5%] h-[75vw] w-[75vw] rounded-full bg-gradient-to-br from-[#BAC8FF]/50 via-[#D0BFFF]/45 to-transparent blur-[110px]"
				></div>
				<div
					class="absolute top-[25%] -right-[15%] h-[75vw] w-[75vw] rounded-full bg-gradient-to-bl from-[#FCC2D7]/40 via-[#E599F7]/30 to-transparent blur-[120px]"
				></div>
				<div
					class="absolute -bottom-[15%] left-[20%] h-[65vw] w-[65vw] rounded-full bg-gradient-to-tr from-[#A5D8FF]/45 via-[#B197FC]/35 to-transparent blur-[130px]"
				></div>
			</div>
		{:else if activeWallpaper === 'light-breeze'}
			<!-- Light Azure Sky & Clean Mint -->
			<div
				class="h-full w-full bg-gradient-to-b from-[#E7F5FF] via-[#E6FCF5] to-[#F3F0FF] relative"
			>
				<div
					class="absolute -top-[10%] left-[20%] h-[70vw] w-[70vw] rounded-full bg-[#74C0FC]/35 blur-[120px]"
				></div>
				<div
					class="absolute bottom-[10%] right-[15%] h-[65vw] w-[65vw] rounded-full bg-[#63E6BE]/30 blur-[130px]"
				></div>
			</div>
		{:else if activeWallpaper === 'light-golden'}
			<!-- Light Champagne & Warm Peach -->
			<div
				class="h-full w-full bg-gradient-to-tr from-[#FFF4E6] via-[#FFF0F6] to-[#F8F0FC] relative"
			>
				<div
					class="absolute top-[10%] right-[10%] h-[70vw] w-[70vw] rounded-full bg-[#FFD8A8]/45 blur-[130px]"
				></div>
				<div
					class="absolute bottom-[5%] left-[10%] h-[65vw] w-[65vw] rounded-full bg-[#FFC9C9]/35 blur-[120px]"
				></div>
			</div>
		{/if}
	</div>

	<!-- Dim overlay adjustable from Settings -->
	{#if dim > 0}
		<div
			class="absolute inset-0 bg-black transition-opacity duration-300"
			style={`opacity: ${dim / 100};`}
		></div>
	{/if}

	<!-- Subtle WCAG AA Contrast Scrim -->
	<div
		class={`absolute inset-0 transition-opacity duration-300 ${
			theme === 'dark' ? 'bg-black/20' : 'bg-white/20'
		}`}
	></div>
</div>

