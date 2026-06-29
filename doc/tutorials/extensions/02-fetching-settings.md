---
title: "Fetching Settings & Configs"
description: "How to use manifest-extension.yaml to request system configurations without hardcoding file paths."
category: "Tutorials"
---

# 2. Fetching Settings & Configurations

In traditional software, your plugin might read a `.json` file from the disk to get its settings. **In Cluaize, this is strictly forbidden.** Extensions operate in a secure boundary and should never perform blind disk I/O to read Engine configs.

Instead, you **request** what you need using a `manifest-extension.yaml` file, and the Engine injects it directly into your payload.

---

## Step 1: The Manifest File

Create a `manifest-extension.yaml` in the root of your extension folder.

```yaml
version: "1.0.0"
description: "Extension that requires system booster settings"
type: "extension"

# Request system settings to be injected by the Engine
system_bindings:
  - "system_booster.think_mode"
  - "permission.max_memory_mb"

execution:
  envelope: "NATIVE"
  entry_point: "execute_cel"
  binary_path: "target/release/my_first_extension.dll"
```

### What happens here?
When the AI Agent triggers your extension, the Engine reads `system_bindings`. It opens `system_booster.json`, extracts `think_mode`, and bundles it into the C-Pointer payload sent to your extension.

---

## Step 2: Extracting Values in Rust

Because the Engine did the hard work of file I/O, your extension simply reads from the JSON payload.

Update your `src/lib.rs`:

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use serde_json::Value;

#[no_mangle]
pub extern "C" fn execute_cel(payload_ptr: *const c_char) -> *mut c_char {
    if payload_ptr.is_null() {
        return CString::new("{}").unwrap().into_raw();
    }

    let c_str = unsafe { CStr::from_ptr(payload_ptr) };
    let json_str = c_str.to_str().unwrap_or("{}");

    // Deserialize the Engine's payload
    if let Ok(command) = serde_json::from_str::<Value>(json_str) {
        
        // Extract the variables requested in manifest
        let think_mode = command.get("system_booster")
            .and_then(|b| b.get("think_mode"))
            .and_then(|t| t.as_str())
            .unwrap_or("Off");

        println!("Engine provided think_mode: {}", think_mode);
        
        let response = serde_json::json!({ 
            "status": "success", 
            "active_mode": think_mode 
        });
        
        return CString::new(response.to_string()).unwrap().into_raw();
    }

    CString::new("{}").unwrap().into_raw()
}
```

> [!TIP]
> **No Path Hardcoding:** By relying on `system_bindings`, your extension works cross-platform (Windows/Linux) without worrying about where `system_booster.json` is physically located on the user's hard drive.
