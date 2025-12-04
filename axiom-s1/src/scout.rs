//! Scout - Headless Browser for Web Scraping
//!
//! Phase 1 of the Sovereign Loop: SENSE
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScoutError {
    #[error("Failed to connect to browser: {0}")]
    Connection(String),
    #[error("Navigation failed: {0}")]
    Navigation(String),
    #[error("Scraping failed: {0}")]
    Scraping(String),
    #[error("Timeout")]
    Timeout,
}

/// Scraped page content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedPage {
    pub url: String,
    pub title: String,
    pub content: String,
    pub links: Vec<String>,
    pub metadata: PageMetadata,
    pub hash: String,
    pub timestamp: String,
}

/// Page metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMetadata {
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub author: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
}

/// Scout a URL and return scraped content
pub async fn scout_url(url: &str) -> Result<serde_json::Value, ScoutError> {
    tracing::info!("Scout: Scraping {}", url);
    
    // In production, this would use fantoccini with a headless browser
    // For now, we use reqwest for simple HTTP fetching
    
    let client = reqwest::Client::builder()
        .user_agent("AxiomS1/1.0 (Sovereign Browser)")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| ScoutError::Connection(e.to_string()))?;
    
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| ScoutError::Navigation(e.to_string()))?;
    
    let status = response.status();
    if !status.is_success() {
        return Err(ScoutError::Navigation(format!("HTTP {}", status)));
    }
    
    let html = response
        .text()
        .await
        .map_err(|e| ScoutError::Scraping(e.to_string()))?;
    
    // Extract content
    let scraped = parse_html(url, &html);
    
    // Hash the content for provenance
    let hash = crate::invariance::sha256(&scraped.content);
    
    Ok(serde_json::json!({
        "url": scraped.url,
        "title": scraped.title,
        "content": scraped.content,
        "links": scraped.links,
        "metadata": scraped.metadata,
        "hash": hash,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "provenance": {
            "source_type": "web",
            "source_uri": url,
            "content_hash": hash
        }
    }))
}

/// Scout search results
pub async fn scout_search(query: &str) -> Result<serde_json::Value, ScoutError> {
    tracing::info!("Scout: Searching for '{}'", query);
    
    // In production, this would perform actual search
    // For now, return a placeholder
    
    Ok(serde_json::json!({
        "query": query,
        "results": [],
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "note": "Search functionality requires headless browser setup"
    }))
}

/// Parse HTML and extract structured content
fn parse_html(url: &str, html: &str) -> ScrapedPage {
    // Simple HTML parsing (in production, use scraper crate)
    let title = extract_between(html, "<title>", "</title>")
        .unwrap_or_else(|| "Untitled".to_string());
    
    // Extract text content (simplified)
    let content = extract_text_content(html);
    
    // Extract links
    let links = extract_links(html);
    
    // Extract metadata
    let metadata = PageMetadata {
        description: extract_meta_content(html, "description"),
        keywords: extract_meta_content(html, "keywords")
            .map(|k| k.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default(),
        author: extract_meta_content(html, "author"),
        og_title: extract_meta_property(html, "og:title"),
        og_description: extract_meta_property(html, "og:description"),
    };
    
    let hash = crate::invariance::sha256(&content);
    
    ScrapedPage {
        url: url.to_string(),
        title,
        content,
        links,
        metadata,
        hash,
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}

/// Extract text between tags
fn extract_between(html: &str, start: &str, end: &str) -> Option<String> {
    let start_idx = html.find(start)? + start.len();
    let end_idx = html[start_idx..].find(end)? + start_idx;
    Some(html[start_idx..end_idx].trim().to_string())
}

/// Extract meta content by name
fn extract_meta_content(html: &str, name: &str) -> Option<String> {
    let pattern = format!(r#"<meta\s+name=["']{}["']\s+content=["']([^"']+)["']"#, name);
    if let Ok(re) = regex::Regex::new(&pattern) {
        if let Some(caps) = re.captures(html) {
            return caps.get(1).map(|m| m.as_str().to_string());
        }
    }
    None
}

/// Extract meta property by name
fn extract_meta_property(html: &str, property: &str) -> Option<String> {
    let pattern = format!(r#"<meta\s+property=["']{}["']\s+content=["']([^"']+)["']"#, property);
    if let Ok(re) = regex::Regex::new(&pattern) {
        if let Some(caps) = re.captures(html) {
            return caps.get(1).map(|m| m.as_str().to_string());
        }
    }
    None
}

/// Extract text content from HTML (simplified)
fn extract_text_content(html: &str) -> String {
    // Remove script and style tags
    let mut text = html.to_string();
    
    let patterns = [
        r"<script[^>]*>[\s\S]*?</script>",
        r"<style[^>]*>[\s\S]*?</style>",
        r"<[^>]+>",
        r"\s+",
    ];
    
    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            text = re.replace_all(&text, " ").to_string();
        }
    }
    
    // Decode HTML entities (simplified)
    text = text.replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"");
    
    // Clean up whitespace
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Extract links from HTML
fn extract_links(html: &str) -> Vec<String> {
    let pattern = r#"href=["']([^"']+)["']"#;
    let mut links = Vec::new();
    
    if let Ok(re) = regex::Regex::new(pattern) {
        for cap in re.captures_iter(html) {
            if let Some(m) = cap.get(1) {
                let link = m.as_str().to_string();
                if link.starts_with("http") {
                    links.push(link);
                }
            }
        }
    }
    
    // Deduplicate
    links.sort();
    links.dedup();
    links
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_between() {
        let html = "<title>Test Title</title>";
        let title = extract_between(html, "<title>", "</title>");
        assert_eq!(title, Some("Test Title".to_string()));
    }
    
    #[test]
    fn test_extract_links() {
        let html = r#"<a href="https://example.com">Link</a>"#;
        let links = extract_links(html);
        assert!(links.contains(&"https://example.com".to_string()));
    }
    
    #[test]
    fn test_extract_text() {
        let html = "<p>Hello <b>World</b></p><script>alert(1)</script>";
        let text = extract_text_content(html);
        assert!(text.contains("Hello"));
        assert!(text.contains("World"));
        assert!(!text.contains("alert"));
    }
}

