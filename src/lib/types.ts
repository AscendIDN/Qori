export type SearchMode = 'keyword' | 'semantic' | 'hybrid' | 'web';

export interface BrowserTab {
	id: string;
	title: string;
	page: 'search' | 'results' | 'settings' | 'web';
	query: string;
	url?: string;
	settingsSection?: 'general' | 'library' | 'sources';
}

export interface Source {
	id: number;
	kind: 'web' | 'folder' | 'bookmark' | 'history';
	uri: string;
	label: string;
	crawl_depth: number;
	refresh_hours: number;
	enabled: boolean;
	last_run_at: string | null;
	doc_count?: number;
}

export interface DocumentItem {
	id: number;
	source_id: number;
	url: string;
	title: string;
	description: string;
	content_hash: string;
	lang: string;
	word_count: number;
	fetched_at: string;
	pinned: boolean;
	tags: string[];
	snippet?: string;
	score?: number;
	source_kind?: string;
	source_label?: string;
}

export interface SearchFilter {
	kind?: string;
	tag?: string;
	date_range?: string;
	lang?: string;
	site?: string;
}

export interface SearchResponse {
	documents: DocumentItem[];
	total: number;
	duration_ms: number;
	query: string;
	mode?: string;
	fallback?: string | null;
}

export interface AiStatus {
	ollama_available: boolean;
	ollama_url: string;
	embedding_model: string;
	available_models: string[];
	searxng_available: boolean;
	searxng_url: string;
}

export interface CrawlProgressEvent {
	source_id?: number;
	url: string;
	status: 'fetching' | 'indexed' | 'skipped' | 'error' | 'done';
	message: string;
	timestamp: string;
	current?: number;
	total?: number;
}

export interface AppStats {
	total_documents: number;
	total_sources: number;
	index_size_bytes: number;
	last_crawl_at: string | null;
	search_count: number;
}

export interface SettingsState {
	theme: 'dark' | 'light';
	fontSize: 'normal' | 'large' | 'compact';
	searxngUrl: string;
	ollamaUrl: string;
	embeddingModel: string;
	autoCrawl: boolean;
	dataDir: string;
}
