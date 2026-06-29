use scraper::{Html, Selector};

/// Removes HTML tags, JS, and CSS using the `scraper` crate
pub struct Stripper;

impl Stripper {
    /// Deeply cleans raw HTML, using dynamic exclusion rules from the CEL command.
    pub fn clean_html(raw_html: &str, exclude_rules: &str) -> String {
        let document = Html::parse_document(raw_html);
        
        // Use provided CEL rules or fallback to default junk tags
        let rule_string = if exclude_rules.trim().is_empty() {
            "script, style, noscript, svg, nav, footer, iframe"
        } else {
            exclude_rules
        };
        
        let exclude_selector = Selector::parse(rule_string).unwrap_or_else(|_| {
            Selector::parse("script, style").unwrap()
        });
        
        let mut clean_text = String::with_capacity(raw_html.len() / 2);
        
        for node in document.tree.nodes() {
            if let Some(element_ref) = scraper::ElementRef::wrap(node) {
                if exclude_selector.matches(&element_ref) {
                    continue; 
                }
                
                for text_node in node.children() {
                    if let Some(text) = text_node.value().as_text() {
                        let trimmed = text.trim();
                        if !trimmed.is_empty() {
                            clean_text.push_str(trimmed);
                            clean_text.push(' ');
                        }
                    }
                }
            }
        }
        
        let cleaned: Vec<&str> = clean_text.split_whitespace().collect();
        cleaned.join(" ")
    }
}
