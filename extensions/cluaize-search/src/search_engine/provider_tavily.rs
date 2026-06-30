use crate::search_engine::provider::{SearchProvider, SearchResult};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

pub struct TavilyProvider {
    pub api_key: String,
}

#[async_trait]
impl SearchProvider for TavilyProvider {
    fn name(&self) -> &'static str {
        "Tavily"
    }

    async fn query(&self, query: &str) -> Result<Vec<SearchResult>, String> {
        let client = Client::new();
        let url = "https://api.tavily.com/search";
        
        let body = serde_json::json!({
            "api_key": self.api_key,
            "query": query,
            "search_depth": "basic", // Only basic to save cost/time
            "include_answer": false,
            "include_raw_content": false, // We will fetch it ourselves
            "max_results": 5
        });

        let response = client.post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Tavily network error: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Tavily returned error status: {}", response.status()));
        }

        let json: Value = response.json().await.map_err(|e| format!("Failed to parse JSON: {}", e))?;
        let mut results = Vec::new();

        if let Some(results_array) = json.get("results").and_then(|r| r.as_array()) {
            for item in results_array {
                let title = item.get("title").and_then(|t| t.as_str()).unwrap_or("No Title").to_string();
                let url = item.get("url").and_then(|u| u.as_str()).unwrap_or("").to_string();
                let snippet = item.get("content").and_then(|c| c.as_str()).map(|s| s.to_string());
                
                if !url.is_empty() {
                    let favicon = crate::search_engine::provider::get_favicon_url(&url);
                    results.push(SearchResult { title, url, favicon, snippet, raw_content: None });
                }
            }
        }

        Ok(results)
    }
}
