use reqwest::Client;
use std::time::Duration;
use crate::search_engine::rotator::Rotator;
use futures::future::join_all;

/// Handles making asynchronous parallel HTTP requests to metasearch mirrors
pub struct Multiplexer;

impl Multiplexer {
    pub async fn fetch_query(query: &str, timeout_secs: u64, _custom_mirrors: Option<Vec<String>>, api_key: &str, api_type: &str, think_mode: bool, response_length: &str) -> Result<Vec<crate::search_engine::provider::SearchResult>, String> {
        let provider = Rotator::get_provider(api_type, api_key);
        
        let mut search_results = provider.query(query).await?;
        if search_results.is_empty() {
            return Err("Search provider returned no results.".to_string());
        }

        let do_deep_scrape = think_mode || response_length == "long";

        if do_deep_scrape {
            let mut fetch_futures = Vec::new();
            // Limit to top 3 to save time/bandwidth
            for result in search_results.iter().take(3) {
                fetch_futures.push(Self::fetch_url_internal(result.url.clone(), timeout_secs));
            }

            let fetched_contents = join_all(fetch_futures).await;
            
            for (i, content_result) in fetched_contents.into_iter().enumerate() {
                if let Some(result) = search_results.get_mut(i) {
                    match content_result {
                        Ok(html) => result.raw_content = Some(html),
                        Err(e) => result.raw_content = Some(format!("Error fetching: {}", e)),
                    }
                }
            }
        }

        Ok(search_results)
    }

    /// Fetches the raw HTML from a direct URL (public API)
    pub async fn fetch_url(url: &str, timeout_secs: u64) -> Result<String, String> {
        Self::fetch_url_internal(url.to_string(), timeout_secs).await
    }

    /// Internal fetching logic
    async fn fetch_url_internal(url: String, timeout_secs: u64) -> Result<String, String> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
            .build()
            .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Network request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Website returned error status: {}", response.status()));
        }

        let html = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        Ok(html)
    }
}
