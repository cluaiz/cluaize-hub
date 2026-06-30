use crate::search_engine::provider::{SearchProvider, SearchResult};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

pub struct SerpApiProvider {
    pub api_key: String,
}

#[async_trait]
impl SearchProvider for SerpApiProvider {
    fn name(&self) -> &'static str {
        "SerpAPI"
    }

    async fn query(&self, query: &str) -> Result<Vec<SearchResult>, String> {
        let client = Client::new();
        let encoded_query = urlencoding::encode(query);
        let url = format!(
            "https://serpapi.com/search.json?q={}&api_key={}&engine=google&num=5",
            encoded_query, self.api_key
        );

        let response = client.get(&url)
            .send()
            .await
            .map_err(|e| format!("SerpAPI network error: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("SerpAPI returned error status: {}", response.status()));
        }

        let json: Value = response.json().await.map_err(|e| format!("Failed to parse JSON: {}", e))?;
        let mut results = Vec::new();

        if let Some(results_array) = json.get("organic_results").and_then(|r| r.as_array()) {
            for item in results_array {
                let title = item.get("title").and_then(|t| t.as_str()).unwrap_or("No Title").to_string();
                let url = item.get("link").and_then(|u| u.as_str()).unwrap_or("").to_string();
                let snippet = item.get("snippet").and_then(|c| c.as_str()).map(|s| s.to_string());
                
                if !url.is_empty() {
                    let favicon = crate::search_engine::provider::get_favicon_url(&url);
                    results.push(SearchResult { title, url, favicon, snippet, raw_content: None });
                }
            }
        }

        Ok(results)
    }
}
