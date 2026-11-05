<script lang="ts">
	import { X, Plus, FileText, Check } from 'lucide-svelte';
	import { api } from '../api';

	let {
		isOpen = $bindable(false),
		onAdded = () => {}
	}: {
		isOpen: boolean;
		onAdded?: () => void;
	} = $props();

	let title = $state('');
	let url = $state('');
	let body = $state('');
	let tagsStr = $state('');
	let isSaving = $state(false);

	async function handleSave() {
		if (!title.trim() || !body.trim()) return;
		isSaving = true;
		try {
			const tags = tagsStr
				.split(',')
				.map((t) => t.trim())
				.filter(Boolean);

			await api.addManualDocument({
				title: title.trim(),
				url:
					url.trim() ||
					`manual://${encodeURIComponent(title.trim().toLowerCase().replace(/\s+/g, '-'))}`,
				body: body.trim(),
				tags
			});

			title = '';
			url = '';
			body = '';
			tagsStr = '';
			isOpen = false;
			onAdded();
		} catch (err) {
			console.error('Failed to add manual doc:', err);
		} finally {
			isSaving = false;
		}
	}
</script>

{#if isOpen}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/75 p-4 backdrop-blur-sm"
		onclick={(e) => {
			if (e.target === e.currentTarget) isOpen = false;
		}}
		role="presentation"
	>
		<div
			class="animate-in fade-in zoom-in-95 w-full max-w-xl border border-[var(--line-strong)] bg-[var(--surface)] shadow-2xl duration-150"
		>
			<!-- Header -->
			<div
				class="flex items-center justify-between border-b border-[var(--line)] px-5 py-3.5 font-mono text-xs"
			>
				<div class="flex items-center gap-2 font-bold tracking-widest text-[var(--fg)] uppercase">
					<FileText class="h-4 w-4 text-[var(--accent-green)]" />
					<span>INDEX MANUAL DOCUMENT</span>
				</div>
				<button
					onclick={() => (isOpen = false)}
					class="text-[var(--fg-dim)] transition-colors hover:text-[var(--fg)]"
				>
					<X class="h-4 w-4" />
				</button>
			</div>

			<!-- Form Body -->
			<form
				onsubmit={(e) => {
					e.preventDefault();
					handleSave();
				}}
				class="flex flex-col gap-4 p-5 font-mono text-xs"
			>
				<div>
					<label class="mb-1 block text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">
						DOCUMENT TITLE
					</label>
					<input
						bind:value={title}
						required
						type="text"
						placeholder="e.g. Tantivy Inverted Index Internals"
						class="w-full border border-[var(--line)] bg-[var(--bg)] px-3 py-2 text-[var(--fg)] placeholder:text-[var(--fg-faint)] focus:border-[var(--fg)] focus:outline-none"
					/>
				</div>

				<div>
					<label class="mb-1 block text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">
						CANONICAL URL / FILE PATH (OPTIONAL)
					</label>
					<input
						bind:value={url}
						type="text"
						placeholder="https://... or C:/path/to/doc.md"
						class="w-full border border-[var(--line)] bg-[var(--bg)] px-3 py-2 text-[var(--fg)] placeholder:text-[var(--fg-faint)] focus:border-[var(--fg)] focus:outline-none"
					/>
				</div>

				<div>
					<label class="mb-1 block text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">
						DOCUMENT BODY / CONTENT (INDEXED INTO TANTIVY FULL-TEXT)
					</label>
					<textarea
						bind:value={body}
						required
						rows="6"
						placeholder="Paste text, notes, research excerpts, or documentation here..."
						class="w-full border border-[var(--line)] bg-[var(--bg)] p-3 leading-relaxed text-[var(--fg)] placeholder:text-[var(--fg-faint)] focus:border-[var(--fg)] focus:outline-none"
					></textarea>
				</div>

				<div>
					<label class="mb-1 block text-[10px] tracking-widest text-[var(--fg-faint)] uppercase">
						TAGS (COMMA SEPARATED)
					</label>
					<input
						bind:value={tagsStr}
						type="text"
						placeholder="Rust, Algorithms, Architecture, Notes"
						class="w-full border border-[var(--line)] bg-[var(--bg)] px-3 py-2 text-[var(--fg)] placeholder:text-[var(--fg-faint)] focus:border-[var(--fg)] focus:outline-none"
					/>
				</div>

				<div class="flex items-center justify-end gap-3 border-t border-[var(--line)] pt-3">
					<button
						type="button"
						onclick={() => (isOpen = false)}
						class="border border-[var(--line)] bg-transparent px-4 py-2 font-mono text-xs text-[var(--fg-dim)] uppercase hover:border-[var(--line-strong)]"
					>
						CANCEL
					</button>
					<button
						type="submit"
						disabled={isSaving}
						class="flex items-center gap-1.5 bg-[var(--fg)] px-4 py-2 font-mono text-xs font-bold text-[var(--bg)] uppercase hover:bg-[var(--fg)]/90 disabled:opacity-50"
					>
						<Plus class="h-3.5 w-3.5 stroke-[2.5]" />
						<span>{isSaving ? 'INDEXING...' : 'INDEX DOCUMENT'}</span>
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
