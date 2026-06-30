
use serde_json::Value;

#[tokio::main]
async fn main() {
    let raw_html = reqwest::get("https://en.wikipedia.org/wiki/Rust_(programming_language)")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let clean_text = cluaiz_search::parser::stripper::Stripper::clean_html(&raw_html, "style, script, nav, footer, table");
    
    std::fs::write("rust_search_result.md", clean_text).unwrap();
    println!("Saved output to rust_search_result.md");
}

