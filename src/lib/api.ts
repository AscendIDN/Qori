import type {
	SearchMode,
	SearchFilter,
	SearchResponse,
	Source,
	DocumentItem,
	CrawlProgressEvent,
	AppStats,
	SettingsState
} from './types';

function isTauri(): boolean {
	return (
		typeof window !== 'undefined' && ('__TAURI_INTERNALS__' in window || '__TAURI__' in window)
	);
}

async function invokeTauri<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
	if (isTauri()) {
		try {
			const { invoke } = await import('@tauri-apps/api/core');
			return await invoke<T>(cmd, args);
		} catch (err) {
			console.warn(`[Tauri IPC] ${cmd} failed, falling back to mock:`, err);
			throw err;
		}
	}
	throw new Error('Not running inside Tauri');
}

// In-memory demo data for preview / fallback mode
const MOCK_DOCS: DocumentItem[] = [
	{
		id: 1,
		source_id: 1,
		url: 'https://artcraft.com/philosophy/craft-over-code',
		title: 'Craft Over Code: The Architecture of Tactile Interfaces',
		description:
			'An inquiry into neo-brutalist interaction design, micro-density typography, and local-first computing.',
		content_hash: '9f82c448a3',
		lang: 'en',
		word_count: 2480,
		fetched_at: '2026-11-04 14:22:01',
		pinned: true,
		tags: ['Design', 'LocalFirst', 'Philosophy'],
		snippet:
			'Search engines once treated information as a shared commons. By executing indexing <mark>entirely on user hardware</mark>, we reclaim the right to personal memory without intermediary surveillance.',
		score: 0.94,
		source_kind: 'web',
		source_label: 'Design Archive'
	},
	{
		id: 2,
		source_id: 2,
		url: 'file:///C:/Users/jathniel/Documents/Research/Local-First-Software.md',
		title: 'Local-First Software: You Own Your Data, in spite of the Cloud',
		description:
			'Foundational principles of CRDTs, embedded search indices, and offline-first desktop systems.',
		content_hash: '6e11bc92a1',
		lang: 'en',
		word_count: 5120,
		fetched_at: '2026-11-05 09:15:30',
		pinned: false,
		tags: ['Architecture', 'Offline', 'Rust'],
		snippet:
			'Cloud software gives you easy collaboration, but steals ownership. <mark>Qori</mark> embeds Tantivy and SQLite directly in the client binary, yielding sub-millisecond retrieval across millions of words.',
		score: 0.88,
		source_kind: 'folder',
		source_label: 'Research Notes'
	},
	{
		id: 3,
		source_id: 1,
		url: 'https://news.ycombinator.com/item?id=4021942',
		title: 'Show HN: Tantivy – Full-Text Search Engine Library in Rust',
		description:
			'Discussion on Tantivy schema design, fast query evaluation, and Lucene benchmarks.',
		content_hash: '2c44a98b01',
		lang: 'en',
		word_count: 1420,
		fetched_at: '2026-11-03 18:40:12',
		pinned: false,
		tags: ['Rust', 'SearchEngine'],
		snippet:
			'Tantivy delivers competitive indexing throughput while maintaining a minimal memory footprint, making it ideal for <mark>embedded personal indexing</mark> on commodity hardware.',
		score: 0.79,
		source_kind: 'web',
		source_label: 'Tech Bookmarks'
	},
	{
		id: 4,
		source_id: 3,
		url: 'file:///C:/Users/jathniel/Bookmarks/browser_export_2026.html',
		title: 'Reciprocal Rank Fusion (RRF) for Hybrid Vector and BM25 Search',
		description:
			'Mathematical formulation of rank blending without calibrated score normalization.',
		content_hash: '3d55e88f29',
		lang: 'en',
		word_count: 3200,
		fetched_at: '2026-11-02 11:00:44',
		pinned: true,
		tags: ['IR', 'Algorithms', 'HybridSearch'],
		snippet:
			'Combining sparse term frequencies with dense semantic embeddings via <mark>RRF formula 1/(k + r)</mark> provides robust ordering regardless of disparate score distributions.',
		score: 0.74,
		source_kind: 'bookmark',
		source_label: 'Browser Bookmarks'
	}
];

const MOCK_SOURCES: Source[] = [
	{
		id: 1,
		kind: 'web',
		uri: 'https://artcraft.com',
		label: 'ArtCraft & Editorial Tech',
		crawl_depth: 2,
		refresh_hours: 168,
		enabled: true,
		last_run_at: '2026-11-04 14:22:01',
		doc_count: 184
	},
	{
		id: 2,
		kind: 'folder',
		uri: 'C:/Users/jathniel/Documents/Research',
		label: 'Local Research Vault',
		crawl_depth: 3,
		refresh_hours: 24,
		enabled: true,
		last_run_at: '2026-11-05 09:15:30',
		doc_count: 842
	},
	{
		id: 3,
		kind: 'bookmark',
		uri: 'C:/Users/jathniel/Bookmarks/browser_export.html',
		label: 'Chrome & Edge Bookmarks',
		crawl_depth: 1,
		refresh_hours: 720,
		enabled: true,
		last_run_at: '2026-11-02 11:00:44',
		doc_count: 619
	},
	{
		id: 4,
		kind: 'web',
		uri: 'https://docs.rs/tantivy',
		label: 'Rust Tantivy Documentation',
		crawl_depth: 2,
		refresh_hours: 336,
		enabled: true,
		last_run_at: '2026-11-01 19:30:00',
		doc_count: 340
	}
];

let mockDocs = [...MOCK_DOCS];
let mockSources = [...MOCK_SOURCES];

