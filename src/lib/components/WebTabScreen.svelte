<script lang="ts">
	import {
		Globe,
		ExternalLink,
		RotateCw,
		Copy,
		Check,
		ShieldCheck,
		ArrowLeft,
		AlertTriangle
	} from 'lucide-svelte';
	import { api } from '../api';

	let {
		url = '',
		title = '',
		onClose = () => {}
	}: {
		url: string;
		title: string;
		onClose?: () => void;
	} = $props();

	let iframeEl = $state<HTMLIFrameElement | null>(null);
	let isCopied = $state(false);
	let isLoading = $state(true);
	let hasError = $state(false);

	function reloadIframe() {
		isLoading = true;
		hasError = false;
		if (iframeEl) {
			iframeEl.src = url;
		}
	}

	function copyUrl() {
		navigator.clipboard.writeText(url);
		isCopied = true;
		setTimeout(() => {
			isCopied = false;
		}, 2000);
	}

	function openInSystemBrowser() {
		api.openDocument(url);
	}

	function handleIframeLoad() {
		isLoading = false;
	}

	function handleIframeError() {
		isLoading = false;
		hasError = true;
	}
</script>

<div class="relative flex h-[calc(100vh-80px)] w-full flex-col bg-[var(--bg-deep)]">
	<!-- Tab Sub-Header & Controls -->
	<div
		class="flex h-10 w-full shrink-0 items-center justify-between border-b border-[var(--line)] bg-[var(--surface)] px-4 font-mono text-xs select-none"
	>
		<!-- Left: Back to Search / Hub -->
		<div class="flex items-center gap-2">
			<button
				onclick={onClose}
				class="flex items-center gap-1.5 border border-[var(--line)] bg-[var(--bg-deep)] px-2.5 py-1 text-[11px] font-bold text-[var(--fg-dim)] uppercase transition-colors hover:border-[var(--fg)] hover:text-[var(--fg)]"
				title="Close web view and return to search"
			>
				<ArrowLeft class="h-3 w-3" />
				<span>BACK</span>
			</button>

			<div class="flex items-center gap-1.5 border border-[var(--line)] bg-[var(--bg-deep)] px-2.5 py-1 text-[10px] text-[var(--accent-green)]">
				<ShieldCheck class="h-3 w-3" />
				<span class="tracking-wider uppercase">SANDBOXED WEB VIEW</span>
			</div>
		</div>

		<!-- Center: Active URL Badge -->
		<div class="flex max-w-md items-center gap-2 truncate px-2 font-mono text-[11px] text-[var(--fg-dim)]">
			<Globe class="h-3 w-3 shrink-0 text-[var(--accent-blue)]" />
			<span class="truncate">{url}</span>
		</div>

		<!-- Right: Reload, Copy, Open External -->
		<div class="flex items-center gap-1.5">
			<button
				onclick={copyUrl}
				class="flex h-7 w-7 items-center justify-center rounded text-[var(--fg-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--fg)]"
				title="Copy URL"
			>
				{#if isCopied}
					<Check class="h-3.5 w-3.5 text-[var(--accent-green)]" />
				{:else}
					<Copy class="h-3.5 w-3.5" />
				{/if}
			</button>

			<button
				onclick={reloadIframe}
				class="flex h-7 w-7 items-center justify-center rounded text-[var(--fg-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--fg)]"
				title="Reload page"
			>
				<RotateCw class={`h-3.5 w-3.5 ${isLoading ? 'animate-spin' : ''}`} />
			</button>

			<button
				onclick={openInSystemBrowser}
				class="flex items-center gap-1 border border-[var(--line)] bg-[var(--fg)] px-2.5 py-1 font-mono text-[10px] font-bold tracking-wider text-[var(--bg)] uppercase transition-colors hover:bg-[var(--fg)]/90"
				title="Open in System Browser (Edge / Chrome)"
			>
				<span>OPEN EXTERNAL</span>
				<ExternalLink class="h-3 w-3 stroke-[2.5]" />
			</button>
		</div>
	</div>

	<!-- Loading bar -->
	{#if isLoading}
		<div class="h-0.5 w-full overflow-hidden bg-[var(--line)]">
			<div class="h-full w-1/3 animate-pulse bg-[var(--accent-blue)]"></div>
		</div>
	{/if}

	<!-- Main Web Content Container -->
	<div class="relative flex-1 w-full bg-white">
		<iframe
			bind:this={iframeEl}
			src={url}
			title={title || 'Web View'}
			onload={handleIframeLoad}
			onerror={handleIframeError}
			class="h-full w-full border-0"
			sandbox="allow-same-origin allow-scripts allow-popups allow-forms allow-modals"
		></iframe>

		<!-- Fallback notice for X-Frame-Options protected sites -->
		<div
			class="pointer-events-none absolute bottom-4 right-4 z-20 max-w-sm border border-[var(--line-strong)] bg-[var(--surface)]/95 p-3.5 font-mono text-xs text-[var(--fg)] shadow-2xl backdrop-blur-md"
		>
			<div class="mb-1 flex items-center gap-1.5 font-bold tracking-wider text-[var(--accent-blue)] uppercase">
				<Globe class="h-3.5 w-3.5" />
				<span>EMBEDDED PREVIEW</span>
			</div>
			<p class="text-[11px] leading-relaxed text-[var(--fg-dim)]">
				If this website prevents cross-origin iframe embedding, click
				<button
					onclick={openInSystemBrowser}
					class="pointer-events-auto inline font-bold text-[var(--accent-blue)] underline underline-offset-2 hover:text-[var(--fg)]"
				>
					OPEN EXTERNAL
				</button>
				to view it directly.
			</p>
		</div>
	</div>
</div>
