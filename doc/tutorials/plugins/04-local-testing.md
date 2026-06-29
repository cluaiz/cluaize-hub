---
title: "Testing WASM Plugins Locally"
description: "How to test WASM sandbox execution without booting the entire Engine."
category: "Tutorials"
---

# 4. Local Testing for WASM Plugins

When developing a WASM plugin, testing inside the Cluaize LLM stack is slow and inefficient. You should write unit tests that manually serialize MsgPack and pass it to your `cluaize_entry` function natively.

---

## The Integration Test

Create a `tests/wasm_execution_test.rs` file.

Since the WASM is compiled to `wasm32-wasi`, your host machine (Windows/Linux) running `cargo test` won't be able to run it unless you use a WASM runtime, OR you write a standard native `#[test]` that calls your internal logic before compilation to WASM.

The best practice is to extract your logic into a pure function and test that natively, simulating the MsgPack serialization.

```rust
use serde_json::json;
use rmp_serde::Serializer;
use serde::Serialize;
use math_plugin::{PayloadType, ExtensionPayload, cluaize_entry};

#[test]
fn test_plugin_logic_via_msgpack() {
    // 1. Create a mock payload matching the Engine's format
    let payload_json = json!({
        "action": "calculate",
        "expr": "10 * 5"
    });

    // 2. Serialize to MsgPack
    let mut msgpack_buf = Vec::new();
    payload_json.serialize(&mut Serializer::new(&mut msgpack_buf)).unwrap();

    // 3. Construct the FFI Struct
    let ext_payload = ExtensionPayload {
        payload_type: PayloadType::MsgPack,
        data_ptr: msgpack_buf.as_ptr(),
        data_len: msgpack_buf.len(),
    };

    // 4. Invoke the Entry Point directly
    let result_ptr = cluaize_entry(&ext_payload as *const _);
    
    // 5. Assert the result
    assert!(!result_ptr.is_null());
    let c_str = unsafe { std::ffi::CStr::from_ptr(result_ptr) };
    let json_resp: serde_json::Value = serde_json::from_str(c_str.to_str().unwrap()).unwrap();
    
    assert_eq!(json_resp["status"], "success");
    
    // 6. Free the memory (CRITICAL)
    math_plugin::cluaize_free_payload(result_ptr, 0);
}
```

> [!TIP]
> By passing a raw pointer to `cluaize_entry` in a native unit test, you guarantee that when the Engine does the exact same thing across the Wasmtime boundary, your deserialization logic will perfectly parse the MsgPack payload.
