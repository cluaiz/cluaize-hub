---
title: "Extracting Settings in WASM"
description: "How to use system_bindings to fetch variables via MsgPack in a WASM Plugin."
category: "Tutorials"
---

# 2. Fetching Settings & Variables in WASM

Because plugins run inside the strict `Wasmtime` VM (sandbox), they cannot read `.json` configuration files directly from the host machine's disk. 

Instead, plugins request `system_bindings` in their manifest, and the Cluaize Engine bundles those settings into a `MsgPack` payload, passing a pointer directly into the WASM module.

---

## Step 1: Subscribe to Variables

In your `manifest-plugin.yaml`, declare the hardware or booster variables you need.

```yaml
# ---------------------------------------------------------------------
# SYSTEM BINDINGS (Dynamic Hardware & Booster State)
# ---------------------------------------------------------------------
system_bindings:
  - "system_booster.think_mode"
  - "system_control.silicon_truth.cpu.logical_threads"
```

## Step 2: Deserializing MsgPack in Rust

The Engine serializes the data according to the `payload_format: "MsgPack"` rule defined in the manifest. We must use `rmp_serde` to decode it inside the WASM environment.

Update your `src/lib.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::ffi::{c_char, CString};

// 1. Define the Expected Data Structure
#[derive(Deserialize, Debug)]
struct CelPayload {
    action: String,
    system_booster: Option<serde_json::Value>,
    system_control: Option<serde_json::Value>,
    // Your specific plugin arguments
    expr: Option<String>, 
}

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

    // 2. Deserialize the MsgPack Payload
    let payload: CelPayload = match rmp_serde::from_slice(incoming_bytes) {
        Ok(p) => p,
        Err(_) => {
            let error = r#"{"status":"error", "message":"MsgPack parse failed"}"#;
            return CString::new(error).unwrap().into_raw();
        }
    };

    // 3. Extract the System Bindings safely
    let think_mode = payload.system_booster
        .as_ref()
        .and_then(|b| b.get("think_mode"))
        .and_then(|t| t.as_str())
        .unwrap_or("Off");

    // Plugin Logic Goes Here...
    let response = format!(
        r#"{{"status": "success", "think_mode": "{}", "expression_received": "{:?}"}}"#, 
        think_mode, payload.expr
    );
    
    CString::new(response).unwrap_or_default().into_raw()
}
```

> [!IMPORTANT]
> The WASM module must return a dynamically allocated `CString` (which creates a memory leak in WASM linear memory). To fix this, you must also export a `cluaize_free_payload` function so the Engine can free it!

```rust
#[no_mangle]
pub extern "C" fn cluaize_free_payload(ptr: *mut c_char, _len: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
```
