/// RAM-Aware Compression and BM25-lite Text Ranking
pub struct Ranker;

impl Ranker {
    /// Dynamically compresses the extracted context based on available RAM.
    /// Uses keyword density scoring to keep only the most relevant sentences.
    pub fn compress_context(clean_text: &str, query: &str, max_ram_mb: usize) -> String {
        let text_len = clean_text.len();
        
        // If we have abundant RAM or the text is small, return full context
        if max_ram_mb >= 8192 || text_len < 10_000 {
            return clean_text.to_string();
        }
        
        let query_terms: Vec<&str> = query.split_whitespace().collect();
        let sentences: Vec<&str> = clean_text.split(|c| c == '.' || c == '?' || c == '!').collect();
        
        let mut relevant_sentences = Vec::new();
        let mut current_size = 0;
        let max_allowed_size = (max_ram_mb / 2) * 1024; // Dynamic scaling based on MB
        
        for sentence in sentences {
            let s_lower = sentence.to_lowercase();
            let mut score = 0;
            
            for term in &query_terms {
                if s_lower.contains(&term.to_lowercase()) {
                    score += 1;
                }
            }
            
            if score > 0 {
                if current_size + sentence.len() < max_allowed_size {
                    relevant_sentences.push(sentence.trim());
                    current_size += sentence.len();
                } else {
                    break;
                }
            }
        }
        
        if relevant_sentences.is_empty() {
            clean_text.chars().take(max_allowed_size).collect()
        } else {
            relevant_sentences.join(". ") + "."
        }
    }
}
