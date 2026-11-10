import type {
	SearchMode,
	SearchFilter,
	SearchResponse,
	Source,
	DocumentItem,
	CrawlProgressEvent,
	AppStats,
	SettingsState,
	AiStatus
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

// Zero fake/dummy documents
const MOCK_DOCS: DocumentItem[] = [];

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
			const q = query.trim();
			const start = performance.now();
			if (!q) {
				return { documents: [], total: 0, duration_ms: 0, query };
			}
			try {
				const resp = await fetch(
					`https://en.wikipedia.org/w/api.php?action=opensearch&search=${encodeURIComponent(q)}&limit=10&namespace=0&format=json&origin=*`
				);
				if (resp.ok) {
					const data = await resp.json();
					const titles: string[] = data[1] || [];
					const snippets: string[] = data[2] || [];
					const urls: string[] = data[3] || [];
					const docs: DocumentItem[] = titles.map((title, idx) => ({
						id: idx + 1,
						source_id: 0,
						url: urls[idx] || `https://en.wikipedia.org/wiki/${encodeURIComponent(title)}`,
						title,
						description: snippets[idx] || title,
						content_hash: '',
						lang: 'id',
						word_count: 500,
						fetched_at: new Date().toISOString(),
						pinned: false,
						tags: ['Web'],
						snippet: snippets[idx] || undefined,
						score: 1 / (idx + 1),
						source_kind: 'web',
						source_label: urls[idx] ? new URL(urls[idx]).hostname : 'en.wikipedia.org'
					}));
					return {
						documents: docs,
						total: docs.length,
						duration_ms: Math.round(performance.now() - start),
						query
					};
				}
			} catch (fetchErr) {
				console.warn('Browser fallback search failed:', fetchErr);
			}

			return {
				documents: [],
				total: 0,
				duration_ms: Math.round(performance.now() - start),
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
	},

	async checkAiStatus(): Promise<AiStatus> {
		try {
			return await invokeTauri<AiStatus>('check_ai_status');
		} catch {
			return {
				ollama_available: false,
				ollama_url: 'http://localhost:11434',
				embedding_model: 'nomic-embed-text',
				available_models: [],
				searxng_available: false,
				searxng_url: 'http://localhost:8888'
			};
		}
	},

	async saveSettings(settings: Record<string, string>): Promise<void> {
		try {
			await invokeTauri<void>('save_settings', { settings });
		} catch {
			// Mock local storage fallback
			if (typeof localStorage !== 'undefined') {
				Object.entries(settings).forEach(([k, v]) => localStorage.setItem(`qori_setting_${k}`, v));
			}
		}
	},

	async getSettings(): Promise<Record<string, string>> {
		try {
			return await invokeTauri<Record<string, string>>('get_settings');
		} catch {
			return {};
		}
	}
};
