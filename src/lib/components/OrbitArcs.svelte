<script lang="ts">
	import { onMount } from 'svelte';

	let mouseX = $state(0);
	let mouseY = $state(0);
	let targetX = 0;
	let targetY = 0;
	let rafId: number;

	function handleMouseMove(e: MouseEvent) {
		const cx = window.innerWidth / 2;
		const cy = window.innerHeight / 2;
		targetX = (e.clientX - cx) / cx;
		targetY = (e.clientY - cy) / cy;
	}

	function tick() {
		mouseX += (targetX - mouseX) * 0.05;
		mouseY += (targetY - mouseY) * 0.05;
		rafId = requestAnimationFrame(tick);
	}

	onMount(() => {
		window.addEventListener('mousemove', handleMouseMove, { passive: true });
		rafId = requestAnimationFrame(tick);
		return () => {
			window.removeEventListener('mousemove', handleMouseMove);
			cancelAnimationFrame(rafId);
		};
	});
</script>

<div class="pointer-events-none absolute inset-0 z-0 overflow-hidden select-none">
	<!-- Gradient Orbit Arcs SVG -->
	<svg
		class="absolute -top-24 left-1/2 h-[900px] w-[1400px] -translate-x-1/2 opacity-40"
		viewBox="0 0 1400 900"
		fill="none"
	>
		<defs>
			<linearGradient id="orbit-blue" x1="0%" y1="0%" x2="100%" y2="100%">
				<stop offset="0%" stop-color="#2f6bff" stop-opacity="0.8" />
				<stop offset="100%" stop-color="#2f6bff" stop-opacity="0" />
			</linearGradient>
			<linearGradient id="orbit-purple" x1="0%" y1="100%" x2="100%" y2="0%">
				<stop offset="0%" stop-color="#8a2be2" stop-opacity="0.7" />
				<stop offset="100%" stop-color="#8a2be2" stop-opacity="0" />
			</linearGradient>
			<linearGradient id="orbit-green" x1="100%" y1="0%" x2="0%" y2="100%">
				<stop offset="0%" stop-color="#3ddc84" stop-opacity="0.6" />
				<stop offset="100%" stop-color="#3ddc84" stop-opacity="0" />
			</linearGradient>
			<linearGradient id="orbit-red" x1="50%" y1="0%" x2="50%" y2="100%">
				<stop offset="0%" stop-color="#e5484d" stop-opacity="0.5" />
				<stop offset="100%" stop-color="#e5484d" stop-opacity="0" />
			</linearGradient>
		</defs>

		<!-- Outer ellipse arcs with dash animation -->
		<ellipse
			cx="700"
			cy="380"
			rx="640"
			ry="280"
			stroke="url(#orbit-blue)"
			stroke-width="1.2"
			class="orbit-arc"
		/>
		<ellipse
			cx="700"
			cy="400"
			rx="520"
			ry="220"
			stroke="url(#orbit-purple)"
			stroke-width="1.2"
			class="orbit-arc"
			style="animation-delay: 0.3s;"
		/>
		<ellipse
			cx="700"
			cy="420"
			rx="400"
			ry="170"
			stroke="url(#orbit-green)"
			stroke-width="1"
			class="orbit-arc"
			style="animation-delay: 0.6s;"
		/>
		<ellipse
			cx="700"
			cy="440"
			rx="280"
			ry="120"
			stroke="url(#orbit-red)"
			stroke-width="0.8"
			class="orbit-arc"
			style="animation-delay: 0.9s;"
		/>
	</svg>

	<!-- Faint Outlined Floating Cards with Cursor Parallax -->
	<div
		class="absolute top-[18%] left-[8%] hidden w-48 border border-[var(--line)] bg-[var(--surface)]/30 p-3 backdrop-blur-[2px] transition-transform duration-100 ease-out xl:block"
		style={`transform: translate3d(${mouseX * -25}px, ${mouseY * -20}px, 0);`}
	>
		<div
			class="mb-2 flex items-center justify-between border-b border-[var(--line)] pb-1.5 font-mono text-[9px] text-[var(--fg-faint)]"
		>
			<span>LOCAL INDEX</span>
			<span class="text-[var(--accent-green)]">BM25</span>
		</div>
		<div class="mb-1.5 h-1.5 w-24 bg-[var(--line)]"></div>
		<div class="h-1.5 w-16 bg-[var(--line)]"></div>
	</div>

	<div
		class="absolute top-[28%] right-[10%] hidden w-56 border border-[var(--line)] bg-[var(--surface)]/30 p-3 backdrop-blur-[2px] transition-transform duration-100 ease-out xl:block"
		style={`transform: translate3d(${mouseX * 30}px, ${mouseY * 25}px, 0);`}
	>
		<div
			class="mb-2 flex items-center justify-between border-b border-[var(--line)] pb-1.5 font-mono text-[9px] text-[var(--fg-faint)]"
		>
			<span>TANTIVY CORE</span>
			<span class="text-[var(--accent-blue)]">0.22</span>
		</div>
		<div class="flex items-center gap-1.5 font-mono text-[10px] text-[var(--fg-dim)]">
			<span class="h-1.5 w-1.5 bg-[var(--accent-blue)]"></span>
			<span>0.8ms query latency</span>
		</div>
	</div>

	<div
		class="absolute bottom-[24%] left-[12%] hidden w-44 border border-[var(--line)] bg-[var(--surface)]/30 p-2.5 backdrop-blur-[2px] transition-transform duration-100 ease-out lg:block"
		style={`transform: translate3d(${mouseX * 20}px, ${mouseY * -30}px, 0);`}
	>
		<div class="mb-1 font-mono text-[9px] text-[var(--fg-faint)]">RRF FUSION</div>
		<div class="font-mono text-[11px] font-bold text-[var(--accent-purple)]">1/(60 + rank)</div>
	</div>

	<div
		class="absolute right-[14%] bottom-[20%] hidden w-52 border border-[var(--line)] bg-[var(--surface)]/30 p-3 backdrop-blur-[2px] transition-transform duration-100 ease-out lg:block"
		style={`transform: translate3d(${mouseX * -35}px, ${mouseY * 20}px, 0);`}
	>
		<div class="mb-1 flex items-center justify-between font-mono text-[9px] text-[var(--fg-faint)]">
			<span>ROBOTS.TXT</span>
			<span class="text-[var(--accent-green)]">COMPLIANT</span>
		</div>
		<div class="font-mono text-[10px] text-[var(--fg-dim)]">Polite crawler // 1 req/2s</div>
	</div>
</div>
