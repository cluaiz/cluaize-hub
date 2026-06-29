# 🔌 Cluaize Hub: Plugins

Welcome to the **Plugins** directory of the Cluaize Hub.

## What is a Plugin?
A Plugin represents **"The Functional Muscle"** of the Cluaize Ecosystem. 

Plugins are standalone, compiled third-party binaries (`.dll`, `.wasm`, `.so`) that give the Engine specific hardware, network, or OS-level capabilities that it otherwise wouldn't have.

### Key Characteristics
1. **No Brains:** Plugins do NOT contain a `SKILL.md`. They do not teach the AI how to use them.
2. **Pure Execution:** They simply accept a payload via the CXP (Cluaize Extension Protocol) C-FFI boundary, execute it at bare-metal speeds, and return the pointer.
3. **Triggered by Skills/Extensions:** A Plugin is usually invoked by an AI that has been trained by a separate Skill or Extension.

### When to use a Plugin?
If you are writing a pure, highly-optimized algorithm (like an image-processing function or a web scraper) that doesn't need to inject custom context into the LLM, it belongs here.

> **Rule of Thumb:** If your module is just a bare-metal `.dll` or `.wasm` binary meant to be executed blindly by the engine, it belongs in `plugins`. If it also requires teaching the AI custom syntax, it belongs in `Extensions`.
