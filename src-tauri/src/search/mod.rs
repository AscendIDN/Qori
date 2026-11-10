use std::collections::HashMap;
use base64::prelude::*;
use tantivy::query::{AllQuery, BooleanQuery, Occur, PhraseQuery, Query, RegexQuery, TermQuery};
use tantivy::schema::{IndexRecordOption, Term};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ParsedQueryCriteria {
    pub phrases: Vec<String>,
    pub terms: Vec<String>,
    pub excludes: Vec<String>,
    pub site: Option<String>,
    pub tag: Option<String>,
    pub before_date: Option<String>,
    pub after_date: Option<String>,
}

pub fn parse_search_syntax(input: &str) -> ParsedQueryCriteria {
    let mut criteria = ParsedQueryCriteria::default();
    let mut chars = input.chars().peekable();
    let mut current_token = String::new();

    while let Some(&ch) = chars.peek() {
        if ch == '"' {
            chars.next(); // consume opening quote
            let mut phrase = String::new();
            for c in chars.by_ref() {
                if c == '"' {
                    break;
                }
                phrase.push(c);
            }
            if !phrase.trim().is_empty() {
                criteria.phrases.push(phrase.trim().to_lowercase());
            }
        } else if ch.is_whitespace() {
            chars.next();
            if !current_token.is_empty() {
                process_token(std::mem::take(&mut current_token), &mut criteria);
            }
        } else {
            current_token.push(ch);
            chars.next();
        }
    }

    if !current_token.is_empty() {
        process_token(current_token, &mut criteria);
    }

    criteria
}

fn process_token(token: String, criteria: &mut ParsedQueryCriteria) {
    let lower = token.to_lowercase();
    if let Some(stripped) = lower.strip_prefix('-') {
        if !stripped.is_empty() {
            criteria.excludes.push(stripped.to_string());
        }
    } else if let Some(stripped) = lower.strip_prefix("site:") {
        if !stripped.is_empty() {
            criteria.site = Some(stripped.to_string());
        }
    } else if let Some(stripped) = lower.strip_prefix("tag:") {
        if !stripped.is_empty() {
            criteria.tag = Some(stripped.to_string());
        }
    } else if let Some(stripped) = lower.strip_prefix("before:") {
        if !stripped.is_empty() {
            criteria.before_date = Some(stripped.to_string());
        }
    } else if let Some(stripped) = lower.strip_prefix("after:") {
        if !stripped.is_empty() {
            criteria.after_date = Some(stripped.to_string());
        }
    } else if !token.trim().is_empty() {
        criteria.terms.push(lower);
    }
}

