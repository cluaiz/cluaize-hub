use cluaiz_search::execute_cel;
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::io::{self, Write};

fn main() {
    println!("==================================================");
    println!("🔍 cluaiz Search Extension - Interactive Test 🔍");
    println!("==================================================");

    loop {
        print!("\nEnter a URL or Search Query (or type 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        if input.is_empty() {
            continue;
        }

        // Determine action based on input (if it starts with http, it's a fetch, otherwise query)
        let action = if input.starts_with("http://") || input.starts_with("https://") {
            "fetch"
        } else {
            "query"
        };

        // In a real execution, the cluaiz Engine passes these dynamically via the C-Pointer FFI.
        // Since this is a standalone test, we act as the Engine and mock the payload.
        let think_mode = "Auto";
        let response_length = "auto";
        
        let manifest_str = std::fs::read_to_string("manifest-extension.yaml").unwrap_or_default();
        let manifest: Value = serde_yaml::from_str(&manifest_str).unwrap_or(serde_json::json!({}));
        let search_api_key = manifest
            .get("settings")
            .and_then(|s| s.get("search_api_key"))
            .and_then(|k| k.as_str())
            .unwrap_or("");
        let search_api_type = manifest
            .get("settings")
            .and_then(|s| s.get("search_api_type"))
            .and_then(|t| t.as_str())
            .unwrap_or("duckduckgo");

        let payload = serde_json::json!({
            "action": action,
            "target": input,
            "timeout_secs": 15,
            "max_context_length": 500, // Limiting for terminal display
            "search_api_key": search_api_key,
            "search_api_type": search_api_type,
            "system_booster": {
                "think_mode": think_mode,
                "response_length": response_length
            }
        });

        println!(
            "\n⏳ Running {} for: '{}' (Provider: {}, Think Mode: {}) ...\n",
            action, input, search_api_type, think_mode
        );

        let json_str = payload.to_string();
        let c_str = CString::new(json_str).unwrap();
        let result_ptr = execute_cel(c_str.as_ptr());

        let result_c_str = unsafe { CStr::from_ptr(result_ptr) };
        let result_str = result_c_str.to_str().unwrap().to_string();
        cluaiz_search::free_string(result_ptr);

        match serde_json::from_str::<Value>(&result_str) {
            Ok(parsed) => {
                if parsed["status"] == "success" {
                    println!("✅ SUCCESS!\n");
                    let formatted_json =
                        serde_json::to_string_pretty(&parsed).unwrap_or_else(|_| "[]".to_string());
                    println!("{}", formatted_json);

                    // Also save to a file for easy viewing
                    let safe_name =
                        input.replace(&['/', ':', '\\', '?', '*', '<', '>', '|', '"'][..], "_");
                    let mut filename = safe_name.clone();
                    filename.truncate(20);
                    let filepath = format!("rust_search_{}.json", filename);
                    std::fs::write(&filepath, formatted_json).unwrap();
                    println!("\n💾 Saved full result to: {}", filepath);
                } else {
                    println!(
                        "❌ ERROR JSON DUMP: {}",
                        serde_json::to_string_pretty(&parsed)
                            .unwrap_or_else(|_| "Unknown error".to_string())
                    );
                }
            }
            Err(e) => {
                println!("❌ FAILED TO PARSE JSON: {}", e);
                println!("RAW: {}", result_str);
            }
        }
    }
}
