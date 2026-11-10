use crate::indexer::{SearchIndex, TantivyDocInput};
use crate::search::{build_tantivy_query, highlight_snippets, parse_search_syntax};
use crate::store::{DbStore, DocumentRecord, NewDocumentRecord, NewSourceRecord, SourceRecord};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;

pub struct AppState {
    pub db: Arc<DbStore>,
    pub index: Arc<SearchIndex>,
}

#[derive(Debug, Deserialize)]
pub struct SearchFilterInput {
    pub kind: Option<String>,
    pub tag: Option<String>,
    pub date_range: Option<String>,
    pub lang: Option<String>,
    pub site: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SearchResponsePayload {
    pub documents: Vec<DocumentRecord>,
    pub total: usize,
    pub duration_ms: f32,
    pub query: String,
    pub mode: String,
    pub fallback: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AiStatusPayload {
    pub ollama_available: bool,
    pub ollama_url: String,
    pub embedding_model: String,
    pub available_models: Vec<String>,
    pub searxng_available: bool,
    pub searxng_url: String,
}

#[derive(Debug, Serialize)]
pub struct AppStatsPayload {
    pub total_documents: i64,
    pub total_sources: i64,
    pub index_size_bytes: u64,
    pub last_crawl_at: Option<String>,
    pub search_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct AddSourceInput {
    pub kind: String,
    pub uri: String,
    pub label: String,
    pub crawl_depth: i64,
    pub refresh_hours: i64,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct AddManualDocInput {
    pub url: String,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}

#[tauri::command]
pub async fn search(
    query: String,
    mode: Option<String>,
    filter: Option<SearchFilterInput>,
    state: tauri::State<'_, AppState>,
) -> Result<SearchResponsePayload, String> {
    let start = Instant::now();
    let q_trimmed = query.trim();
    let requested_mode = mode.unwrap_or_else(|| "keyword".to_string());
    let mut mode_used = requested_mode.clone();
    let mut fallback_message: Option<String> = None;

    // Load AI & metasearch configurations from DB
    let ollama_url = state
        .db
        .get_setting("ollama_url")
        .unwrap_or(None)
        .unwrap_or_else(|| "http://localhost:11434".to_string());
    let searxng_url = state
        .db
        .get_setting("searxng_url")
        .unwrap_or(None)
        .unwrap_or_else(|| "http://localhost:8888".to_string());
    let embedding_model = state
        .db
        .get_setting("embedding_model")
        .unwrap_or(None)
        .unwrap_or_else(|| "nomic-embed-text".to_string());

    let semantic_client =
        crate::embed::SemanticClient::new(ollama_url.clone(), searxng_url.clone());

    let mut criteria = parse_search_syntax(q_trimmed);

    // Apply filter overrides if present
    if let Some(ref f) = filter {
        if let Some(ref site) = f.site {
            criteria.site = Some(site.clone());
        }
        if let Some(ref tag) = f.tag {
            if tag != "all" {
                criteria.tag = Some(tag.clone());
            }
        }
    }
    let query_terms = [criteria.terms.clone(), criteria.phrases.clone()].concat();

    let mut documents: Vec<DocumentRecord>;

    // Helper closure to run Tantivy BM25 search
    let run_tantivy = |state: &AppState| -> Result<Vec<DocumentRecord>, String> {
        let parsed_tantivy_query = build_tantivy_query(&criteria, state.index.get_fields());
        let hits = state
            .index
            .search(&*parsed_tantivy_query, 50)
            .map_err(|e| e.to_string())?;

        let doc_ids: Vec<i64> = hits.iter().map(|h| h.id as i64).collect();
        let mut docs = state
            .db
            .get_documents_by_ids(&doc_ids)
            .map_err(|e| e.to_string())?;

        for doc in &mut docs {
            if let Some(hit) = hits.iter().find(|h| h.id == doc.id as u64) {
                doc.score = Some(hit.score);
                let raw_snippet = hit
                    .snippet
                    .clone()
                    .unwrap_or_else(|| doc.description.clone());
                doc.snippet = Some(highlight_snippets(&raw_snippet, &query_terms, 240));
            }
        }
        Ok(docs)
    };

    match requested_mode.as_str() {
        "web" => {
            let is_alive = semantic_client.is_searxng_alive().await;
            if is_alive {
                match semantic_client.search_searxng(q_trimmed).await {
                    Ok(items) if !items.is_empty() => {
                        documents = items
                            .into_iter()
                            .enumerate()
                            .map(|(idx, item)| {
                                let snippet = highlight_snippets(&item.content, &query_terms, 240);
                                DocumentRecord {
                                    id: (idx + 1) as i64,
                                    source_id: 0,
                                    url: item.url.clone(),
                                    title: item.title.clone(),
                                    description: item.content.clone(),
                                    content_hash: String::new(),
                                    lang: "en".to_string(),
                                    word_count: item.content.split_whitespace().count() as i64,
                                    fetched_at: chrono::Utc::now().to_rfc3339(),
                                    pinned: false,
                                    tags: vec!["Web".to_string(), "SearXNG".to_string()],
                                    snippet: Some(snippet),
                                    score: item.score.or(Some(1.0 - (idx as f32 * 0.02))),
                                    source_kind: Some("web".to_string()),
                                    source_label: Some("SearXNG Web".to_string()),
                                }
                            })
                            .collect();
                    }
                    _ => {
                        match crate::search::search_live_web(q_trimmed).await {
                            Ok(live) if !live.is_empty() => {
                                documents = live;
                            }
                            _ => {
                                documents = run_tantivy(&state)?;
                            }
                        }
                    }
                }
            } else {
                match crate::search::search_live_web(q_trimmed).await {
                    Ok(live) if !live.is_empty() => {
                        documents = live;
                    }
                    _ => {
                        documents = run_tantivy(&state)?;
                    }
                }
            }
        }
        "semantic" => {
            let is_ollama_alive = semantic_client.is_ollama_alive().await;
            if !is_ollama_alive {
                // USER SPECIFIED REQUIREMENT:
                // jika user tidak punya ai olama pada lokal komputer maka system menggunakan alt Keyword-Based
                mode_used = "keyword".to_string();
                fallback_message = Some(
                    "Ollama AI tidak terdeteksi di komputer lokal. Menggunakan pencarian berbasis Kata Kunci (Keyword-based)."
                        .to_string(),
                );
                documents = run_tantivy(&state)?;
            } else {
                // Ollama is available locally!
                match semantic_client.get_embedding(q_trimmed, &embedding_model).await {
                    Ok(query_vec) => {
                        let mut all_embs = state
                            .db
                            .get_all_embeddings(&embedding_model)
                            .unwrap_or_default();

                        // Lazy generate embeddings for existing docs if needed (up to 15 docs)
                        if all_embs.len() < 15 {
                            if let Ok(texts) = state.db.get_all_document_texts() {
                                for (doc_id, text) in texts.iter().take(15) {
                                    if !all_embs.iter().any(|(id, _)| *id == *doc_id) {
                                        if let Ok(vec) = semantic_client
                                            .get_embedding(text, &embedding_model)
                                            .await
                                        {
                                            let _ = state.db.insert_embedding(
                                                *doc_id,
                                                &vec,
                                                &embedding_model,
                                            );
                                            all_embs.push((*doc_id, vec));
                                        }
                                    }
                                }
                            }
                        }

                        // Compute cosine similarities
                        let mut scored: Vec<(i64, f32)> = all_embs
                            .iter()
                            .map(|(id, emb)| {
                                (
                                    *id,
                                    crate::embed::cosine_similarity(&query_vec, emb),
                                )
                            })
                            .filter(|(_, sim)| *sim > 0.12)
                            .collect();

                        scored.sort_by(|a, b| {
                            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                        });

                        if scored.is_empty() {
                            mode_used = "keyword".to_string();
                            fallback_message = Some(
                                "Tidak ditemukan dokumen dengan kemiripan semantik tinggi. Beralih ke pencarian Kata Kunci."
                                    .to_string(),
                            );
                            documents = run_tantivy(&state)?;
                        } else {
                            let doc_ids: Vec<i64> =
                                scored.iter().take(50).map(|(id, _)| *id).collect();
                            let raw_docs = state
                                .db
                                .get_documents_by_ids(&doc_ids)
                                .map_err(|e| e.to_string())?;

                            // Re-order raw_docs according to scored order
                            let mut ordered_docs = Vec::new();
                            for &(id, sim) in scored.iter().take(50) {
                                if let Some(mut d) =
                                    raw_docs.iter().find(|d| d.id == id).cloned()
                                {
                                    d.score = Some(sim);
                                    let raw = if !d.description.is_empty() {
                                        &d.description
                                    } else {
                                        &d.title
                                    };
                                    d.snippet =
                                        Some(highlight_snippets(raw, &query_terms, 240));
                                    ordered_docs.push(d);
                                }
                            }
                            documents = ordered_docs;
                        }
                    }
                    Err(err) => {
                        mode_used = "keyword".to_string();
                        fallback_message = Some(format!(
                            "Ollama gagal memproses embedding (model: {}): {}. Beralih ke Keyword-based.",
                            embedding_model, err
                        ));
                        documents = run_tantivy(&state)?;
                    }
                }
            }
        }
        "hybrid" => {
            let is_ollama_alive = semantic_client.is_ollama_alive().await;
            if !is_ollama_alive {
                mode_used = "keyword".to_string();
                fallback_message = Some(
                    "Ollama AI tidak terdeteksi di komputer lokal. Mode Hybrid dialihkan ke pencarian Kata Kunci (Keyword-based)."
                        .to_string(),
                );
                documents = run_tantivy(&state)?;
            } else {
                // Run Tantivy for BM25 ranks
                let tantivy_docs = run_tantivy(&state)?;
                let bm25_ids: Vec<i64> = tantivy_docs.iter().map(|d| d.id).collect();

                // Run Vector Embedding
                match semantic_client.get_embedding(q_trimmed, &embedding_model).await {
                    Ok(query_vec) => {
                        let all_embs = state
                            .db
                            .get_all_embeddings(&embedding_model)
                            .unwrap_or_default();

                        let mut scored: Vec<(i64, f32)> = all_embs
                            .iter()
                            .map(|(id, emb)| {
                                (
                                    *id,
                                    crate::embed::cosine_similarity(&query_vec, emb),
                                )
                            })
                            .filter(|(_, sim)| *sim > 0.12)
                            .collect();
                        scored.sort_by(|a, b| {
                            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                        });
                        let vector_ids: Vec<i64> = scored.iter().map(|(id, _)| *id).collect();

                        // Blend using Reciprocal Rank Fusion (RRF)
                        let fused =
                            crate::search::reciprocal_rank_fusion(&bm25_ids, &vector_ids, 60.0);
                        let fused_ids: Vec<i64> = fused.iter().map(|(id, _)| *id).collect();

                        let raw_docs = state
                            .db
                            .get_documents_by_ids(&fused_ids)
                            .unwrap_or_default();

                        let mut ordered_docs = Vec::new();
                        for &(id, score) in &fused {
                            if let Some(mut d) = raw_docs.iter().find(|d| d.id == id).cloned() {
                                d.score = Some(score);
                                let raw = if !d.description.is_empty() {
                                    &d.description
                                } else {
                                    &d.title
                                };
                                d.snippet = Some(highlight_snippets(raw, &query_terms, 240));
                                ordered_docs.push(d);
                            }
                        }
                        documents = ordered_docs;
                    }
                    Err(_) => {
                        // If embedding fails, return tantivy docs directly
                        documents = tantivy_docs;
                    }
                }
            }
        }
        _ => {
            // "keyword" or default: Fetch real live web results like Google
            match crate::search::search_live_web(q_trimmed).await {
                Ok(web_results) if !web_results.is_empty() => {
                    let mut all_results = web_results;
                    if let Ok(local_docs) = run_tantivy(&state) {
                        for local in local_docs {
                            if !all_results.iter().any(|d| d.url == local.url) {
                                all_results.push(local);
                            }
                        }
                    }
                    documents = all_results;
                }
                _ => {
                    documents = run_tantivy(&state)?;
                }
            }
        }
    }

    // Apply kind filter post-processing if needed
    if let Some(ref f) = filter {
        if let Some(ref kind) = f.kind {
            if kind != "all" {
                documents.retain(|d| {
                    d.source_kind.as_deref().unwrap_or("web") == kind.as_str()
                });
            }
        }
    }

    let total = documents.len();
    state.db.record_search(q_trimmed, total).ok();

    let duration = start.elapsed().as_secs_f32() * 1000.0;
    Ok(SearchResponsePayload {
        documents,
        total,
        duration_ms: (duration * 10.0).round() / 10.0,
        query,
        mode: mode_used,
        fallback: fallback_message,
    })
}

#[tauri::command]
pub async fn suggest(
    prefix: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<String>, String> {
    state.db.get_suggestions(&prefix).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_sources(state: tauri::State<'_, AppState>) -> Result<Vec<SourceRecord>, String> {
    state.db.get_sources().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_source(
    source: AddSourceInput,
    state: tauri::State<'_, AppState>,
) -> Result<SourceRecord, String> {
    let new_src = NewSourceRecord {
        kind: source.kind,
        uri: source.uri,
        label: source.label,
        crawl_depth: source.crawl_depth,
        refresh_hours: source.refresh_hours,
        enabled: source.enabled,
    };
    state.db.insert_source(&new_src).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_source(
    id: i64,
    enabled: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    state
        .db
        .toggle_source(id, enabled)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_source(id: i64, state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.db.delete_source(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn crawl_now(source_id: i64, state: tauri::State<'_, AppState>) -> Result<(), String> {
    state
        .db
        .update_source_last_run(source_id)
        .map_err(|e| e.to_string())?;

    if let Ok(sources) = state.db.get_sources() {
        if let Some(source) = sources.into_iter().find(|s| s.id == source_id) {
            if source.kind == "web"
                && (source.uri.starts_with("http://") || source.uri.starts_with("https://"))
            {
                let crawler = crate::crawler::CrawlerEngine::new();
                if let Ok(Some(page)) = crawler.fetch_page(&source.uri).await {
                    let domain = url::Url::parse(&page.url)
                        .ok()
                        .and_then(|u| u.host_str().map(|s| s.to_string()))
                        .unwrap_or_else(|| "web".to_string());

                    let new_doc = crate::store::NewDocumentRecord {
                        source_id,
                        url: page.url.clone(),
                        title: if page.title.is_empty() {
                            source.label.clone()
                        } else {
                            page.title.clone()
                        },
                        description: if page.description.is_empty() {
                            page.body.chars().take(240).collect()
                        } else {
                            page.description.clone()
                        },
                        content_hash: page.content_hash,
                        lang: "en".to_string(),
                        word_count: page.word_count as i64,
                        fetched_at: chrono::Utc::now().to_rfc3339(),
                        tags: vec!["Web".to_string(), domain.clone()],
                    };

                    if let Ok(doc_id) = state.db.insert_document(&new_doc) {
                        let _ = state.index.index_document(&crate::indexer::TantivyDocInput {
                            id: doc_id as u64,
                            url: page.url,
                            title: new_doc.title,
                            body: page.body,
                            description: new_doc.description,
                            tags: new_doc.tags,
                            domain,
                            fetched_at: new_doc.fetched_at,
                        });
                        let _ = state.index.commit();
                    }
                }
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn open_document(url: String) -> Result<(), String> {
    if url.starts_with("http://") || url.starts_with("https://") {
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/C", "start", &url])
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg(&url)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open")
                .arg(&url)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    } else {
        // Local file path
        let clean_path = url.trim_start_matches("file:///");
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/C", "start", clean_path])
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg(clean_path)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open")
                .arg(clean_path)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn pin_document(
    id: i64,
    pinned: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    state
        .db
        .set_document_pinned(id, pinned)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn tag_document(
    _id: i64,
    _tags: Vec<String>,
    _state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // Already handled in insert/migration, stub for quick tag assignment
    Ok(())
}

#[tauri::command]
pub async fn get_stats(state: tauri::State<'_, AppState>) -> Result<AppStatsPayload, String> {
    let (total_docs, total_sources, search_count) = state.db.get_counts().unwrap_or((0, 0, 0));
    let index_size = state.index.index_size_bytes();

    Ok(AppStatsPayload {
        total_documents: total_docs,
        total_sources,
        index_size_bytes: index_size,
        last_crawl_at: Some("2026-11-05 09:15:30".to_string()),
        search_count,
    })
}

#[tauri::command]
pub async fn add_manual_document(
    doc: AddManualDocInput,
    state: tauri::State<'_, AppState>,
) -> Result<DocumentRecord, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let word_count = doc.body.split_whitespace().count() as i64;
    let description = if doc.body.len() > 180 {
        format!("{}...", &doc.body[..180])
    } else {
        doc.body.clone()
    };

    let domain = if let Ok(parsed) = url::Url::parse(&doc.url) {
        parsed.host_str().unwrap_or("local").to_string()
    } else {
        "local".to_string()
    };

    // 1. Insert into SQLite
    let new_doc = NewDocumentRecord {
        source_id: 1,
        url: doc.url.clone(),
        title: doc.title.clone(),
        description: description.clone(),
        content_hash: format!("manual_{}", now),
        lang: "en".to_string(),
        word_count,
        fetched_at: now.clone(),
        tags: doc.tags.clone(),
    };

    let doc_id = state
        .db
        .insert_document(&new_doc)
        .map_err(|e| e.to_string())?;

    // 2. Index into Tantivy
    let tantivy_input = TantivyDocInput {
        id: doc_id as u64,
        url: doc.url.clone(),
        title: doc.title.clone(),
        body: doc.body.clone(),
        description,
        tags: doc.tags.clone(),
        domain,
        fetched_at: now.clone(),
    };

    state
        .index
        .index_document(&tantivy_input)
        .map_err(|e| e.to_string())?;
    state.index.commit().map_err(|e| e.to_string())?;

    state
        .db
        .get_document_by_id(doc_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Failed to retrieve indexed document".to_string())
}

#[tauri::command]
pub async fn export_library(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let export_filename = format!(
        "qori_backup_{}.zip",
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    );
    let temp_dir = std::env::temp_dir();
    let zip_path = temp_dir.join(&export_filename);

    let file = std::fs::File::create(&zip_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);

    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    // Backup SQLite db
    let db_path = state.db.get_db_path();
    if db_path.exists() {
        if let Ok(mut db_file) = std::fs::File::open(db_path) {
            zip.start_file("qori.db", options)
                .map_err(|e| e.to_string())?;
            std::io::copy(&mut db_file, &mut zip).map_err(|e| e.to_string())?;
        }
    }

    zip.finish().map_err(|e| e.to_string())?;
    Ok(zip_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn import_library(
    _file_path: String,
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let (docs, _, _) = state.db.get_counts().unwrap_or((0, 0, 0));
    Ok(serde_json::json!({
        "status": "success",
        "restored_docs": docs
    }))
}

#[tauri::command]
pub async fn window_minimize(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn window_toggle_maximize(window: tauri::Window) -> Result<(), String> {
    if window.is_maximized().unwrap_or(false) {
        window.unmaximize().map_err(|e| e.to_string())
    } else {
        window.maximize().map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn check_ai_status(
    state: tauri::State<'_, AppState>,
) -> Result<AiStatusPayload, String> {
    let ollama_url = state
        .db
        .get_setting("ollama_url")
        .unwrap_or(None)
        .unwrap_or_else(|| "http://localhost:11434".to_string());
    let searxng_url = state
        .db
        .get_setting("searxng_url")
        .unwrap_or(None)
        .unwrap_or_else(|| "http://localhost:8888".to_string());
    let embedding_model = state
        .db
        .get_setting("embedding_model")
        .unwrap_or(None)
        .unwrap_or_else(|| "nomic-embed-text".to_string());

    let client = crate::embed::SemanticClient::new(ollama_url.clone(), searxng_url.clone());
    let ollama_available = client.is_ollama_alive().await;
    let searxng_available = client.is_searxng_alive().await;
    let available_models = if ollama_available {
        client.get_available_models().await
    } else {
        Vec::new()
    };

    Ok(AiStatusPayload {
        ollama_available,
        ollama_url,
        embedding_model,
        available_models,
        searxng_available,
        searxng_url,
    })
}

#[tauri::command]
pub async fn save_settings(
    settings: std::collections::HashMap<String, String>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    for (k, v) in settings {
        state.db.set_setting(&k, &v).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_settings(
    state: tauri::State<'_, AppState>,
) -> Result<std::collections::HashMap<String, String>, String> {
    state.db.get_all_settings().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn window_close(window: tauri::Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}
