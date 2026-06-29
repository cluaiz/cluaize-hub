---
title: "Understanding CEL Language"
description: "The role of CEL, when to use it, and why extensions should never parse it."
category: "Tutorials"
---

# 3. The CEL Architecture: Pros, Cons, and Usage

A common point of confusion is the **CEL (Cluaize Execution Language)**. Developers often ask: *"Should my extension parse CEL to get its settings?"*

**Short Answer:** No. Extensions never parse CEL.

## What is CEL?

CEL is a high-level orchestration language built strictly for the **AI Agent (LLM)**. It allows the LLM to chain multiple extensions together at zero-latency without waiting for JSON responses.

```cel
// Example of an AI Agent generating a CEL script:
let $user_data = use plugin::database -> invoke(get_user, id: "404");
if ($user_data.is_admin) {
    use plugin::system_control -> invoke(wipe_cache);
}
```

### The Separation of Concerns

```mermaid
flowchart LR
    A["AI Model"] -->|Generates CEL| B["Engine (inference-cel)"]
    B -->|Parses AST & Evaluates| C["Execution Planner"]
    C -->|Compiles to Bincode| D["C-Pointer (FFI)"]
    D -->|Raw Data| E["Your Extension DLL"]
```

1. **The Engine** reads CEL, evaluates variables, and resolves logic.
2. **Your Extension** only receives the final, compiled raw data.

---

## CEL vs. C-Pointer IPC: Which One to Use?

If you want to read a setting, test a plugin, or pass data, you have two approaches. Here is exactly when to use which:

| Feature | Direct C-Pointer / IPC Pipe | CEL Language API |
|---------|-----------------------------|------------------|
| **Speed** | **Blazing Fast (<1ms)** | Slower (~5ms) |
| **Complexity** | Simple JSON deserialization | Requires Lexer, AST Parser, Planner |
| **Who uses it?** | **Extension Developers & Local Tests** | **The AI Model / End Users** |
| **Data Format** | Raw Bytes / MsgPack / JSON | Abstract Syntax Trees (AST) |

### Anti-Pattern: Parsing CEL inside the Extension

> [!WARNING]
> **Never import `inference-cel` into your extension to parse settings.**
> Doing so bloats your DLL size, creates circular dependencies, and violates the FFI sandbox. Your extension is a "Muscle". The Engine is the "Brain". Let the Engine do the parsing.

### Summary
- If you are building the logic of an extension: **Rely on the C-Pointer payload.**
- If you are writing a prompt for the AI to execute multiple tasks: **Use CEL.**
