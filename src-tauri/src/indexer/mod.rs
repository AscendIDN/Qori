use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::query::{Query, QueryParser};
use tantivy::schema::*;
use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy, Term};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Tantivy error: {0}")]
    Tantivy(#[from] tantivy::TantivyError),
    #[error("Open directory error: {0}")]
    OpenDir(#[from] tantivy::directory::error::OpenDirectoryError),
    #[error("Query parse error: {0}")]
    QueryParse(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct TantivyFields {
    pub id: Field,
    pub url: Field,
    pub title: Field,
    pub body: Field,
    pub description: Field,
    pub tags: Field,
    pub domain: Field,
    pub fetched_at: Field,
}

pub struct SearchIndex {
    index: Index,
    reader: IndexReader,
    writer: Mutex<IndexWriter>,
    fields: TantivyFields,
    index_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct TantivyDocInput {
    pub id: u64,
    pub url: String,
    pub title: String,
    pub body: String,
    pub description: String,
    pub tags: Vec<String>,
    pub domain: String,
    pub fetched_at: String,
}

#[derive(Debug, Clone)]
pub struct SearchHit {
    pub id: u64,
    pub score: f32,
    pub snippet: Option<String>,
}

impl SearchIndex {
    pub fn new<P: AsRef<Path>>(data_dir: P) -> Result<Self, IndexerError> {
        let index_dir = data_dir.as_ref().join("tantivy_index");
        std::fs::create_dir_all(&index_dir)?;

        let mut schema_builder = Schema::builder();
        let id_field = schema_builder.add_u64_field("id", INDEXED | STORED | FAST);
        let url_field = schema_builder.add_text_field("url", STRING | STORED);
        let title_field = schema_builder.add_text_field("title", TEXT | STORED);
        let body_field = schema_builder.add_text_field("body", TEXT | STORED);
        let desc_field = schema_builder.add_text_field("description", TEXT | STORED);
        let tags_field = schema_builder.add_text_field("tags", TEXT | STORED);
        let domain_field = schema_builder.add_text_field("domain", STRING | STORED);
        let fetched_at_field = schema_builder.add_text_field("fetched_at", STORED);

        let schema = schema_builder.build();

        let dir = MmapDirectory::open(&index_dir)?;
        let index = Index::open_or_create(dir, schema)?;

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        let writer = index.writer(50_000_000)?; // 50MB memory buffer

        let fields = TantivyFields {
            id: id_field,
            url: url_field,
            title: title_field,
            body: body_field,
            description: desc_field,
            tags: tags_field,
            domain: domain_field,
            fetched_at: fetched_at_field,
        };

        Ok(Self {
            index,
            reader,
            writer: Mutex::new(writer),
            fields,
            index_dir,
        })
    }

    pub fn in_memory() -> Result<Self, IndexerError> {
        let mut schema_builder = Schema::builder();
        let id_field = schema_builder.add_u64_field("id", INDEXED | STORED | FAST);
        let url_field = schema_builder.add_text_field("url", STRING | STORED);
        let title_field = schema_builder.add_text_field("title", TEXT | STORED);
        let body_field = schema_builder.add_text_field("body", TEXT | STORED);
        let desc_field = schema_builder.add_text_field("description", TEXT | STORED);
        let tags_field = schema_builder.add_text_field("tags", TEXT | STORED);
        let domain_field = schema_builder.add_text_field("domain", STRING | STORED);
        let fetched_at_field = schema_builder.add_text_field("fetched_at", STORED);

        let schema = schema_builder.build();
        let index = Index::create_in_ram(schema);

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        let writer = index.writer(15_000_000)?;

        let fields = TantivyFields {
            id: id_field,
            url: url_field,
            title: title_field,
            body: body_field,
            description: desc_field,
            tags: tags_field,
            domain: domain_field,
            fetched_at: fetched_at_field,
        };

        Ok(Self {
            index,
            reader,
            writer: Mutex::new(writer),
            fields,
            index_dir: PathBuf::from("ram"),
        })
    }

    pub fn index_document(&self, doc_input: &TantivyDocInput) -> Result<(), IndexerError> {
        let writer = self.writer.lock().unwrap();

        // Delete existing document with same ID to prevent duplicates
        let term = Term::from_field_u64(self.fields.id, doc_input.id);
        writer.delete_term(term);

        let tags_joined = doc_input.tags.join(" ");

        let mut doc = TantivyDocument::default();
        doc.add_u64(self.fields.id, doc_input.id);
        doc.add_text(self.fields.url, &doc_input.url);
        doc.add_text(self.fields.title, &doc_input.title);
        doc.add_text(self.fields.body, &doc_input.body);
        doc.add_text(self.fields.description, &doc_input.description);
        doc.add_text(self.fields.tags, &tags_joined);
        doc.add_text(self.fields.domain, &doc_input.domain);
        doc.add_text(self.fields.fetched_at, &doc_input.fetched_at);

        writer.add_document(doc)?;
        Ok(())
    }

    pub fn commit(&self) -> Result<(), IndexerError> {
        let mut writer = self.writer.lock().unwrap();
        writer.commit()?;
        self.reader.reload()?;
        Ok(())
    }

    pub fn search(
        &self,
        parsed_query: &dyn Query,
        limit: usize,
    ) -> Result<Vec<SearchHit>, IndexerError> {
        let searcher = self.reader.searcher();
        let top_docs = searcher.search(parsed_query, &TopDocs::with_limit(limit))?;

        let mut hits = Vec::new();
        for (score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;

            if let Some(id_val) = retrieved_doc
                .get_first(self.fields.id)
                .and_then(|v| v.as_u64())
            {
                // Generate simple snippet from body
                let body_text = retrieved_doc
                    .get_first(self.fields.body)
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();

                let desc_text = retrieved_doc
                    .get_first(self.fields.description)
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();

                let snippet = if !body_text.is_empty() {
                    let preview: String = body_text.chars().take(240).collect();
                    Some(format!("{}...", preview.trim()))
                } else if !desc_text.is_empty() {
                    Some(desc_text.to_string())
                } else {
                    None
                };

                hits.push(SearchHit {
                    id: id_val,
                    score,
                    snippet,
                });
            }
        }

        Ok(hits)
    }

    pub fn build_query_parser(&self) -> QueryParser {
        QueryParser::for_index(
            &self.index,
            vec![self.fields.title, self.fields.body, self.fields.tags],
        )
    }

    pub fn get_fields(&self) -> &TantivyFields {
        &self.fields
    }

    pub fn index_size_bytes(&self) -> u64 {
        if self.index_dir.as_os_str() == "ram" {
            return 1024 * 1024;
        }
        walkdir::WalkDir::new(&self.index_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|e| e.metadata().ok())
            .filter(|m| m.is_file())
            .map(|m| m.len())
            .sum()
    }
}
