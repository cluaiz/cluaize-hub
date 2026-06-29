---
title: "Build Your First WASM Plugin"
description: "How to compile WebAssembly (WASM) plugins for sandboxed execution."
category: "Tutorials"
---

# 1. Build Your First Plugin (WASM / C++)

In the Cluaize ecosystem, a **Plugin** is distinct from an Extension. While Extensions are Native DLLs providing raw OS access, Plugins are **Compiled WebAssembly (WASM)** modules (`type: plugin`). 

Plugins run entirely within the Engine's WASM sandbox. They are meant for "pure muscle" tasks (e.g., fast math, text parsing) and have absolutely no AI prompt or "brain" attached.

> [!TIP]
> **For more details, reference:** [`skill_architecture.md`](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaize-hub/doc/architecture/skill_architecture.md)
> 
> For **Plugins**, creating a `SKILL.md` file is **OPTIONAL but RECOMMENDED**. While a plugin can execute blindly, providing a `SKILL.md` teaches the AI how to use it properly and gives the AI the power to understand the tool's context.

---

## The WASM Architecture

```mermaid
flowchart TD
    A["Cluaize Engine"] -->|CEL Router Identifies Plugin| B{"Wasmtime VM"}
    B -->|Compiles ExtensionPayload (MsgPack)| C["Compiled Plugin (.wasm)"]
    C -->|Executes Sandbox Logic| B
    B -->|Returns CString| A
```

> [!NOTE]
> Plugins are highly secure. Since they execute inside the `Wasmtime` VM, they cannot access the host operating system, read local files, or crash the Engine memory unless explicitly granted permission.

## Step 1: The Manifest

Every plugin requires a `manifest-plugin.yaml`. The Engine uses this to configure the WASM limits and entry points.

```yaml
# =====================================================================
# CLUAIZE PLUGIN MANIFEST (manifest-plugin.yaml)
# =====================================================================
name: "cluaize-math-accelerator"
version: "1.0.0"
description: "A hardware-accelerated math parsing plugin compiled to WASM."
author: "Cluaiz Engineers"
type: "plugin"

discovery:
  # Keywords that will be injected into the AI's prompt. 
  semantic_triggers: ["math", "calculation", "formula"]

activation:
  lazy_load: true
  trigger_on: 
    - "on_command:use plugin::cluaize-math-accelerator"

permissions:
  # Hard RAM limit (in MB). The Engine will OOM kill the WASM if exceeded.
  max_memory_mb: 64
  # Max CPU execution time (ms). To prevent infinite loops.
  max_cpu_time_ms: 500
  network_access: false
  vram_kv_inject: false
  file_system: "none"

execution:
  # Sandbox type. "WASM" (Strict limits)
  envelope: "WASM"
  # The exported C-language function name. The Engine will call this.
  entry_point: "cluaize_entry"
  # How to serialize the data. "MsgPack" or "JSON".
  payload_format: "MsgPack"
```

## Step 2: Scaffold the Rust WASM Project

To build a plugin in Rust, compile to the `wasm32-wasi` target.

```bash
cargo new --lib math_plugin
cd math_plugin
rustup target add wasm32-wasi
```

Update your `Cargo.toml`:
```toml
[package]
name = "math_plugin"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
rmp-serde = "1.1" # For MsgPack parsing
```

## Step 3: The Code (MsgPack via C-Pointer)

Inside `src/lib.rs`, expose the `cluaize_entry` function. Unlike basic strings, the Engine passes a structured `ExtensionPayload` pointer containing the `MsgPack` serialized data.

```rust
use std::ffi::{c_char, CString};

// The data structure passed by the Engine
#[repr(C)]
pub enum PayloadType { Json, Cdql, WasmBinary, RawBytes, Bincode, MsgPack }

#[repr(C)]
pub struct ExtensionPayload {
    pub payload_type: PayloadType,
    pub data_ptr: *const u8,
    pub data_len: usize,
}

#[no_mangle]
pub extern "C" fn cluaize_entry(payload_ptr: *const ExtensionPayload) -> *mut c_char {
    if payload_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let payload_ref = unsafe { &*payload_ptr };
    let incoming_bytes = unsafe {
        std::slice::from_raw_parts(payload_ref.data_ptr, payload_ref.data_len)
    };

    // Deserialize the MsgPack data passed from the Engine
    // (Logic will be expanded in the Next Tutorial)

    let response = r#"{"status": "success", "message": "WASM execution complete."}"#;
    CString::new(response).unwrap_or_default().into_raw()
}
```

To compile:
```bash
cargo build --target wasm32-wasi --release
```
