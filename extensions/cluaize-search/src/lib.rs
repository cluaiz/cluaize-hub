use serde_json::{json, Value};
use std::os::raw::c_char;
use std::ffi::{CStr, CString};

pub mod search_engine;
pub mod parser;
pub mod config;

use search_engine::multiplexer::Multiplexer;
use parser::{stripper::Stripper, ranker::Ranker, metadata::MetadataExtractor};

/// The main FFI entry point for the cluaiz engine to call into this native extension.
/// Engine calls this via `UnifiedExecutor::execute()` after parsing CEL itself.
/// The extension NEVER parses CEL — the engine handles that in `ffi_bridge.rs`.
#[no_mangle]
pub extern "C" fn execute_cel(cel_command_ptr: *const c_char) -> *mut c_char {
    if cel_command_ptr.is_null() {
        return create_error_response("Null pointer received for CEL command");
    }

    // SAFETY: Engine guarantees valid C string from UnifiedExecutor
    let c_str = unsafe { CStr::from_ptr(cel_command_ptr) };
    let cel_json = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return create_error_response("Invalid UTF-8 in CEL command pointer"),
    };

    let command: Value = match serde_json::from_str(cel_json) {
        Ok(v) => v,
        Err(_) => return create_error_response("Failed to parse CEL JSON payload"),
    };

    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(e) => return create_error_response(&format!("Failed to spawn tokio runtime: {}", e)),
    };

    let result = rt.block_on(async {
        process_command(command).await
    });

    let c_response = CString::new(result.to_string()).unwrap_or_else(|_| CString::new("{}").unwrap());
    c_response.into_raw()
}

/// Core async logic to process the parsed CEL command
async fn process_command(command: Value) -> Value {
    let action = command.get("action").and_then(|a| a.as_str()).unwrap_or("");
    let target = command.get("target").and_then(|t| t.as_str()).unwrap_or("");
    
    // Dynamic parameters passed from Engine's CEL payload
    let max_ram_mb = command.get("max_ram_mb").and_then(|r| r.as_u64()).unwrap_or(4096) as usize;
    let timeout_secs = command.get("timeout_secs").and_then(|t| t.as_u64()).unwrap_or(10);
    let exclude_rules = command.get("exclude_rules").and_then(|r| r.as_str()).unwrap_or("");
    let max_context_length = command.get("max_context_length").and_then(|m| m.as_u64()).unwrap_or(0) as usize;
    
    let custom_mirrors = command.get("mirrors").and_then(|m| m.as_array()).map(|arr| {
        arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<String>>()
    });

    let config = match config::get_dynamic_config().await {
        Ok(c) => c,
        Err(e) => {
            return json!({ "status": "error", "message": format!("Failed to fetch dynamic config: {}", e) });
        }
    };

    match action {
        "query" => {
            let start_time = std::time::Instant::now();
            match Multiplexer::fetch_query(target, timeout_secs, custom_mirrors, &config.search_api_key, &config.search_api_type, config.think_mode, &config.response_length).await {
                Ok(mut results) => {
                    for result in &mut results {
                        if let Some(html) = result.raw_content.take() {
                            let clean_text = Stripper::clean_html(&html, exclude_rules);
                            let compressed = Ranker::compress_context(&clean_text, target, max_ram_mb);
                            let final_text = apply_context_limit(&compressed, max_context_length);
                            result.raw_content = Some(final_text);
                        }
                    }
                    let execution_time_sec = start_time.elapsed().as_secs_f64();
                    json!({ 
                        "status": "success", 
                        "metadata": {
                            "provider": config.search_api_type,
                            "think_mode": config.think_mode_str,
                            "response_length": config.response_length,
                            "results_count": results.len(),
                            "execution_time_sec": execution_time_sec
                        },
                        "results": results 
                    })
                },
                Err(e) => json!({ "status": "error", "message": e })
            }
        },

        "fetch" => {
            let is_youtube = is_youtube_url(target);
            match Multiplexer::fetch_url(target, timeout_secs).await {
                Ok(raw_html) => {
                    let metadata = MetadataExtractor::extract(&raw_html);
                    
                    // YouTube: Only return OG metadata, skip full content extraction (zero-bloat)
                    if is_youtube {
                        json!({ 
                            "status": "success", 
                            "source": "youtube_og_only",
                            "metadata": metadata
                        })
                    } else {
                        let clean_text = Stripper::clean_html(&raw_html, exclude_rules);
                        let compressed = Ranker::compress_context(&clean_text, "", max_ram_mb);
                        let final_text = apply_context_limit(&compressed, max_context_length);
                        json!({ 
                            "status": "success", 
                            "metadata": metadata,
                            "content": final_text 
                        })
                    }
                },
                Err(e) => json!({ "status": "error", "message": e })
            }
        },
        _ => json!({ "status": "error", "message": format!("Unknown action: {}", action) })
    }
}

/// Detects if a URL is a YouTube link
fn is_youtube_url(url: &str) -> bool {
    let lower = url.to_lowercase();
    lower.contains("youtube.com") || lower.contains("youtu.be")
}

/// Applies developer-specified max_context_length limit from CEL payload
fn apply_context_limit(text: &str, max_len: usize) -> String {
    if max_len > 0 && text.len() > max_len {
        text.chars().take(max_len).collect()
    } else {
        text.to_string()
    }
}

fn create_error_response(msg: &str) -> *mut c_char {
    let err = json!({ "status": "error", "message": msg }).to_string();
    CString::new(err).unwrap().into_raw()
}

/// Memory cleanup for the FFI boundary
#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
