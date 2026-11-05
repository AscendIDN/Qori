use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedBookmark {
    pub url: String,
    pub title: String,
    pub folder: Option<String>,
    pub add_date: Option<String>,
}

pub struct BookmarkParser;

impl BookmarkParser {
    /// Parses standard Netscape Bookmark HTML exported by Chrome, Firefox, Safari, and Edge.
    pub fn parse_html(html: &str) -> Vec<ImportedBookmark> {
        let mut bookmarks = Vec::new();
        let document = scraper::Html::parse_document(html);

        if let Ok(sel) = scraper::Selector::parse("a[href]") {
            for element in document.select(&sel) {
                if let Some(href) = element.value().attr("href") {
                    if href.starts_with("http://") || href.starts_with("https://") {
                        let title = element.text().collect::<String>().trim().to_string();
                        let add_date = element.value().attr("add_date").map(|s| s.to_string());

                        bookmarks.push(ImportedBookmark {
                            url: href.to_string(),
                            title: if title.is_empty() {
                                href.to_string()
                            } else {
                                title
                            },
                            folder: None,
                            add_date,
                        });
                    }
                }
            }
        }

        bookmarks
    }

    /// Parses Chrome/Edge JSON bookmarks file format.
    pub fn parse_json(json_str: &str) -> Vec<ImportedBookmark> {
        let mut bookmarks = Vec::new();
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(json_str) {
            if let Some(roots) = v.get("roots").and_then(|r| r.as_object()) {
                for (_root_name, root_val) in roots {
                    Self::traverse_json_node(root_val, None, &mut bookmarks);
                }
            }
        }
        bookmarks
    }

    fn traverse_json_node(
        node: &serde_json::Value,
        folder_name: Option<&str>,
        bookmarks: &mut Vec<ImportedBookmark>,
    ) {
        if let Some(node_type) = node.get("type").and_then(|t| t.as_str()) {
            if node_type == "url" {
                if let (Some(url), Some(name)) = (
                    node.get("url").and_then(|u| u.as_str()),
                    node.get("name").and_then(|n| n.as_str()),
                ) {
                    bookmarks.push(ImportedBookmark {
                        url: url.to_string(),
                        title: name.to_string(),
                        folder: folder_name.map(|s| s.to_string()),
                        add_date: node
                            .get("date_added")
                            .and_then(|d| d.as_str())
                            .map(|s| s.to_string()),
                    });
                }
            } else if node_type == "folder" {
                let current_folder = node
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("Folder");
                if let Some(children) = node.get("children").and_then(|c| c.as_array()) {
                    for child in children {
                        Self::traverse_json_node(child, Some(current_folder), bookmarks);
                    }
                }
            }
        }
    }
}

pub struct LocalFileIngest;

impl LocalFileIngest {
    /// Ingests and extracts clean plaintext from markdown (.md) documents
    pub fn extract_markdown(content: &str) -> (String, String) {
        let parser = Parser::new(content);
        let mut text_parts = Vec::new();
        let mut first_heading = String::new();
        let mut in_heading = false;

        for event in parser {
            match event {
                Event::Start(Tag::Heading { .. }) => {
                    in_heading = true;
                }
                Event::End(TagEnd::Heading(_)) => {
                    in_heading = false;
                }
                Event::Text(t) => {
                    if in_heading && first_heading.is_empty() {
                        first_heading = t.to_string();
                    }
                    text_parts.push(t.to_string());
                }
                Event::Code(c) => {
                    text_parts.push(c.to_string());
                }
                _ => {}
            }
        }

        let full_text = text_parts.join(" ");
        (first_heading, full_text)
    }

    /// Read local file based on extension (.md, .txt, .html)
    pub fn read_file_content<P: AsRef<Path>>(path: P) -> Result<(String, String), std::io::Error> {
        let p = path.as_ref();
        let ext = p
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        let raw = std::fs::read_to_string(p)?;

        if ext == "md" {
            let (heading, body) = Self::extract_markdown(&raw);
            let title = if !heading.is_empty() {
                heading
            } else {
                p.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Document")
                    .to_string()
            };
            Ok((title, body))
        } else if ext == "html" || ext == "htm" {
            let page = crate::crawler::HtmlExtractor::extract(&raw, &p.to_string_lossy());
            Ok((page.title, page.body))
        } else {
            // Default plain text
            let title = p
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Document")
                .to_string();
            Ok((title, raw))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_netscape_bookmarks() {
        let html = r#"
        <!DOCTYPE NETSCAPE-Bookmark-file-1>
        <META HTTP-EQUIV="Content-Type" CONTENT="text/html; charset=UTF-8">
        <TITLE>Bookmarks</TITLE>
        <H1>Bookmarks</H1>
        <DL><p>
            <DT><A HREF="https://artcraft.com" ADD_DATE="1730000000">ArtCraft Official</A>
            <DT><A HREF="https://github.com/quickwit-oss/tantivy">Tantivy Github</A>
        </DL><p>
        "#;

        let bookmarks = BookmarkParser::parse_html(html);
        assert_eq!(bookmarks.len(), 2);
        assert_eq!(bookmarks[0].url, "https://artcraft.com");
        assert_eq!(bookmarks[0].title, "ArtCraft Official");
        assert_eq!(bookmarks[1].url, "https://github.com/quickwit-oss/tantivy");
    }

    #[test]
    fn test_extract_markdown() {
        let md = r#"
# Local-First Architecture

We believe software should run directly on user machines.

```rust
fn main() {
    println!("Hello local world");
}
```
        "#;

        let (title, body) = LocalFileIngest::extract_markdown(md);
        assert_eq!(title, "Local-First Architecture");
        assert!(body.contains("run directly on user machines"));
        assert!(body.contains("Hello local world"));
    }
}
