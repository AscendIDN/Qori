use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceRecord {
    pub id: i64,
    pub kind: String,
    pub uri: String,
    pub label: String,
    pub crawl_depth: i64,
    pub refresh_hours: i64,
    pub enabled: bool,
    pub last_run_at: Option<String>,
    pub doc_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSourceRecord {
    pub kind: String,
    pub uri: String,
    pub label: String,
    pub crawl_depth: i64,
    pub refresh_hours: i64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRecord {
    pub id: i64,
    pub source_id: i64,
    pub url: String,
    pub title: String,
    pub description: String,
    pub content_hash: String,
    pub lang: String,
    pub word_count: i64,
    pub fetched_at: String,
    pub pinned: bool,
    pub tags: Vec<String>,
    pub snippet: Option<String>,
    pub score: Option<f32>,
    pub source_kind: Option<String>,
    pub source_label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewDocumentRecord {
    pub source_id: i64,
    pub url: String,
    pub title: String,
    pub description: String,
    pub content_hash: String,
    pub lang: String,
    pub word_count: i64,
    pub fetched_at: String,
    pub tags: Vec<String>,
}

pub struct DbStore {
    conn: Mutex<Connection>,
    db_path: PathBuf,
}

impl DbStore {
    pub fn new<P: AsRef<Path>>(data_dir: P) -> Result<Self> {
        let dir = data_dir.as_ref();
        std::fs::create_dir_all(dir).ok();
        let legacy_path = dir.join("private_search.db");
        let db_path = dir.join("qori.db");
        if !db_path.exists() && legacy_path.exists() {
            let _ = std::fs::rename(&legacy_path, &db_path);
        }
        let conn = Connection::open(&db_path)?;

        // Enable foreign keys and WAL mode for high concurrent performance
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;",
        )?;

        let store = Self {
            conn: Mutex::new(conn),
            db_path,
        };
        store.run_migrations()?;
        Ok(store)
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA synchronous = NORMAL;",
        )?;
        let store = Self {
            conn: Mutex::new(conn),
            db_path: PathBuf::from(":memory:"),
        };
        store.run_migrations()?;
        Ok(store)
    }

    pub fn run_migrations(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS sources (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                kind TEXT NOT NULL,
                uri TEXT NOT NULL UNIQUE,
                label TEXT,
                crawl_depth INTEGER DEFAULT 1,
                refresh_hours INTEGER DEFAULT 168,
                enabled INTEGER DEFAULT 1,
                last_run_at TEXT
            );

            CREATE TABLE IF NOT EXISTS documents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_id INTEGER REFERENCES sources(id) ON DELETE CASCADE,
                url TEXT NOT NULL UNIQUE,
                title TEXT,
                description TEXT,
                content_hash TEXT,
                lang TEXT,
                favicon BLOB,
                word_count INTEGER DEFAULT 0,
                fetched_at TEXT,
                pinned INTEGER DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL
            );

            CREATE TABLE IF NOT EXISTS document_tags (
                document_id INTEGER REFERENCES documents(id) ON DELETE CASCADE,
                tag_id INTEGER REFERENCES tags(id) ON DELETE CASCADE,
                PRIMARY KEY (document_id, tag_id)
            );

            CREATE TABLE IF NOT EXISTS search_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                query TEXT NOT NULL,
                result_count INTEGER DEFAULT 0,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS document_embeddings (
                document_id INTEGER PRIMARY KEY REFERENCES documents(id) ON DELETE CASCADE,
                embedding_json TEXT NOT NULL,
                model TEXT NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );

            CREATE INDEX IF NOT EXISTS idx_documents_url ON documents(url);
            CREATE INDEX IF NOT EXISTS idx_documents_source ON documents(source_id);
            CREATE INDEX IF NOT EXISTS idx_search_history_query ON search_history(query);
            CREATE INDEX IF NOT EXISTS idx_doc_embeddings_model ON document_embeddings(model);",
        )?;
        Ok(())
    }

    pub fn insert_source(&self, s: &NewSourceRecord) -> Result<SourceRecord> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO sources (kind, uri, label, crawl_depth, refresh_hours, enabled)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(uri) DO UPDATE SET
                label = excluded.label,
                crawl_depth = excluded.crawl_depth,
                refresh_hours = excluded.refresh_hours,
                enabled = excluded.enabled;",
            params![
                s.kind,
                s.uri,
                s.label,
                s.crawl_depth,
                s.refresh_hours,
                s.enabled as i32
            ],
        )?;
        let id = conn.last_insert_rowid();
        Ok(SourceRecord {
            id,
            kind: s.kind.clone(),
            uri: s.uri.clone(),
            label: s.label.clone(),
            crawl_depth: s.crawl_depth,
            refresh_hours: s.refresh_hours,
            enabled: s.enabled,
            last_run_at: None,
            doc_count: 0,
        })
    }

    pub fn get_sources(&self) -> Result<Vec<SourceRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT s.id, s.kind, s.uri, s.label, s.crawl_depth, s.refresh_hours, s.enabled, s.last_run_at,
                    COUNT(d.id) as doc_count
             FROM sources s
             LEFT JOIN documents d ON d.source_id = s.id
             GROUP BY s.id
             ORDER BY s.id DESC",
        )?;

        let rows = stmt.query_map([], |row| {
            let enabled_int: i32 = row.get(6)?;
            Ok(SourceRecord {
                id: row.get(0)?,
                kind: row.get(1)?,
                uri: row.get(2)?,
                label: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                crawl_depth: row.get(4)?,
                refresh_hours: row.get(5)?,
                enabled: enabled_int == 1,
                last_run_at: row.get(7)?,
                doc_count: row.get(8)?,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn toggle_source(&self, id: i64, enabled: bool) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE sources SET enabled = ?1 WHERE id = ?2",
            params![enabled as i32, id],
        )?;
        Ok(())
    }

    pub fn delete_source(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM sources WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn update_source_last_run(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE sources SET last_run_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    pub fn insert_document(&self, doc: &NewDocumentRecord) -> Result<i64> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO documents (source_id, url, title, description, content_hash, lang, word_count, fetched_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(url) DO UPDATE SET
                title = excluded.title,
                description = excluded.description,
                content_hash = excluded.content_hash,
                lang = excluded.lang,
                word_count = excluded.word_count,
                fetched_at = excluded.fetched_at;",
            params![
                doc.source_id,
                doc.url,
                doc.title,
                doc.description,
                doc.content_hash,
                doc.lang,
                doc.word_count,
                doc.fetched_at
            ],
        )?;

        // Retrieve document ID
        let doc_id: i64 = tx.query_row(
            "SELECT id FROM documents WHERE url = ?1",
            params![doc.url],
            |r| r.get(0),
        )?;

        // Insert tags
        for tag in &doc.tags {
            tx.execute(
                "INSERT INTO tags (name) VALUES (?1) ON CONFLICT(name) DO NOTHING;",
                params![tag],
            )?;
            let tag_id: i64 =
                tx.query_row("SELECT id FROM tags WHERE name = ?1", params![tag], |r| {
                    r.get(0)
                })?;
            tx.execute(
                "INSERT INTO document_tags (document_id, tag_id) VALUES (?1, ?2) ON CONFLICT DO NOTHING;",
                params![doc_id, tag_id],
            )?;
        }

        tx.commit()?;
        Ok(doc_id)
    }

    pub fn get_document_by_id(&self, id: i64) -> Result<Option<DocumentRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT d.id, d.source_id, d.url, d.title, d.description, d.content_hash, d.lang,
                    d.word_count, d.fetched_at, d.pinned, s.kind, s.label
             FROM documents d
             LEFT JOIN sources s ON s.id = d.source_id
             WHERE d.id = ?1",
        )?;

        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            let pinned_int: i32 = row.get(9)?;
            let doc_id: i64 = row.get(0)?;

            // Fetch tags
            let mut tag_stmt = conn.prepare(
                "SELECT t.name FROM tags t
                 JOIN document_tags dt ON dt.tag_id = t.id
                 WHERE dt.document_id = ?1",
            )?;
            let tag_rows = tag_stmt.query_map(params![doc_id], |r| r.get(0))?;
            let mut tags = Vec::new();
            for t in tag_rows {
                tags.push(t?);
            }

            Ok(Some(DocumentRecord {
                id: doc_id,
                source_id: row.get(1)?,
                url: row.get(2)?,
                title: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                description: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
                content_hash: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
                lang: row.get::<_, Option<String>>(6)?.unwrap_or_default(),
                word_count: row.get(7)?,
                fetched_at: row.get::<_, Option<String>>(8)?.unwrap_or_default(),
                pinned: pinned_int == 1,
                tags,
                snippet: None,
                score: None,
                source_kind: row.get(10)?,
                source_label: row.get(11)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn get_documents_by_ids(&self, ids: &[i64]) -> Result<Vec<DocumentRecord>> {
        let mut docs = Vec::new();
        for &id in ids {
            if let Some(doc) = self.get_document_by_id(id)? {
                docs.push(doc);
            }
        }
        Ok(docs)
    }

    pub fn set_document_pinned(&self, id: i64, pinned: bool) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE documents SET pinned = ?1 WHERE id = ?2",
            params![pinned as i32, id],
        )?;
        Ok(())
    }

    pub fn record_search(&self, query: &str, count: usize) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO search_history (query, result_count) VALUES (?1, ?2)",
            params![query, count as i64],
        )?;
        Ok(())
    }

    pub fn get_suggestions(&self, prefix: &str) -> Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let pattern = format!("%{}%", prefix.trim());
        let mut results = Vec::new();

        // 1. Check search history
        let mut stmt = conn.prepare(
            "SELECT DISTINCT query FROM search_history
             WHERE query LIKE ?1
             ORDER BY id DESC LIMIT 4",
        )?;
        let rows = stmt.query_map(params![pattern], |r| r.get(0))?;
        for r in rows {
            results.push(r?);
        }

        // 2. Check document titles
        let mut stmt2 = conn.prepare(
            "SELECT DISTINCT title FROM documents
             WHERE title LIKE ?1
             ORDER BY pinned DESC, id DESC LIMIT 4",
        )?;
        let rows2 = stmt2.query_map(params![pattern], |r| r.get(0))?;
        for r in rows2 {
            let t: String = r?;
            if !results.contains(&t) && !t.is_empty() {
                results.push(t);
            }
        }

        // 3. Check tags
        let mut stmt3 = conn.prepare(
            "SELECT name FROM tags
             WHERE name LIKE ?1
             LIMIT 3",
        )?;
        let rows3 = stmt3.query_map(params![pattern], |r| r.get(0))?;
        for r in rows3 {
            let tag: String = r?;
            let tag_query = format!("tag:{}", tag);
            if !results.contains(&tag_query) {
                results.push(tag_query);
            }
        }

        Ok(results)
    }

    pub fn get_counts(&self) -> Result<(i64, i64, i64)> {
        let conn = self.conn.lock().unwrap();
        let total_docs: i64 = conn.query_row("SELECT COUNT(*) FROM documents", [], |r| r.get(0))?;
        let total_sources: i64 =
            conn.query_row("SELECT COUNT(*) FROM sources", [], |r| r.get(0))?;
        let total_searches: i64 =
            conn.query_row("SELECT COUNT(*) FROM search_history", [], |r| r.get(0))?;
        Ok((total_docs, total_sources, total_searches))
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_all_settings(&self) -> Result<std::collections::HashMap<String, String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
        let rows = stmt.query_map([], |r| {
            let k: String = r.get(0)?;
            let v: String = r.get(1)?;
            Ok((k, v))
        })?;
        let mut map = std::collections::HashMap::new();
        for r in rows {
            let (k, v) = r?;
            map.insert(k, v);
        }
        Ok(map)
    }

    pub fn insert_embedding(&self, doc_id: i64, embedding: &[f32], model: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let json = serde_json::to_string(embedding).unwrap_or_else(|_| "[]".to_string());
        conn.execute(
            "INSERT INTO document_embeddings (document_id, embedding_json, model)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(document_id) DO UPDATE SET
                embedding_json = excluded.embedding_json,
                model = excluded.model",
            params![doc_id, json, model],
        )?;
        Ok(())
    }

    pub fn get_all_embeddings(&self, model: &str) -> Result<Vec<(i64, Vec<f32>)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT document_id, embedding_json FROM document_embeddings WHERE model = ?1",
        )?;
        let rows = stmt.query_map(params![model], |row| {
            let id: i64 = row.get(0)?;
            let json: String = row.get(1)?;
            let emb: Vec<f32> = serde_json::from_str(&json).unwrap_or_default();
            Ok((id, emb))
        })?;
        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn get_all_document_texts(&self) -> Result<Vec<(i64, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, description FROM documents")?;
        let rows = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let title: String = row.get(1)?;
            let desc: String = row.get(2)?;
            let combined = if desc.is_empty() {
                title
            } else {
                format!("{}. {}", title, desc)
            };
            Ok((id, combined))
        })?;
        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn get_db_path(&self) -> &Path {
        &self.db_path
    }
}