export const api = {
	async search(
		query: string,
		mode: SearchMode = 'keyword',
		filter?: SearchFilter
	): Promise<SearchResponse> {
		try {
			return await invokeTauri<SearchResponse>('search', { query, mode, filter });
		} catch {
			const q = query.trim().toLowerCase();
			const start = performance.now();
			let filtered = mockDocs;

			if (q) {
				filtered = filtered.filter(
					(doc) =>
						doc.title.toLowerCase().includes(q) ||
						doc.description.toLowerCase().includes(q) ||
						doc.url.toLowerCase().includes(q) ||
						doc.tags.some((t) => t.toLowerCase().includes(q))
				);
			}

			if (filter?.kind && filter.kind !== 'all') {
				filtered = filtered.filter((doc) => doc.source_kind === filter.kind);
			}

			if (filter?.tag && filter.tag !== 'all') {
				filtered = filtered.filter((doc) => doc.tags.includes(filter.tag!));
			}

			return {
				documents: filtered,
				total: filtered.length,
				duration_ms: Math.round((performance.now() - start) * 10) / 10 + 1.2,
				query
			};
		}
	},

	async suggest(prefix: string): Promise<string[]> {
		try {
			return await invokeTauri<string[]>('suggest', { prefix });
		} catch {
			if (!prefix.trim())
				return [
					'Local-First Software',
					'Tantivy Search Engine',
					'ArtCraft UI',
					'Reciprocal Rank Fusion'
				];
			const p = prefix.toLowerCase();
			const suggestions = new Set<string>();
			mockDocs.forEach((d) => {
				if (d.title.toLowerCase().includes(p)) suggestions.add(d.title);
				d.tags.forEach((t) => {
					if (t.toLowerCase().includes(p)) suggestions.add(`tag:${t}`);
				});
			});
			return Array.from(suggestions).slice(0, 6);
		}
	},

	async getSources(): Promise<Source[]> {
		try {
			return await invokeTauri<Source[]>('get_sources');
		} catch {
			return mockSources;
		}
	},

	async addSource(source: Omit<Source, 'id' | 'last_run_at' | 'doc_count'>): Promise<Source> {
		try {
			return await invokeTauri<Source>('add_source', { source });
		} catch {
			const newSource: Source = {
				...source,
				id: Date.now(),
				last_run_at: null,
				doc_count: 0
			};
			mockSources = [newSource, ...mockSources];
			return newSource;
		}
	},

	async deleteSource(id: number): Promise<void> {
		try {
			await invokeTauri<void>('delete_source', { id });
		} catch {
			mockSources = mockSources.filter((s) => s.id !== id);
		}
	},

	async toggleSource(id: number, enabled: boolean): Promise<void> {
		try {
			await invokeTauri<void>('toggle_source', { id, enabled });
		} catch {
			mockSources = mockSources.map((s) => (s.id === id ? { ...s, enabled } : s));
		}
	},

	async crawlNow(sourceId: number): Promise<void> {
		try {
			await invokeTauri<void>('crawl_now', { sourceId });
		} catch {
			console.log(`[Mock Crawl] Triggered crawl for source ${sourceId}`);
		}
	},

	async openDocument(urlOrPath: string): Promise<void> {
		try {
			await invokeTauri<void>('open_document', { url: urlOrPath });
		} catch {
			if (urlOrPath.startsWith('http://') || urlOrPath.startsWith('https://')) {
				window.open(urlOrPath, '_blank');
			} else {
				alert(`Opening local document: ${urlOrPath}`);
			}
		}
	},

	async pinDocument(id: number, pinned: boolean): Promise<void> {
		try {
			await invokeTauri<void>('pin_document', { id, pinned });
		} catch {
			mockDocs = mockDocs.map((d) => (d.id === id ? { ...d, pinned } : d));
		}
	},

	async tagDocument(id: number, tags: string[]): Promise<void> {
		try {
			await invokeTauri<void>('tag_document', { id, tags });
		} catch {
			mockDocs = mockDocs.map((d) => (d.id === id ? { ...d, tags } : d));
		}
	},

	async getStats(): Promise<AppStats> {
		try {
			return await invokeTauri<AppStats>('get_stats');
		} catch {
			const totalDocs =
				mockSources.reduce((acc, s) => acc + (s.doc_count || 0), 0) || mockDocs.length;
			return {
				total_documents: totalDocs,
				total_sources: mockSources.length,
				index_size_bytes: 14820412,
				last_crawl_at: '2026-11-05 09:15:30',
				search_count: 87
			};
		}
	},

	async exportLibrary(): Promise<string> {
		try {
			return await invokeTauri<string>('export_library');
		} catch {
			return 'library_backup_20261105.zip';
		}
	},

	async importLibrary(filePath: string): Promise<{ restored_docs: number }> {
		try {
			return await invokeTauri<{ restored_docs: number }>('import_library', { filePath });
		} catch {
			return { restored_docs: mockDocs.length };
		}
	},

	async addManualDocument(doc: {
		url: string;
		title: string;
		body: string;
		tags: string[];
	}): Promise<DocumentItem> {
		try {
			return await invokeTauri<DocumentItem>('add_manual_document', { doc });
		} catch {
			const newDoc: DocumentItem = {
				id: Date.now(),
				source_id: 1,
				url: doc.url,
				title: doc.title,
				description: doc.body.slice(0, 140) + '...',
				content_hash: 'manual_' + Date.now().toString(16),
				lang: 'en',
				word_count: doc.body.split(/\s+/).length,
				fetched_at: new Date().toISOString().replace('T', ' ').slice(0, 19),
				pinned: false,
				tags: doc.tags,
				snippet: doc.body.slice(0, 200),
				score: 1.0,
				source_kind: 'web',
				source_label: 'Manual Entry'
			};
			mockDocs = [newDoc, ...mockDocs];
			return newDoc;
		}
	}
};
