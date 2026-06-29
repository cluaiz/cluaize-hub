use scraper::{Html, Selector};
use serde_json::Value;

/// Extracts OpenGraph meta tags, titles, and logos (essential for YouTube links and citations)
pub struct MetadataExtractor;

impl MetadataExtractor {
    /// Extracts rich metadata from the raw HTML to provide context to the LLM (like OpenGraph data).
    pub fn extract(raw_html: &str) -> Value {
        let document = Html::parse_document(raw_html);
        
        let title_selector = Selector::parse("title, meta[property='og:title']").unwrap();
        let desc_selector = Selector::parse("meta[name='description'], meta[property='og:description']").unwrap();
        let image_selector = Selector::parse("meta[property='og:image']").unwrap();
        
        let mut extracted_title = String::new();
        let mut extracted_desc = String::new();
        let mut extracted_image = String::new();

        // Extract Title
        if let Some(title_element) = document.select(&title_selector).next() {
            if title_element.value().name() == "title" {
                extracted_title = title_element.text().collect::<Vec<_>>().join(" ");
            } else if let Some(content) = title_element.value().attr("content") {
                extracted_title = content.to_string();
            }
        }

        // Extract Description
        if let Some(desc_element) = document.select(&desc_selector).next() {
            if let Some(content) = desc_element.value().attr("content") {
                extracted_desc = content.to_string();
            }
        }

        // Extract Image (Thumbnail)
        if let Some(img_element) = document.select(&image_selector).next() {
            if let Some(content) = img_element.value().attr("content") {
                extracted_image = content.to_string();
            }
        }

        serde_json::json!({
            "title": extracted_title,
            "description": extracted_desc,
            "thumbnail": extracted_image
        })
    }
}
