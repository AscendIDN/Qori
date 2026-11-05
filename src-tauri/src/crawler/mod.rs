use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use url::Url;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RobotsRules {
    pub disallowed_prefixes: Vec<String>,
    pub crawl_delay_ms: u64,
}

impl RobotsRules {
    pub fn parse(content: &str, target_agent: &str) -> Self {
        let mut rules = RobotsRules {
            disallowed_prefixes: Vec::new(),
            crawl_delay_ms: 2000, // Default 2 seconds politeness
        };

        let mut applies_to_us = false;
        let target_lower = target_agent.to_lowercase();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if let Some(pos) = trimmed.find(':') {
                let key = trimmed[..pos].trim().to_lowercase();
                let val = trimmed[pos + 1..].trim();

                if key == "user-agent" {
                    let agent = val.to_lowercase();
                    applies_to_us = agent == "*" || agent.contains(&target_lower);
                } else if applies_to_us {
                    if key == "disallow" {
                        if !val.is_empty() {
                            rules.disallowed_prefixes.push(val.to_string());
                        }
                    } else if key == "crawl-delay" {
                        if let Ok(secs) = val.parse::<f64>() {
                            rules.crawl_delay_ms = (secs * 1000.0).max(1000.0) as u64;
                        }
                    }
                }
            }
        }

