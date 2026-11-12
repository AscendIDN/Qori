<script lang="ts">
	import {
		Compass,
		Search,
		Shield,
		ShieldCheck,
		Camera,
		Download,
		Menu,
		Plus,
		Star,
		Heart,
		Clock,
		ExternalLink,
		Bookmark,
		Check,
		Sun,
		Moon
	} from 'lucide-svelte';

	let theme = $state<'dark' | 'light'>('dark');

	function toggleTheme() {
		theme = theme === 'dark' ? 'light' : 'dark';
		document.documentElement.setAttribute('data-theme', theme);
	}
</script>

<div class="min-h-screen bg-[var(--surface)] text-[var(--text)] p-8 overflow-y-auto select-none">
	<div class="max-w-5xl mx-auto space-y-12">
		<!-- Header -->
		<div class="flex items-center justify-between border-b border-[var(--border)] pb-6">
			<div>
				<div class="flex items-center gap-2 mb-1">
					<div class="h-6 w-6 rounded-[var(--radius-sm)] bg-gradient-to-tr from-[var(--accent)] to-[#C084FC] flex items-center justify-center text-white font-bold text-xs">
						Q
					</div>
					<h1 class="text-xl font-bold tracking-tight">QORI Design System & Style Guide</h1>
				</div>
				<p class="text-xs text-[var(--text-dim)]">
					Tokens and component showcase inspired by modern browser chrome (Opera / Arc) adhering to Antislop rules.
				</p>
			</div>

			<button
				onclick={toggleTheme}
				class="flex items-center gap-2 px-4 py-2 rounded-full bg-[var(--surface-hover)] border border-[var(--border)] text-xs font-semibold hover:border-[var(--accent)] transition-all shadow-sm"
			>
				{#if theme === 'dark'}
					<Sun class="h-4 w-4 text-amber-400" />
					<span>Switch to Light Mode</span>
				{:else}
					<Moon class="h-4 w-4 text-[var(--accent)]" />
					<span>Switch to Dark Mode</span>
				{/if}
			</button>
		</div>

		<!-- 1. Radii Hierarchy -->
		<section class="space-y-4">
			<h2 class="text-sm font-bold uppercase tracking-wider text-[var(--text-faint)]">1. Radii Hierarchy (Purposeful, Not Slop)</h2>
			<div class="grid grid-cols-1 sm:grid-cols-4 gap-4">
				<div class="p-4 rounded-[var(--radius-sm)] bg-[var(--surface-hover)] border border-[var(--border)] text-center">
					<div class="font-bold text-sm">10px (--radius-sm)</div>
					<div class="text-xs text-[var(--text-dim)] mt-1">Buttons & Controls</div>
				</div>
				<div class="p-4 rounded-[var(--radius-md)] bg-[var(--surface-hover)] border border-[var(--border)] text-center">
					<div class="font-bold text-sm">14px (--radius-md)</div>
					<div class="text-xs text-[var(--text-dim)] mt-1">Cards & Speed Dial</div>
				</div>
				<div class="p-4 rounded-[var(--radius-lg)] bg-[var(--surface-hover)] border border-[var(--border)] text-center">
					<div class="font-bold text-sm">18px (--radius-lg)</div>
					<div class="text-xs text-[var(--text-dim)] mt-1">Main Content & Modals</div>
				</div>
				<div class="p-4 rounded-full bg-[var(--surface-hover)] border border-[var(--border)] text-center">
					<div class="font-bold text-sm">999px (--radius-pill)</div>
					<div class="text-xs text-[var(--text-dim)] mt-1">Search & Status Badges</div>
				</div>
			</div>
		</section>

		<!-- 2. Typography Hierarchy -->
		<section class="space-y-4">
			<h2 class="text-sm font-bold uppercase tracking-wider text-[var(--text-faint)]">2. Typography & Accent Flourish</h2>
			<div class="p-6 rounded-[var(--radius-md)] bg-[var(--surface-hover)] border border-[var(--border)] space-y-3">
				<div>
					<span class="text-xs text-[var(--text-faint)] block mb-1">Display & Headings: Plus Jakarta Sans</span>
					<h3 class="text-2xl font-extrabold tracking-tight">Personal Search Engine</h3>
				</div>
				<div>
					<span class="text-xs text-[var(--text-faint)] block mb-1">Tagline with Instrument Serif Accent Word:</span>
					<p class="text-sm text-[var(--text-dim)]">
						Personal search that <span class="tagline-accent text-lg text-[var(--text)] italic">belongs</span> entirely to you.
					</p>
				</div>
				<div>
					<span class="text-xs text-[var(--text-faint)] block mb-1">Data / Monospace: JetBrains Mono</span>
					<div class="font-mono text-xs text-[var(--accent)] bg-[var(--surface)] p-2 rounded-[var(--radius-sm)] border border-[var(--border)] inline-block">
						Ctrl + K  •  128 MB RAM  •  https://qori.local
					</div>
				</div>
			</div>
		</section>

		<!-- 3. Key Components -->
		<section class="space-y-6">
			<h2 class="text-sm font-bold uppercase tracking-wider text-[var(--text-faint)]">3. Browser Chrome Component Sampler</h2>

			<!-- Tab Pills -->
			<div class="space-y-2">
				<span class="text-xs text-[var(--text-dim)]">Top Bar Tab Pills:</span>
				<div class="flex items-center gap-2 p-2 rounded-[var(--radius-md)] bg-[var(--surface-hover)] border border-[var(--border)]">
					<div class="flex items-center gap-2 px-3 py-1.5 rounded-full bg-[var(--surface)] text-xs font-semibold border border-[var(--border)] shadow-xs">
						<Compass class="h-3.5 w-3.5 text-[var(--text-dim)]" />
						<span>Start Page</span>
					</div>
					<div class="flex items-center gap-2 px-3 py-1.5 rounded-full text-xs font-medium text-[var(--text-dim)] hover:bg-[var(--accent-soft)]">
						<Search class="h-3.5 w-3.5 text-[var(--accent)]" />
						<span>Rust Tantivy</span>
					</div>
					<div class="h-7 w-7 rounded-full flex items-center justify-center text-[var(--text-dim)] hover:bg-[var(--surface)]">
						<Plus class="h-4 w-4" />
					</div>
				</div>
			</div>

			<!-- Large Search Pill -->
			<div class="space-y-2">
				<span class="text-xs text-[var(--text-dim)]">Start Page Search Pill (~840px):</span>
				<div class="flex h-[56px] w-full items-center gap-3 rounded-full bg-[var(--surface)] px-4 shadow-[var(--shadow-search)] border border-[var(--border)]">
					<div class="h-8 w-8 rounded-full bg-gradient-to-tr from-[var(--accent)] to-[#C084FC] flex items-center justify-center text-white font-bold text-xs">
						Q
					</div>
					<span class="flex-1 text-sm text-[var(--text-dim)]">Search your library or the web...</span>
					<div class="h-8 px-3 rounded-full bg-[var(--accent)] text-white flex items-center gap-1.5 text-xs font-semibold shadow-xs">
						<Search class="h-3.5 w-3.5" />
						<span>Search</span>
					</div>
				</div>
			</div>

			<!-- Badges & Tools -->
			<div class="space-y-2">
				<span class="text-xs text-[var(--text-dim)]">Status Badges & Pill Buttons:</span>
				<div class="flex flex-wrap items-center gap-3">
					<div class="flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-[var(--accent)] text-white font-mono text-[11px] font-bold">
						<Shield class="h-3.5 w-3.5" />
						<span>VPN ACTIVE</span>
					</div>
					<div class="flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-emerald-500/15 border border-emerald-500/30 text-emerald-400 font-mono text-[11px] font-bold">
						<ShieldCheck class="h-3.5 w-3.5" />
						<span>AIR-GAPPED SYSTEM</span>
					</div>
					<div class="flex items-center gap-1.5 px-3 py-1 rounded-full bg-[var(--surface-hover)] border border-[var(--border)] text-xs font-medium">
						<Download class="h-3.5 w-3.5" />
						<span>Export Vault</span>
					</div>
					<div class="flex items-center gap-1.5 px-3 py-1 rounded-full bg-[var(--surface-hover)] border border-[var(--border)] text-xs font-medium">
						<Camera class="h-3.5 w-3.5" />
						<span>Screenshot</span>
					</div>
				</div>
			</div>
		</section>
	</div>
</div>

