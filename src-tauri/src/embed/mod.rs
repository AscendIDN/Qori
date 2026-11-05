use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize)]
struct OllamaEmbedRequest {
    model: String,
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct OllamaEmbedResponse {
    embedding: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearxngResultItem {
    pub title: String,
    pub url: String,
    pub content: String,
    pub score: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct SearxngResponse {
    results: Vec<SearxngResultItem>,
}

pub struct SemanticClient {
    client: reqwest::Client,
    ollama_url: String,
    searxng_url: String,
}

impl SemanticClient {
    pub fn new(ollama_url: String, searxng_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(8))
            .build()
            .unwrap_or_default();

        Self {
            client,
            ollama_url,
            searxng_url,
        }
    }

    /// Fetches vector embeddings from local Ollama instance (e.g. nomic-embed-text)
    pub async fn get_embedding(&self, text: &str, model: &str) -> Result<Vec<f32>, String> {
        let url = format!("{}/api/embeddings", self.ollama_url.trim_end_matches('/'));
        let body = OllamaEmbedRequest {
            model: model.to_string(),
            prompt: text.to_string(),
        };

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Ollama connection error: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Ollama returned HTTP {}", resp.status()));
        }

        let data: OllamaEmbedResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

        Ok(data.embedding)
    }

    /// Queries local SearXNG instance over localhost HTTP JSON API
    pub async fn search_searxng(&self, query: &str) -> Result<Vec<SearxngResultItem>, String> {
        let url = format!(
            "{}/search?q={}&format=json",
            self.searxng_url.trim_end_matches('/'),
            urlencoding_encode(query)
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("SearXNG connection error: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("SearXNG returned HTTP {}", resp.status()));
        }

        let data: SearxngResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse SearXNG response: {}", e))?;

        Ok(data.results)
    }
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for i in 0..a.len() {
        dot_product += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a.sqrt() * norm_b.sqrt())
    }
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
    fn test_cosine_similarity() {
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![1.0, 0.0, 0.0];
        let v3 = vec![0.0, 1.0, 0.0];

        // Identical vectors = 1.0
        assert!((cosine_similarity(&v1, &v2) - 1.0).abs() < 1e-5);
        // Orthogonal vectors = 0.0
        assert!((cosine_similarity(&v1, &v3) - 0.0).abs() < 1e-5);
    }
}
