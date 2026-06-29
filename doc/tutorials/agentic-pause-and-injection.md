---
title: "Agentic Pause & Mid-Layer Injection"
description: "How the Cluaize Engine halts generation, injects tool schemas, and resumes inference via C-FFI interception."
category: "Extensions"
---

# Agentic Pause & Mid-Layer Injection

The Cluaize Engine does not rely on standard REST API-based tool calling for native extensions. Instead, it uses a highly optimized C-FFI interception mechanism known as an **Agentic Pause** to dynamically inject instructions (like `SKILL.md`) into the AI's context window exactly when needed.

This document explains the reality of how AI triggers extensions in Cluaize.

## 1. The Trigger Token

When the AI determines it needs external data (e.g., searching the web), it does not emit a JSON payload or write raw CEL code immediately. Instead, it emits a specific, raw native token in its generation stream:

```
<TRIGGER:extension:cluaize-search>
```

This token is the core signaling mechanism.

## 2. The Agentic Pause (Interception)

Inside the Engine's dispatcher (`chat.rs`), there is a strict monitoring loop. When the dispatcher detects the `<TRIGGER:X:Y>` token pattern, it immediately **breaks the streaming loop**. 

The generation halts. This is the **Agentic Pause**. The user's screen pauses, and the Engine takes control.

## 3. Mid-Layer Dual-Cache Injection

Once paused, the Engine performs the following operations without the LLM's knowledge:
1. **Registry Lookup:** It queries the `MasterRegistry` for the requested extension (e.g., `cluaize-search`).
2. **Schema & Manual Fetch:** It reads the `manifest-extension.yaml` and the `SKILL.md` files directly from the extension's domain folder.
3. **Prompt Mutation:** The Engine modifies the ongoing conversation prompt, appending the schema:
   ```text
   [SYSTEM INJECTION: TOOL SCHEMA FOR cluaize-search]
   --- AI SKILL MANUAL ---
   <contents of SKILL.md>
   [SYSTEM: RESUME GENERATION]
   ```
4. **VRAM Optimization:** It triggers `agentic_pause_compile_cache` to calculate the newly expanded context length and dynamically allocate or swap KV Cache in VRAM.

## 4. Resuming Generation

After the injection and cache compilation are complete, the Engine re-invokes the dispatcher (`dispatch_stream`). 

The AI wakes up from the pause, reads the newly injected `SKILL.md`, and now understands exactly *how* to use the tool. It then generates the required parameters or query strings as specified by the `SKILL.md` guidelines.

## Why This Architecture?

By keeping `SKILL.md` out of the initial system prompt, Cluaize saves hundreds of context tokens. The AI only learns about an extension *after* it attempts to trigger it, ensuring zero VRAM waste for tools that are not used in a given session.
