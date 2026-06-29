# The Cluaiz Build Pipeline

One of the key features of the Cluaiz build pipeline is that **developers do not need to manually compile native binaries or generate neural tensors.** 

The entire build process is handled automatically by the CI/CD pipeline (GitHub Actions).

## How it works

When a developer creates a new skill (e.g., `skills/web-intelligence/frontend-dev`), they only push the source code and prompts.

### 1. `logic.wasm` (The Execution Engine)
- **What the developer writes:** A `src/` folder containing Rust or C++ source code (e.g., `main.rs` for an AST parser).
- **What the CI pipeline does:** 
  1. Detects changes in the `src/` folder.
  2. Runs `cargo build --target wasm32-wasi --release`.
  3. Attaches the resulting `logic.wasm` binary to the GitHub Release.

### 2. `state.kvcache.bin` (The Neural Memory)
- **What the developer writes:** The `SKILL.md` file (containing the prompt) and any detailed guides in `references/`.
- **What the CI pipeline does:**
  1. Boots up a GPU-enabled runner with the Cluaiz inference engine.
  2. Runs a **Prefill Phase** on the `SKILL.md` prompt.
  3. Dumps the resulting Key-Value self-attention tensors into a binary file.
  4. Attaches this `state.kvcache.bin` file to the GitHub Release.

## The End User Experience

Because of this automated pipeline, the Cluaiz repository remains clean and free of large binary files.

When an end user runs the install command:
```bash
cluaiz skill install frontend-dev
```
The CLI automatically downloads the `SKILL.md` from the source tree, and pulls the pre-built `logic.wasm` and `state.kvcache.bin` directly from the latest GitHub Release, dropping them into `~/.cluaiz/skills/frontend-dev/`.

This ensures **Zero-Token Tax** execution on the edge device, with instantaneous skill loading.
