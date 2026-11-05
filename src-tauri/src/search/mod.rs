use std::collections::HashMap;
use tantivy::query::{AllQuery, BooleanQuery, Occur, PhraseQuery, Query, TermQuery};
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

        let sub_bool = BooleanQuery::new(vec![
            (
                Occur::Should,
                Box::new(TermQuery::new(
                    title_term,
                    IndexRecordOption::WithFreqsAndPositions,
                )),
            ),
            (
                Occur::Should,
                Box::new(TermQuery::new(
                    body_term,
                    IndexRecordOption::WithFreqsAndPositions,
                )),
            ),
            (
                Occur::Should,
                Box::new(TermQuery::new(
                    tags_term,
                    IndexRecordOption::WithFreqsAndPositions,
                )),
            ),
        ]);
        clauses.push((Occur::Must, Box::new(sub_bool)));
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
        let text = "Private search gives you an air-gapped search engine running 100% locally on your machine.";
        let terms = vec!["air-gapped".to_string(), "locally".to_string()];
        let highlighted = highlight_snippets(text, &terms, 200);

        assert!(
            highlighted.contains("<mark>air-gapped</mark>")
                || highlighted.contains("<mark>Air-gapped</mark>")
        );
        assert!(highlighted.contains("<mark>locally</mark>"));
    }
}