pub fn build_tantivy_query(
    criteria: &ParsedQueryCriteria,
    fields: &crate::indexer::TantivyFields,
) -> Box<dyn Query> {
    let mut clauses: Vec<(Occur, Box<dyn Query>)> = Vec::new();

    // 1. Exact Phrases (PhraseQuery across title, body)
    for phrase in &criteria.phrases {
        let words: Vec<&str> = phrase.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }

        let body_terms: Vec<Term> = words
            .iter()
            .map(|w| Term::from_field_text(fields.body, w))
            .collect();
        let title_terms: Vec<Term> = words
            .iter()
            .map(|w| Term::from_field_text(fields.title, w))
            .collect();

        let phrase_body = PhraseQuery::new(body_terms);
        let phrase_title = PhraseQuery::new(title_terms);

        let sub_bool = BooleanQuery::new(vec![
            (Occur::Should, Box::new(phrase_body)),
            (Occur::Should, Box::new(phrase_title)),
        ]);
        clauses.push((Occur::Must, Box::new(sub_bool)));
    }

    // 2. Normal Terms (Must match in either title, body, or tags)
    for term in &criteria.terms {
        let title_term = Term::from_field_text(fields.title, term);
        let body_term = Term::from_field_text(fields.body, term);
        let tags_term = Term::from_field_text(fields.tags, term);

        let mut sub_clauses: Vec<(Occur, Box<dyn Query>)> = vec![
            (
                Occur::Should,
                Box::new(TermQuery::new(
                    title_term.clone(),
                    IndexRecordOption::WithFreqsAndPositions,
                )),
            ),
            (
                Occur::Should,
                Box::new(TermQuery::new(
                    body_term.clone(),
                    IndexRecordOption::WithFreqsAndPositions,
                )),
            ),
            (
                Occur::Should,
                Box::new(TermQuery::new(
                    tags_term.clone(),
                    IndexRecordOption::WithFreqsAndPositions,
                )),
            ),
        ];

        // Also match as regex prefix for short queries like "da" -> "database", "data"
        if term.len() >= 2 {
            let pattern = format!("{}.*", regex::escape(term));
            if let Ok(rq) = RegexQuery::from_pattern(&pattern, fields.title) {
                sub_clauses.push((Occur::Should, Box::new(rq)));
            }
            if let Ok(rq) = RegexQuery::from_pattern(&pattern, fields.body) {
                sub_clauses.push((Occur::Should, Box::new(rq)));
            }
            if let Ok(rq) = RegexQuery::from_pattern(&pattern, fields.tags) {
                sub_clauses.push((Occur::Should, Box::new(rq)));
            }
        }

        clauses.push((Occur::Must, Box::new(BooleanQuery::new(sub_clauses))));
    }

    // 3. Excludes (Occur::MustNot)
    for excl in &criteria.excludes {
        let title_term = Term::from_field_text(fields.title, excl);
        let body_term = Term::from_field_text(fields.body, excl);

        clauses.push((
            Occur::MustNot,
            Box::new(TermQuery::new(title_term, IndexRecordOption::Basic)),
        ));
        clauses.push((
            Occur::MustNot,
            Box::new(TermQuery::new(body_term, IndexRecordOption::Basic)),
        ));
    }

    // 4. Site filter
    if let Some(ref site) = criteria.site {
        let domain_term = Term::from_field_text(fields.domain, site);
        clauses.push((
            Occur::Must,
            Box::new(TermQuery::new(domain_term, IndexRecordOption::Basic)),
        ));
    }

    // 5. Tag filter
    if let Some(ref tag) = criteria.tag {
        let tag_term = Term::from_field_text(fields.tags, tag);
        clauses.push((
            Occur::Must,
            Box::new(TermQuery::new(tag_term, IndexRecordOption::Basic)),
        ));
    }

    if clauses.is_empty() {
        Box::new(AllQuery)
    } else {
        Box::new(BooleanQuery::new(clauses))
    }
}

pub fn highlight_snippets(text: &str, query_terms: &[String], max_len: usize) -> String {
    if text.is_empty() {
        return String::new();
    }

    let lower_text = text.to_lowercase();
    let mut best_start = 0;

    // Find the earliest match of any term
    for term in query_terms {
        if term.is_empty() {
            continue;
        }
        if let Some(idx) = lower_text.find(term) {
            best_start = idx.saturating_sub(60);
            break;
        }
    }

    let end_index = (best_start + max_len).min(text.len());
    let slice = &text[best_start..end_index];

    let mut result = slice.to_string();
    for term in query_terms {
        if term.len() < 2 {
            continue;
        }
        let re_pattern = format!("(?i)({})", regex::escape(term));
        if let Ok(re) = regex::Regex::new(&re_pattern) {
            result = re.replace_all(&result, "<mark>$1</mark>").to_string();
        }
    }

    if best_start > 0 {
        format!("...{}...", result.trim())
    } else if end_index < text.len() {
        format!("{}...", result.trim())
    } else {
        result
    }
}

