use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub favicon: Option<String>,
    pub snippet: Option<String>,
    pub raw_content: Option<String>,

}

#[async_trait]
pub trait SearchProvider: Send + Sync {
    /// Returns the name of the provider (e.g., "Tavily", "SerpAPI")
    fn name(&self) -> &'static str;
    
    /// Queries the search API and returns a list of URLs and titles
    async fn query(&self, query: &str) -> Result<Vec<SearchResult>, String>;
}

pub fn get_favicon_url(url_str: &str) -> Option<String> {
    if let Ok(parsed) = reqwest::Url::parse(url_str) {
        if let Some(domain) = parsed.host_str() {
            return Some(format!("https://www.google.com/s2/favicons?sz=64&domain={}", domain));
        }
    }
    None
}

