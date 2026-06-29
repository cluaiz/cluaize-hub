# 🧩 Cluaize DB Extension (Native Bridge)

This extension is the **Native Muscle** for the Cluaize Neural Database.

## Overview
Unlike standard scripts, `cluaize-db` is a **Native Extension** that acts as the FFI bridge between the `cluaize` Inference Engine and the `cluaizd` (Zero-Copy LMDB) Database Engine. 

By placing this inside the `Cluaize Hub`, we successfully decouple the massive storage dependencies from the Core Engine. The Core Engine dynamically loads this extension's `.dll` (`cluaizd_engine.dll`) at runtime to achieve $0\text{-ms}$ memory-mapped FFI database injection.

## Project Structure
- `native/` - The Rust C-FFI crate that statically links against `cluaizd` and `engine-lmdb`. It outputs a `.dll` that the Core Engine dynamically loads.
- `SKILL.md` - The Brain prompt that teaches the AI how to use this Database via CDQL.

---

## 🧠 CEL & FFI Execution Lifecycle (`execute_cel` Demo)

Here is a detailed walk-through of how a raw Cluaize Expression Language (CEL) command is parsed, translated, and executed through the dynamic `execute_cel` FFI boundary.

### 1. The CEL Expression Input
When the LLM or a user runs a database operation, they write standard CEL syntax:
```cel
use extension::cluaize-db -> save(memory_id: "user_session_42", payload: "User logged in from Windows IP")
```

### 2. Translation to JSON Payload Envelope
The Cluaize Engine compiles this CEL AST and serializes it into a standard JSON payload (`CelPayload`). This is what gets sent over the C-FFI pointer boundary to `execute_cel`:

```json
{
  "action": "save",
  "memory_id": "user_session_42",
  "payload": "User logged in from Windows IP",
  "vector": [0.12, -0.45, 0.78, 0.05],
  "shard_index": null,
  "query": null
}
```

### 3. FFI Call Execution
The host engine dynamically resolves the exported symbol `execute_cel` from `cluaizd_engine.dll` and executes it, passing the JSON string pointer:
```rust
// FFI Invocation Signature:
// pub extern "C" fn execute_cel(payload_ptr: *const c_char) -> *mut c_char;

let json_raw_ptr = CString::new(json_payload_str).unwrap().into_raw();
let response_raw_ptr = unsafe { execute_cel(json_raw_ptr) };
```

### 4. Inside the DLL: Processing & LMDB Execution
Inside `lib.rs` of `cluaize-db`, the function does the following:
1. Deserializes the pointer `payload_ptr` into the `CelPayload` struct.
2. Matches `payload.action.as_str()` -> routes to `internal_save_context()`.
3. Shards the key `"user_session_42"` using SHA-256 and selects the correct LMDB shard.
4. Executes the database write, returning a `CelResponse` payload:

```json
{
  "status": "success",
  "message": "Execution successful",
  "data": "Saved successfully"
}
```

### 5. Memory Deallocation Handshake
Since the dynamic library allocates the returned string pointer on its own heap, the host engine must pass it back to the DLL's exported deallocator to prevent memory leaks:
```rust
// Free allocated response pointer
unsafe { free_cel_response(response_raw_ptr) };
```

---

## Build Instructions
To build the dynamic muscle library for this extension, run:
```bash
cd native
cargo build --release
```
The Core Engine will automatically resolve and load `cluaizd_engine.dll` from the `target/release/` directory when `cluaizd_connect_ffi` is enabled.