/// Reciprocal Rank Fusion (RRF) for blending BM25 ranks with vector semantic ranks.
/// Formula: RRF_score(d) = sum(1.0 / (k + rank_i))
pub fn reciprocal_rank_fusion(
    bm25_ranked_ids: &[i64],
    vector_ranked_ids: &[i64],
    k: f32,
) -> Vec<(i64, f32)> {
    let mut scores: HashMap<i64, f32> = HashMap::new();

    for (rank, &id) in bm25_ranked_ids.iter().enumerate() {
        let rrf = 1.0 / (k + (rank as f32 + 1.0));
        *scores.entry(id).or_insert(0.0) += rrf;
    }

    for (rank, &id) in vector_ranked_ids.iter().enumerate() {
        let rrf = 1.0 / (k + (rank as f32 + 1.0));
        *scores.entry(id).or_insert(0.0) += rrf;
    }

    let mut fused: Vec<(i64, f32)> = scores.into_iter().collect();
    fused.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    fused
}

pub async fn search_live_web(query: &str) -> Result<Vec<crate::store::DocumentRecord>, String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .timeout(std::time::Duration::from_secs(8))
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| e.to_string())?;

    let encoded = urlencoding_encode(query);
    let search_url = format!("https://www.bing.com/search?q={}&setlang=en", encoded);

    // Try Bing first
    if let Ok(resp) = client.get(&search_url).send().await {
        if resp.status().is_success() {
            if let Ok(html) = resp.text().await {
                let document = scraper::Html::parse_document(&html);
                if let (Ok(algo_sel), Ok(h2_a_sel), Ok(p_sel)) = (
                    scraper::Selector::parse("li.b_algo"),
                    scraper::Selector::parse("h2 a"),
                    scraper::Selector::parse("p"),
                ) {
                    let mut results = Vec::new();
                    for (idx, item) in document.select(&algo_sel).enumerate() {
                        let a = match item.select(&h2_a_sel).next() {
                            Some(a) => a,
                            None => continue,
                        };

                        let title = a.text().collect::<String>().trim().to_string();
                        let raw_href = match a.value().attr("href") {
                            Some(h) if !h.is_empty() => h.to_string(),
                            _ => continue,
                        };

                        // Resolve real destination URL if wrapped by Bing redirect (&u=a1...)
                        let real_url = if let Some(pos) = raw_href.find("&u=") {
                            let rest = &raw_href[pos + 3..];
                            let token = rest.split('&').next().unwrap_or(rest);
                            if let Some(b64_part) = token.strip_prefix("a1") {
                                let mut padded = b64_part.to_string();
                                while padded.len() % 4 != 0 {
                                    padded.push('=');
                                }
                                match BASE64_STANDARD.decode(&padded) {
                                    Ok(bytes) => String::from_utf8(bytes).unwrap_or(raw_href),
                                    Err(_) => raw_href,
                                }
                            } else {
                                raw_href
                            }
                        } else {
                            raw_href
                        };

                        let snippet = item
                            .select(&p_sel)
                            .next()
                            .map(|p| p.text().collect::<String>().trim().to_string())
                            .unwrap_or_default();

                        let domain = url::Url::parse(&real_url)
                            .ok()
                            .and_then(|u| u.host_str().map(|s| s.to_string()))
                            .unwrap_or_else(|| "WEB".to_string());

                        results.push(crate::store::DocumentRecord {
                            id: (idx + 1) as i64,
                            source_id: 0,
                            url: real_url,
                            title,
                            description: snippet.clone(),
                            content_hash: String::new(),
                            lang: "en".to_string(),
                            word_count: snippet.split_whitespace().count() as i64,
                            fetched_at: chrono::Utc::now().to_rfc3339(),
                            pinned: false,
                            tags: vec!["Web".to_string(), domain.clone()],
                            snippet: if snippet.is_empty() { None } else { Some(snippet) },
                            score: Some(1.0 / (idx as f32 + 1.0)),
                            source_kind: Some("web".to_string()),
                            source_label: Some(domain),
                        });
                    }

                    if !results.is_empty() {
                        return Ok(results);
                    }
                }
            }
        }
    }

    // Fallback: Wikipedia OpenSearch API
    let wiki_url = format!(
        "https://en.wikipedia.org/w/api.php?action=opensearch&search={}&limit=10&namespace=0&format=json",
        encoded
    );
    if let Ok(resp) = client.get(&wiki_url).send().await {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(arr) = json.as_array() {
                if arr.len() >= 4 {
                    let titles = arr[1].as_array();
                    let snippets = arr[2].as_array();
                    let urls = arr[3].as_array();
                    if let (Some(titles), Some(snippets), Some(urls)) = (titles, snippets, urls) {
                        let mut docs = Vec::new();
                        for (i, title_val) in titles.iter().enumerate() {
                            let title = title_val.as_str().unwrap_or("").to_string();
                            let link = urls.get(i).and_then(|u| u.as_str()).unwrap_or("").to_string();
                            let snippet = snippets.get(i).and_then(|s| s.as_str()).unwrap_or("").to_string();
                            if !title.is_empty() && !link.is_empty() {
                                let domain = url::Url::parse(&link)
                                    .ok()
                                    .and_then(|u| u.host_str().map(|s| s.to_string()))
                                    .unwrap_or_else(|| "en.wikipedia.org".to_string());
                                docs.push(crate::store::DocumentRecord {
                                    id: (i + 1) as i64,
                                    source_id: 0,
                                    url: link,
                                    title,
                                    description: snippet.clone(),
                                    content_hash: String::new(),
                                    lang: "id".to_string(),
                                    word_count: snippet.split_whitespace().count() as i64,
                                    fetched_at: chrono::Utc::now().to_rfc3339(),
                                    pinned: false,
                                    tags: vec!["Web".to_string(), "Wikipedia".to_string()],
                                    snippet: if snippet.is_empty() { None } else { Some(snippet) },
                                    score: Some(1.0 / (i as f32 + 1.0)),
                                    source_kind: Some("web".to_string()),
                                    source_label: Some(domain),
                                });
                            }
                        }
                        if !docs.is_empty() {
                            return Ok(docs);
                        }
                    }
                }
            }
        }
    }

    Ok(Vec::new())
}

