<div class="pointer-events-none absolute inset-0 z-0 overflow-hidden select-none">
	<!-- Deep Atmospheric Ambient Glow (Soft depth behind hero) -->
	<div
		class="ambient-glow absolute top-[48%] left-1/2 h-[520px] w-[780px] -translate-x-1/2 -translate-y-1/2 rounded-full opacity-55 blur-[130px]"
		style="background: radial-gradient(ellipse at center, rgba(59, 130, 246, 0.20) 0%, rgba(138, 43, 226, 0.14) 35%, rgba(16, 185, 129, 0.04) 65%, transparent 80%);"
	></div>

	<!-- Architectural Background Grid (Kotak-kotak) -->
	<div
		class="absolute inset-0 opacity-[0.05]"
		style="background-image: linear-gradient(var(--fg) 1px, transparent 1px), linear-gradient(to right, var(--fg) 1px, transparent 1px); background-size: 48px 48px; mask-image: radial-gradient(ellipse 80% 80% at 50% 48%, black 35%, transparent 85%);"
	></div>
</div>

<style>
	@keyframes glowBreathing {
		0%,
		100% {
			opacity: 0.45;
			transform: translate(-50%, -50%) scale(0.98);
		}
		50% {
			opacity: 0.65;
			transform: translate(-50%, -50%) scale(1.03);
		}
	}

	.ambient-glow {
		animation: glowBreathing 14s ease-in-out infinite alternate;
	}
</style>

