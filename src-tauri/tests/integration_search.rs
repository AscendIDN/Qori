use private_search_lib::indexer::{SearchIndex, TantivyDocInput};
use private_search_lib::search::{build_tantivy_query, parse_search_syntax};
use private_search_lib::store::{DbStore, NewDocumentRecord, NewSourceRecord};

#[test]
fn test_integration_indexing_and_search() {
    let db = DbStore::in_memory().expect("in-memory db");
    let index = SearchIndex::in_memory().expect("in-memory index");

    // 1. Insert a source
    let src = db
        .insert_source(&NewSourceRecord {
            kind: "web".to_string(),
            uri: "https://artcraft.com".to_string(),
            label: "ArtCraft".to_string(),
            crawl_depth: 2,
            refresh_hours: 168,
            enabled: true,
        })
        .expect("insert source");

    // 2. Insert documents into SQLite and Tantivy
    let doc1 = NewDocumentRecord {
        source_id: src.id,
        url: "https://artcraft.com/offline-first".to_string(),
        title: "Offline-First Search Engine with Tantivy".to_string(),
        description: "A fast local search engine built using Rust and Tantivy.".to_string(),
        content_hash: "hash_001".to_string(),
        lang: "en".to_string(),
        word_count: 500,
        fetched_at: "2026-11-05T00:00:00Z".to_string(),
        tags: vec!["Rust".to_string(), "Search".to_string()],
    };
    let doc1_id = db.insert_document(&doc1).expect("insert doc1");

    index
        .index_document(&TantivyDocInput {
            id: doc1_id as u64,
            url: doc1.url.clone(),
            title: doc1.title.clone(),
            body: "Tantivy is a full-text search engine library inspired by Apache Lucene and written in Rust. It guarantees high performance local search without external dependencies.".to_string(),
            description: doc1.description.clone(),
            tags: doc1.tags.clone(),
            domain: "artcraft.com".to_string(),
            fetched_at: doc1.fetched_at.clone(),
        })
        .expect("index doc1");

    let doc2 = NewDocumentRecord {
        source_id: src.id,
        url: "https://artcraft.com/cloud-database".to_string(),
        title: "Comparison of Cloud Databases".to_string(),
        description: "Cloud database architectures and distributed network latency.".to_string(),
        content_hash: "hash_002".to_string(),
        lang: "en".to_string(),
        word_count: 800,
        fetched_at: "2026-11-05T01:00:00Z".to_string(),
        tags: vec!["Cloud".to_string(), "Database".to_string()],
    };
    let doc2_id = db.insert_document(&doc2).expect("insert doc2");

    index
        .index_document(&TantivyDocInput {
            id: doc2_id as u64,
            url: doc2.url.clone(),
            title: doc2.title.clone(),
            body: "Cloud systems require constant internet connectivity and transmit telemetry to remote servers.".to_string(),
            description: doc2.description.clone(),
            tags: doc2.tags.clone(),
            domain: "artcraft.com".to_string(),
            fetched_at: doc2.fetched_at.clone(),
        })
        .expect("index doc2");

    index.commit().expect("commit");

    // 3. Test Keyword search for "tantivy"
    let criteria = parse_search_syntax("tantivy");
    let query = build_tantivy_query(&criteria, index.get_fields());
    let hits = index.search(&*query, 10).expect("search tantivy");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].id, doc1_id as u64);

    // 4. Test Exclusion: "search -cloud"
    let criteria_exclude = parse_search_syntax("search -cloud");
    let query_exclude = build_tantivy_query(&criteria_exclude, index.get_fields());
    let hits_exclude = index.search(&*query_exclude, 10).expect("search exclude");
    assert_eq!(hits_exclude.len(), 1);
    assert_eq!(hits_exclude[0].id, doc1_id as u64);

    // 5. Test Tag filter: "tag:Cloud"
    let criteria_tag = parse_search_syntax("tag:cloud");
    let query_tag = build_tantivy_query(&criteria_tag, index.get_fields());
    let hits_tag = index.search(&*query_tag, 10).expect("search tag");
    assert_eq!(hits_tag.len(), 1);
    assert_eq!(hits_tag[0].id, doc2_id as u64);

    // 6. Test Autocomplete suggestions
    let suggestions = db.get_suggestions("Offline").expect("suggestions");
    assert!(!suggestions.is_empty());
    assert!(suggestions.iter().any(|s| s.contains("Offline-First")));
}