fn urlencoding_encode(s: &str) -> String {
    let mut encoded = String::new();
    for b in s.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            b' ' => encoded.push('+'),
            _ => {
                encoded.push_str(&format!("%{:02X}", b));
            }
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_parsing_syntax() {
        let raw = "\"personal search\" rust -cloud site:artcraft.com tag:design before:2026-11-01";
        let parsed = parse_search_syntax(raw);

        assert_eq!(parsed.phrases, vec!["personal search"]);
        assert_eq!(parsed.terms, vec!["rust"]);
        assert_eq!(parsed.excludes, vec!["cloud"]);
        assert_eq!(parsed.site, Some("artcraft.com".to_string()));
        assert_eq!(parsed.tag, Some("design".to_string()));
        assert_eq!(parsed.before_date, Some("2026-11-01".to_string()));
    }

    #[test]
    fn test_rrf_fusion() {
        let bm25_ranks = vec![1, 2, 3, 4];
        let vector_ranks = vec![2, 4, 1, 5];

        let fused = reciprocal_rank_fusion(&bm25_ranks, &vector_ranks, 60.0);

        // Document 2 is rank 2 in BM25 and rank 1 in vector, so it should rank highest!
        assert_eq!(fused[0].0, 2);
        assert!(fused[0].1 > fused[1].1);
    }

    #[test]
    fn test_snippet_highlighting() {
        let text = "Qori gives you an air-gapped search engine running 100% locally on your machine.";
        let terms = vec!["air-gapped".to_string(), "locally".to_string()];
        let highlighted = highlight_snippets(text, &terms, 200);

        assert!(
            highlighted.contains("<mark>air-gapped</mark>")
                || highlighted.contains("<mark>Air-gapped</mark>")
        );
        assert!(highlighted.contains("<mark>locally</mark>"));
    }

    #[tokio::test]
    async fn test_live_web_search_resilience() {
        // Must never panic, even if offline
        let res = search_live_web("test").await;
        assert!(res.is_ok());
    }
}
