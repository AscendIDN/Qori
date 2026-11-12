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

<div class="relative flex h-[calc(100vh-112px)] w-full flex-col overflow-hidden rounded-[var(--radius-lg)] bg-[var(--surface)] border border-[var(--border)] shadow-sm">
	<!-- Tab Sub-Header & Controls -->
	<div
		class="flex h-10 w-full shrink-0 items-center justify-between border-b border-[var(--border)] bg-[var(--surface)] px-4 text-xs select-none"
	>
		<!-- Left: Back to Search / Hub -->
		<div class="flex items-center gap-2">
			<button
				onclick={onClose}
				class="flex items-center gap-1.5 rounded-full px-3 py-1 text-xs font-semibold text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)] transition-colors"
				title="Close web view and return to start page"
			>
				<ArrowLeft class="h-3.5 w-3.5" />
				<span>Back</span>
			</button>

			<div class="flex items-center gap-1.5 rounded-full bg-emerald-500/10 border border-emerald-500/20 px-2.5 py-0.5 text-[10px] text-emerald-400 font-medium">
				<ShieldCheck class="h-3 w-3" />
				<span>Sandboxed Preview</span>
			</div>
		</div>

		<!-- Center: Active URL Badge -->
		<div class="flex max-w-md items-center gap-2 truncate px-2 font-mono text-[11px] text-[var(--text-dim)]">
			<Globe class="h-3 w-3 shrink-0 text-[var(--accent)]" />
			<span class="truncate">{url}</span>
		</div>

		<!-- Right: Reload, Copy, Open External -->
		<div class="flex items-center gap-1.5">
			<button
				onclick={copyUrl}
				class="flex h-7 w-7 items-center justify-center rounded-full text-[var(--text-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--text)]"
				title="Copy URL"
			>
				{#if isCopied}
					<Check class="h-3.5 w-3.5 text-emerald-400" />
				{:else}
					<Copy class="h-3.5 w-3.5" />
				{/if}
			</button>

			<button
				onclick={reloadIframe}
				class="flex h-7 w-7 items-center justify-center rounded-full text-[var(--text-dim)] transition-colors hover:bg-[var(--surface-hover)] hover:text-[var(--text)]"
				title="Reload page"
			>
				<RotateCw class={`h-3.5 w-3.5 ${isLoading ? 'animate-spin text-[var(--accent)]' : ''}`} />
			</button>

			<button
				onclick={openInSystemBrowser}
				class="flex items-center gap-1.5 rounded-full bg-[var(--accent)] px-3 py-1 text-xs font-semibold text-white shadow-xs hover:bg-[var(--accent-hover)] transition-all"
				title="Open in System Browser"
			>
				<span>Open in Browser</span>
				<ExternalLink class="h-3 w-3" />
			</button>
		</div>
	</div>

	<!-- Loading bar -->
	{#if isLoading}
		<div class="h-0.5 w-full overflow-hidden bg-[var(--border)]">
			<div class="h-full w-1/3 animate-pulse bg-[var(--accent)]"></div>
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

		<!-- Notice for X-Frame-Options protected sites -->
		<div
			class="pointer-events-none absolute bottom-4 right-4 z-20 max-w-sm rounded-[var(--radius-md)] border border-[var(--border)] bg-[var(--surface)]/95 p-3.5 text-xs text-[var(--text)] shadow-[var(--shadow-popover)] backdrop-blur-md"
		>
			<div class="mb-1 flex items-center gap-1.5 font-bold text-[var(--accent)]">
				<Globe class="h-3.5 w-3.5" />
				<span>Web Security Notice</span>
			</div>
			<p class="text-[11px] leading-relaxed text-[var(--text-dim)]">
				If this website prevents embedded previewing via headers, click
				<button
					onclick={openInSystemBrowser}
					class="pointer-events-auto inline font-bold text-[var(--accent)] underline underline-offset-2 hover:opacity-80"
				>
					Open in Browser
				</button>
				to view it directly.
			</p>
		</div>
	</div>
</div>
