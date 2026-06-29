# 🧩 Cluaize Hub: Extensions

Welcome to the **Extensions** directory of the Cluaize Hub.

## What is an Extension?
An Extension is the most powerful module in the Cluaize Ecosystem. It represents the **Sangam (Union) of the Brain and the Muscle**.

Unlike standalone plugins (which are just dumb execution binaries) or skills (which are just text instructions), an Extension bundles an entire subsystem together.

### The Anatomy of an Extension
Every folder in this directory MUST follow this strict structure:
```text
cluaize-database/          <-- The Extension Name
├── manifest.json          <-- Capabilities, memory limits, and permissions
├── SKILL.md               <-- [THE BRAIN] Teaches the AI how to use this extension
└── native/                <-- [THE MUSCLE] The actual Rust/C++ plugin logic
    ├── Cargo.toml
    └── src/
```

### How it Works
1. When an Extension is loaded, the Engine injects the `SKILL.md` into the AI's context.
2. The AI learns the vocabulary (e.g., CDQL commands) and outputs a CEL Query.
3. The Engine intercepts the CEL Query and routes it to the native binary inside the `native/` folder for bare-metal execution.

> **Rule of Thumb:** If your tool requires teaching the AI a custom syntax, managing memory limits, AND executing native code, it belongs in `Extensions`.
