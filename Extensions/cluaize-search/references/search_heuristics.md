# Search Heuristics & Provider Optimization

This document contains advanced heuristics for the `cluaize-search` extension. When the engine is operating in complex analysis modes, refer to these strategies to maximize search quality.

## Provider Differences

The `cluaize-search` extension dynamically routes your queries to different backend providers based on the `manifest-extension.yaml` configuration. You must optimize your query based on the active provider.

### 1. Tavily (`search_api_type: "tavily"`)
Tavily is an AI-native search engine designed for RAG (Retrieval-Augmented Generation).
- **Optimization:** It prefers natural language questions and dense context over strict boolean queries.
- **Do:** `"What were the major changes in the Rust 2026 edition regarding the memory model?"`
- **Don't:** `"Rust 2026 edition memory model"`

### 2. DuckDuckGo (`search_api_type: "duckduckgo"`)
DuckDuckGo is a traditional keyword-based search engine.
- **Optimization:** It performs poorly on long sentences. It requires exact keywords and operators.
- **Do:** `"Rust 2026 edition" "memory model" performance`
- **Don't:** `"What were the major changes in the Rust 2026 edition regarding the memory model?"`

## Context Management (`exclude_rules`)

When fetching URLs, the `cluaize-search` engine automatically strips HTML. However, many sites contain heavy boilerplate (navbars, footers, ad blocks) that can pollute your context window and dilute the `entropy_threshold`.

To prevent this, you can pass `exclude_rules` in your CEL command.
```cel
let $site_data = use extension::cluaize-search -> fetch(
    url: "https://example.com/article",
    exclude_rules: "nav, footer, .ad-banner, #sidebar, [role='complementary']"
);
```

## Context Limits (`max_context_length`)

To protect your VRAM, you can forcefully truncate the returned HTML string.
```cel
let $results = use extension::cluaize-search -> query(
    q: "Rust memory model",
    max_context_length: 4096
);
```
*Always use `max_context_length` when you are in `think_mode: OFF` to prevent the engine from spending excess time injecting massive context blocks.*
