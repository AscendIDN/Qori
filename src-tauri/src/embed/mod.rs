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

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Option<Vec<OllamaModelItem>>,
}

#[derive(Debug, Deserialize)]
struct OllamaModelItem {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearxngResultItem {
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub content: String,
    pub score: Option<f32>,
    pub engine: Option<String>,
    #[serde(default)]
    pub engines: Vec<String>,
    pub category: Option<String>,
    pub thumbnail: Option<String>,
    #[serde(alias = "thumbnail_src")]
    pub img_src: Option<String>,
    #[serde(alias = "publishedDate")]
    pub published_date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SearxngResponse {
    #[serde(default)]
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

    /// Quickly checks if the local Ollama instance is reachable
    pub async fn is_ollama_alive(&self) -> bool {
        let url = format!("{}/api/tags", self.ollama_url.trim_end_matches('/'));
        match self
            .client
            .get(&url)
            .timeout(Duration::from_millis(1500))
            .send()
            .await
        {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    /// Quickly checks if the local SearXNG instance is reachable
    pub async fn is_searxng_alive(&self) -> bool {
        let url = format!("{}/healthz", self.searxng_url.trim_end_matches('/'));
        match self
            .client
            .get(&url)
            .timeout(Duration::from_millis(1500))
            .send()
            .await
        {
            Ok(resp) => resp.status().is_success(),
            Err(_) => {
                let root = format!("{}/", self.searxng_url.trim_end_matches('/'));
                match self
                    .client
                    .get(&root)
                    .timeout(Duration::from_millis(1500))
                    .send()
                    .await
                {
                    Ok(resp) => resp.status().is_success() || resp.status().is_redirection(),
                    Err(_) => false,
                }
            }
        }
    }

    /// Fetches list of available model names installed in Ollama
    pub async fn get_available_models(&self) -> Vec<String> {
        let url = format!("{}/api/tags", self.ollama_url.trim_end_matches('/'));
        match self
            .client
            .get(&url)
            .timeout(Duration::from_secs(3))
            .send()
            .await
        {
            Ok(resp) => {
                if let Ok(data) = resp.json::<OllamaTagsResponse>().await {
                    data.models
                        .unwrap_or_default()
                        .into_iter()
                        .map(|m| m.name)
                        .collect()
                } else {
                    Vec::new()
                }
            }
            Err(_) => Vec::new(),
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
        self.search_searxng_with_fallback(query, None, Some("brave,duckduckgo"))
            .await
    }

    /// Queries SearXNG with category filtering, engine prioritization (Brave & DuckDuckGo),
    /// en-US language lock, and automatic public privacy instance fallback.
    pub async fn search_searxng_with_fallback(
        &self,
        query: &str,
        category: Option<&str>,
        engines: Option<&str>,
    ) -> Result<Vec<SearxngResultItem>, String> {
        let cat = category.unwrap_or("general");
        let eng = engines.unwrap_or("brave,duckduckgo");
        let encoded_q = urlencoding_encode(query);
        let encoded_cat = urlencoding_encode(cat);
        let encoded_eng = urlencoding_encode(eng);

        let query_params = format!(
            "q={}&categories={}&engines={}&language=en-US&format=json",
            encoded_q, encoded_cat, encoded_eng
        );

        // Tier 1: Try configured local or custom URL (e.g. Docker at http://localhost:8888)
        let primary_url = format!("{}/search?{}", self.searxng_url.trim_end_matches('/'), query_params);
        if let Ok(resp) = self.client.get(&primary_url).timeout(Duration::from_millis(2500)).send().await {
            if resp.status().is_success() {
                if let Ok(data) = resp.json::<SearxngResponse>().await {
                    if !data.results.is_empty() {
                        return Ok(data.results);
                    }
                }
            }
        }

        // Tier 2: Public Privacy Instances Pool (Seamless auto-fallback when Docker is offline)
        let public_instances = [
            "https://searx.be",
            "https://priv.au",
            "https://search.ononoki.org",
            "https://baresearch.org",
            "https://search.sapti.me",
        ];

        for instance in public_instances {
            let instance_url = format!("{}/search?{}", instance, query_params);
            if let Ok(resp) = self.client.get(&instance_url).timeout(Duration::from_millis(3500)).send().await {
                if resp.status().is_success() {
                    if let Ok(data) = resp.json::<SearxngResponse>().await {
                        if !data.results.is_empty() {
                            return Ok(data.results);
                        }
                    }
                }
            }
        }

        // Tier 2b: Relaxed query without engine restriction if specific engines failed
        let relaxed_params = format!("q={}&categories={}&language=en-US&format=json", encoded_q, encoded_cat);
        for instance in public_instances.iter().take(3) {
            let instance_url = format!("{}/search?{}", instance, relaxed_params);
            if let Ok(resp) = self.client.get(&instance_url).timeout(Duration::from_millis(3000)).send().await {
                if resp.status().is_success() {
                    if let Ok(data) = resp.json::<SearxngResponse>().await {
                        if !data.results.is_empty() {
                            return Ok(data.results);
                        }
                    }
                }
            }
        }

        Err("No results from SearXNG instances".to_string())
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
