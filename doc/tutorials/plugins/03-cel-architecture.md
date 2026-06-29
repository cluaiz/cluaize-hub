---
title: "CEL vs WASM Execution"
description: "Why WASM plugins must never parse CEL and how the Engine bridges the two."
category: "Tutorials"
---

# 3. CEL Architecture & WASM Limits

A common misconception is that Plugins (WASM) evaluate the AI's CEL scripts directly. This is fundamentally impossible due to strict sandboxing.

### Anti-Pattern: Parsing CEL inside the WASM Plugin
> [!WARNING]
> **Never import the `inference-cel` compiler into your WASM plugin.**
> Doing so will drastically inflate your `.wasm` binary size and violate the execution sandbox. The Engine is the "Brain" responsible for parsing CEL. The Plugin is the "Muscle" that simply receives arguments.

## How The AI Evaluates a Tool

When the AI writes a CEL script, it looks like this:

```cel
let $math = use plugin::cluaize-math-accelerator -> invoke(calculate, expr: "10 * 5");
```

### The Separation of Concerns

1. **The Engine (Rust Daemon)** runs the Lexer, Parser, and AST compilation.
2. The Engine identifies `cluaize-math-accelerator` and reads its `manifest-plugin.yaml`.
3. The Engine converts `{"action": "calculate", "expr": "10 * 5"}` into **MsgPack**.
4. The Engine instantiates the **Wasmtime VM** and injects the MsgPack pointer.
5. The WASM Plugin computes `50` and returns it.

## When to Use CEL vs IPC?

If you are developing a tool, you might wonder if you should use CEL or direct Named Pipe IPC to test it.

| Approach | Latency | Use Case |
|----------|---------|----------|
| **CEL Script Execution** | ~2ms - 5ms | Used by the **AI LLM** to chain multiple tools together dynamically. |
| **Named Pipe IPC** | **< 1ms** | Used by **Engineers** to write unit tests or query the engine directly, bypassing the AST overhead. |
