---
title: "Web Intelligence (cluaize-search)"
version: "1.0.0"
description: "Native plugin for real-time metasearch, deep HTML parsing, and URL summarization."
author: "Cluaiz Technologies"
soul_type: "PROMPT_CACHE"
keywords: ["search", "web", "fetch", "url", "news", "look up", "google"]
triggers:
  semantic: ["search the web", "look up", "fetch url", "summarize website"]
  entropy_threshold: 0.82
permissions:
  level: "ReadOnly"
  filesystem: false
  network: true
core_metadata:
  token_count: 480
---

# Web Intelligence Skill (cluaize-search)

You are equipped with the Cluaize Native Metasearch extension. This gives you the power to execute real-time web searches and fetch raw content from specific URLs safely and efficiently, directly bypassing hallucination.

## Core Directives

You are reading this because you emitted the `<TRIGGER:extension:cluaize-search>` token. The Engine has executed an Agentic Pause and injected this schema into your context. You must now invoke the search tool with the appropriate parameters based on the user's original request.

DO NOT hallucinate facts. DO NOT write python or bash scripts to scrape websites. The `cluaize-search` extension handles all HTML stripping, context compression, and metadata extraction internally via Rust.

### 1. General Search
When the user asks a question (e.g., "What is the latest Rust version?"), invoke the search tool with a query string.

**Query Guidelines:**
- The engine will return an array of results with extracted content.
- Do not search for conversational phrases. Extract the core entities.
- Example: "Who won the superbowl in 2026?" -> Trigger search with query `"Superbowl winner 2026"`.

### 2. URL Extraction
When the user provides an exact URL (e.g., "Summarize https://github.com"), invoke the search tool with the URL.

**Fetch Guidelines:**
- Use this ONLY when you have an exact HTTP/HTTPS URL.
- The engine will return clean, compressed text extracted from the webpage, omitting boilerplate HTML.

## System Settings Integration (MUST FOLLOW)

Your behavior and query construction MUST dynamically adapt to the current Engine `system_booster` settings injected into your context.

### Deep Thinking Mode (`think_mode: ON`)

When you detect you are in Deep Thinking mode:

1. **Multi-Aspect Queries:** Do not settle for simple keywords. Break the user's complex intent down and fetch deep context. Example: Instead of `"Rust async"`, use `"Rust 2026 async updates OR memory model changes performance benchmarks"`.
2. **Iterative Searching:** If the first result does not contain the answer, you must execute a follow-up query with refined terms.
3. **Synthesis:** You must synthesize the data comprehensively, comparing multiple sources.
4. **Reference Material:** If you are unsure about provider limitations or heuristics (Tavily vs DuckDuckGo), read the `references/search_heuristics.md` file.
5. **Use CEL Macros:** For deep searching, do not write raw CEL. Execute the pre-built macro: `run scripts/deep_search_macro.cel`.

### Fast Mode (`think_mode: OFF`)

When you are in Fast (Zero-Latency) mode:

1. **Precise Queries:** Keep your queries extremely short, exact, and highly specific to minimize execution time. Example: `"Rust latest version release date"`.
2. **Single Shot:** Do not perform iterative searches. Rely on the first batch of results.
3. **Use CEL Macros:** For fast queries, execute the pre-built macro: `run scripts/fast_query_macro.cel`.
4. **Direct Answers:** Provide the answer immediately without long-winded explanations.

## Rules for Response

1. Wait for the engine to execute the search and inject the results into your context window.
2. If the search returns an error or empty results, explicitly inform the user that the search failed and suggest alternative phrasing.
3. **Always cite your sources** by providing the URLs retrieved from the search payload at the end of your response.
4. If you fetch a YouTube link, you will only receive OpenGraph metadata (title, description). Do not attempt to summarize the video transcript unless explicitly told the transcript is available.
