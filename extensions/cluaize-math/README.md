# 🧩 cluaiz Math Extension (WASM Sandbox Demo)

This extension is the **WASM Sandboxed** dynamic plugin template for cluaiz.

## Overview
Unlike `cluaiz-db` (which is a native compiled `.dll` with unrestricted OS access), `cluaiz-math` represents a **Non-Built (Hot-Loaded) Extension** designed for safety and ease of distribution.

- **No Local Compilation:** The host engine loads a pre-compiled WebAssembly bytecode (`bin/plugin.wasm`) directly into the sandboxed Wasmtime engine.
- **Strict Isolation:** The execution is capped at a 32MB memory limit and a 100,000 instruction budget (`fuel_limit`) dynamically enforced by the engine rules to guarantee zero memory leaks or host crash vulnerability.

## How it works (The No-Build Flow)
1. The user copies the plugin folder containing `manifest.yaml` and `bin/plugin.wasm` directly into `.cluaiz/extensions/`.
2. Upon engine boot, `cluaizExtensionRegistry` parses `manifest.yaml` and checks that the `sandbox_type` is `WASM`.
3. The engine initializes a safe `WasmExecutor` context, loads the `bin/plugin.wasm` file dynamically, and routes matching CEL expressions directly through the WebAssembly FFI interface.
4. **No local compiler, Cargo setup, or submodules are required.**
