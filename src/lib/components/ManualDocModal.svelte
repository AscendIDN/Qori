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
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4 backdrop-blur-md select-none"
		onclick={(e) => {
			if (e.target === e.currentTarget) isOpen = false;
		}}
		role="presentation"
	>
		<div
			class="w-full max-w-xl rounded-[var(--radius-lg)] bg-[var(--surface)] border border-[var(--border)] shadow-[var(--shadow-popover)] overflow-hidden text-[var(--text)]"
		>
			<!-- Header -->
			<div class="flex items-center justify-between border-b border-[var(--border)] px-6 py-4">
				<div class="flex items-center gap-2 font-bold text-sm text-[var(--text)]">
					<FileText class="h-4 w-4 text-[var(--accent)]" />
					<span>Index Manual Document into Tantivy Vault</span>
				</div>
				<button
					onclick={() => (isOpen = false)}
					class="p-1 rounded-full text-[var(--text-dim)] hover:text-[var(--text)] hover:bg-[var(--surface-hover)] transition-colors"
					aria-label="Close"
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
				class="flex flex-col gap-4 p-6 text-xs"
			>
				<div>
					<label for="doc-title" class="mb-1.5 block text-[10px] font-bold tracking-wider text-[var(--text-faint)] uppercase">
						Document Title
					</label>
					<input
						id="doc-title"
						bind:value={title}
						required
						type="text"
						placeholder="e.g. Tantivy Inverted Index Internals"
						class="w-full rounded-[var(--radius-sm)] border border-[var(--border)] bg-[var(--surface-hover)] px-3 py-2 text-xs text-[var(--text)] focus:border-[var(--accent)] focus:outline-none"
					/>
				</div>

				<div>
					<label for="doc-url" class="mb-1.5 block text-[10px] font-bold tracking-wider text-[var(--text-faint)] uppercase">
						Canonical URL / File Path (Optional)
					</label>
					<input
						id="doc-url"
						bind:value={url}
						type="text"
						placeholder="https://... or C:/path/to/doc.md"
						class="w-full rounded-[var(--radius-sm)] border border-[var(--border)] bg-[var(--surface-hover)] px-3 py-2 text-xs text-[var(--text)] focus:border-[var(--accent)] focus:outline-none"
					/>
				</div>

				<div>
					<label for="doc-body" class="mb-1.5 block text-[10px] font-bold tracking-wider text-[var(--text-faint)] uppercase">
						Document Body / Content (Indexed into Full-Text Tantivy)
					</label>
					<textarea
						id="doc-body"
						bind:value={body}
						required
						rows="6"
						placeholder="Paste notes, articles, research excerpts, or code markdown here..."
						class="w-full rounded-[var(--radius-sm)] border border-[var(--border)] bg-[var(--surface-hover)] p-3 text-xs leading-relaxed text-[var(--text)] focus:border-[var(--accent)] focus:outline-none"
					></textarea>
				</div>

				<div>
					<label for="doc-tags" class="mb-1.5 block text-[10px] font-bold tracking-wider text-[var(--text-faint)] uppercase">
						Tags (Comma Separated)
					</label>
					<input
						id="doc-tags"
						bind:value={tagsStr}
						type="text"
						placeholder="Rust, Algorithms, Architecture, Notes"
						class="w-full rounded-[var(--radius-sm)] border border-[var(--border)] bg-[var(--surface-hover)] px-3 py-2 text-xs text-[var(--text)] focus:border-[var(--accent)] focus:outline-none"
					/>
				</div>

				<div class="flex items-center justify-end gap-2 border-t border-[var(--border)] pt-4 mt-2">
					<button
						type="button"
						onclick={() => (isOpen = false)}
						class="px-4 py-2 rounded-full text-xs font-semibold text-[var(--text-dim)] hover:bg-[var(--surface-hover)] transition-colors"
					>
						Cancel
					</button>
					<button
						type="submit"
						disabled={isSaving}
						class="flex items-center gap-1.5 rounded-full bg-[var(--accent)] px-5 py-2 text-xs font-semibold text-white shadow-sm hover:bg-[var(--accent-hover)] transition-all disabled:opacity-50"
					>
						<Plus class="h-3.5 w-3.5" />
						<span>{isSaving ? 'Indexing...' : 'Save & Index Document'}</span>
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