        rules
    }

    pub fn is_allowed(&self, path: &str) -> bool {
        for prefix in &self.disallowed_prefixes {
            if path.starts_with(prefix) {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
pub struct ExtractedPage {
    pub url: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub word_count: usize,
    pub content_hash: String,
    pub links: Vec<String>,
}

pub struct HtmlExtractor;

impl HtmlExtractor {
    pub fn extract(html: &str, base_url: &str) -> ExtractedPage {
        let document = scraper::Html::parse_document(html);

        // 1. Extract Title
        let title = if let Ok(sel) = scraper::Selector::parse("title") {
            document
                .select(&sel)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default()
        } else {
            String::new()
        };

        // 2. Extract Meta Description
        let description = if let Ok(sel) =
            scraper::Selector::parse("meta[name='description'], meta[property='og:description']")
        {
            document
                .select(&sel)
                .next()
                .and_then(|e| e.value().attr("content"))
                .unwrap_or_default()
                .to_string()
        } else {
            String::new()
        };

        // 3. Extract Clean Main Content (Readability heuristic)
        // Skip nav, script, style, footer, header
        let body_text = if let Ok(body_sel) = scraper::Selector::parse("main, article, body") {
            let mut parts = Vec::new();
            if let Some(main_el) = document.select(&body_sel).next() {
                for text_node in main_el.text() {
                    let t = text_node.trim();
                    if !t.is_empty() {
                        parts.push(t);
                    }
                }
            }
            parts.join(" ")
        } else {
            String::new()
        };

        // 4. Compute Content Hash (SHA256) for deduplication
        let mut hasher = Sha256::new();
        hasher.update(body_text.as_bytes());
        let content_hash = hex::encode(hasher.finalize());

        let word_count = body_text.split_whitespace().count();

        // 5. Discover Links
        let mut links = Vec::new();
        if let (Ok(base), Ok(a_sel)) = (Url::parse(base_url), scraper::Selector::parse("a[href]")) {
            for a_el in document.select(&a_sel) {
                if let Some(href) = a_el.value().attr("href") {
                    if let Ok(resolved) = base.join(href) {
                        // Keep only HTTP/HTTPS links on same domain
                        if (resolved.scheme() == "http" || resolved.scheme() == "https")
                            && resolved.host_str() == base.host_str()
                        {
                            let mut clean = resolved;
                            clean.set_fragment(None);
                            clean.set_query(None);
                            links.push(clean.to_string());
                        }
                    }
                }
            }
        }

        ExtractedPage {
            url: base_url.to_string(),
            title: title.trim().to_string(),
            description: description.trim().to_string(),
            body: body_text,
            word_count,
            content_hash,
            links,
        }
    }
}

pub struct CrawlerEngine {
    client: reqwest::Client,
    domain_last_request: Arc<Mutex<HashMap<String, Instant>>>,
    robots_cache: Arc<Mutex<HashMap<String, RobotsRules>>>,
}

impl CrawlerEngine {
    pub fn new() -> Self {
        let user_agent =
            "PrivateSearchBot/1.0 (+https://github.com/private-search; personal use)".to_string();
        let client = reqwest::Client::builder()
            .user_agent(&user_agent)
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self {
            client,
            domain_last_request: Arc::new(Mutex::new(HashMap::new())),
            robots_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn fetch_robots_txt(&self, base_url: &Url) -> RobotsRules {
        let host = match base_url.host_str() {
            Some(h) => h.to_string(),
            None => return RobotsRules::default(),
        };

        {
            let cache = self.robots_cache.lock().await;
            if let Some(rules) = cache.get(&host) {
                return rules.clone();
            }
        }

        let robots_url = format!("{}://{}/robots.txt", base_url.scheme(), host);
        let rules = match self.client.get(&robots_url).send().await {
            Ok(resp) if resp.status().is_success() => {
                let text = resp.text().await.unwrap_or_default();
                RobotsRules::parse(&text, "PrivateSearchBot")
            }
            _ => RobotsRules::default(),
        };

        let mut cache = self.robots_cache.lock().await;
        cache.insert(host, rules.clone());
        rules
    }

    pub async fn polite_delay(&self, host: &str, delay_ms: u64) {
        let now = Instant::now();
        let sleep_duration = {
            let map = self.domain_last_request.lock().await;
            if let Some(&last) = map.get(host) {
                let elapsed = now.duration_since(last).as_millis() as u64;
                if elapsed < delay_ms {
                    Some(Duration::from_millis(delay_ms - elapsed))
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(dur) = sleep_duration {
            tokio::time::sleep(dur).await;
        }

        let mut map = self.domain_last_request.lock().await;
        map.insert(host.to_string(), Instant::now());
    }

    pub async fn fetch_page(&self, target_url: &str) -> Result<Option<ExtractedPage>, String> {
        let parsed = Url::parse(target_url).map_err(|e| e.to_string())?;
        let host = parsed.host_str().unwrap_or("unknown");

        let rules = self.fetch_robots_txt(&parsed).await;
        if !rules.is_allowed(parsed.path()) {
            return Ok(None); // Disallowed by robots.txt
        }

        self.polite_delay(host, rules.crawl_delay_ms).await;

        let resp = self
            .client
            .get(target_url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("HTTP status {}", resp.status()));
        }

        let html = resp.text().await.map_err(|e| e.to_string())?;
        let page = HtmlExtractor::extract(&html, target_url);
        Ok(Some(page))
    }
}

impl Default for CrawlerEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robots_txt_parser_disallow() {
        let robots = "
User-agent: *
Disallow: /admin/
Disallow: /private
Crawl-delay: 3

User-agent: Googlebot
Disallow: /something
";
        let rules = RobotsRules::parse(robots, "PrivateSearchBot");
        assert_eq!(rules.crawl_delay_ms, 3000);
        assert!(!rules.is_allowed("/admin/panel"));
        assert!(!rules.is_allowed("/private/secret.html"));
        assert!(rules.is_allowed("/blog/post-1"));
        assert!(rules.is_allowed("/about"));
    }

    #[test]
    fn test_html_extraction() {
        let html = r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>ArtCraft Philosophy</title>
                <meta name="description" content="A guide to tactile and editorial software design.">
            </head>
            <body>
                <header><nav><a href="/home">Home</a></nav></header>
                <main>
                    <h1>The Craft of Interfaces</h1>
                    <p>Local-first computing returns agency to the user.</p>
                    <a href="/research">Research Paper</a>
                </main>
                <footer>Copyright 2026</footer>
            </body>
        </html>
        "#;

        let page = HtmlExtractor::extract(html, "https://artcraft.com/philosophy");
        assert_eq!(page.title, "ArtCraft Philosophy");
        assert_eq!(
            page.description,
            "A guide to tactile and editorial software design."
        );
        assert!(page.body.contains("Local-first computing"));
        assert!(page.links.iter().any(|l| l.contains("/research")));
    }
}
