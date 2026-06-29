---
title: "Get Started: Build Your First Extension"
description: "Learn what Cluaize Extensions are and how to scaffold your first Native DLL."
category: "Tutorials"
---

# 1. Get Started: Build Your First Extension

Welcome to the Cluaize Extension Developer Guide. In Cluaize, **Extensions** (often called plugins) are Native Dynamic-Link Libraries (`.dll` on Windows, `.so` on Linux). 

Unlike standard web APIs, Cluaize extensions run bare-metal. They receive data from the core Cluaize Engine via **C-Pointers (FFI Boundaries)**, making them extremely fast and memory-efficient.

> [!IMPORTANT]
> **For more details, reference:** [`skill_architecture.md`](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaize-hub/doc/architecture/skill_architecture.md)
> 
> For **Extensions**, creating a `SKILL.md` file alongside your DLL is a **STRICT REQUIREMENT**. The DLL provides the muscle, but without a `SKILL.md`, the AI has no "Brain" or context on how to use it. 

---

## The Core Concept

Before writing any code, it is critical to understand the data flow. Extensions do **not** run standalone servers. They are invoked directly by the Engine.

```mermaid
flowchart TD
    A["Cluaize Engine (Host)"] -->|Loads Extension| B{"Registry"}
    B -->|AI Decides to Use Tool| C["Payload Compiler"]
    C -->|Constructs Bincode Payload| D["Extension Boundary (C-Pointer)"]
    D -->|Calls execute_cel(ptr)| E["Your Extension (.dll)"]
    E -->|Returns CString| A
```

## Step 1: Scaffold the Project

Since extensions are native libraries, you will create a Rust `lib` crate.

```bash
cargo new --lib my_first_extension
cd my_first_extension
```

Update your `Cargo.toml` to compile as a dynamic library:

```toml
[package]
name = "my_first_extension"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
serde_json = "1.0"
```

## Step 2: The FFI Entry Point

Every extension must expose a public C-compatible function named `execute_cel`. The Engine will look for this exact function name in your DLL.

Create `src/lib.rs`:

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn execute_cel(payload_ptr: *const c_char) -> *mut c_char {
    if payload_ptr.is_null() {
        return CString::new("{}").unwrap().into_raw();
    }

    // Convert the C-Pointer into a Rust String
    let c_str = unsafe { CStr::from_ptr(payload_ptr) };
    let payload = c_str.to_str().unwrap_or("{}");

    println!("Engine sent: {}", payload);

    // Return a success JSON
    let response = r#"{"status": "success", "message": "Hello from Extension!"}"#;
    CString::new(response).unwrap().into_raw()
}
```

## Next Steps

You've built the foundation! But an extension is useless without data and configuration. Move on to **Part 2: Fetching Settings** to learn how to securely request configurations (like API keys and System Boosters) from the Engine.
