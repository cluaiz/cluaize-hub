use cluaiz_search::execute_cel;
use serde_json::Value;
use std::ffi::{CStr, CString};

/// Helper to save output to tests/output directory
fn save_test_output(test_name: &str, content: &str) {
    let _ = std::fs::create_dir_all("tests/output");
    let filepath = format!("tests/output/{}.txt", test_name);
    let _ = std::fs::write(&filepath, content);
}

/// Helper to call the FFI boundary
fn call_execute_cel(payload: serde_json::Value) -> String {
    let json_str = payload.to_string();
    let c_str = CString::new(json_str).unwrap();

    // Call the FFI function
    let result_ptr = execute_cel(c_str.as_ptr());

    // Read the string back
    let result_c_str = unsafe { CStr::from_ptr(result_ptr) };
    let result_str = result_c_str.to_str().unwrap().to_string();

    // Free the string
    cluaiz_search::free_string(result_ptr);

    result_str
}

/// Dynamic helper to read manifest
fn get_manifest_config() -> (usize, u64, Vec<String>) {
    let yaml_str = std::fs::read_to_string("manifest-extension.yaml")
        .expect("Failed to read manifest-extension.yaml");
    let manifest: Value = serde_yaml::from_str(&yaml_str).expect("Failed to parse YAML");

    let max_ram_mb = manifest["permissions"]["max_memory_mb"]
        .as_u64()
        .unwrap_or(256) as usize;
    // max_cpu_time_ms / 1000 = timeout_secs
    let timeout_secs = manifest["permissions"]["max_cpu_time_ms"]
        .as_u64()
        .unwrap_or(5000)
        / 1000;

    let mut bindings = Vec::new();
    if let Some(arr) = manifest["system_bindings"].as_array() {
        for v in arr {
            if let Some(s) = v.as_str() {
                bindings.push(s.to_string());
            }
        }
    }

    (max_ram_mb, timeout_secs, bindings)
}

#[test]
fn test_stripper_advanced_rules() {
    let raw_html = r#"
        <html>
            <head><title>Test Page</title></head>
            <body>
                <nav>Navigation Links</nav>
                <div class="content">
                    <h1>Main Heading</h1>
                    <p>This is the important text.</p>
                </div>
                <aside class="sidebar">Ads and stuff</aside>
                <script>console.log("tracker");</script>
                <footer>Copyright 2026</footer>
            </body>
        </html>
    "#;

    let cleaned_default = cluaiz_search::parser::stripper::Stripper::clean_html(raw_html, "");
    assert!(!cleaned_default.contains("console.log"));

    let exclude_rules = "script, style, nav, aside.sidebar, footer";
    let cleaned_custom =
        cluaiz_search::parser::stripper::Stripper::clean_html(raw_html, exclude_rules);

    assert!(!cleaned_custom.contains("Navigation Links"));
    assert!(!cleaned_custom.contains("Ads and stuff"));
    assert!(cleaned_custom.contains("Main Heading"));

    save_test_output("test_stripper_advanced_rules", &cleaned_custom);
}

#[test]
fn test_fetch_valid_url() {
    let (max_ram_mb, timeout_secs, _) = get_manifest_config();
    let payload = serde_json::json!({
        "action": "fetch",
        "target": "https://example.com",
        "timeout_secs": timeout_secs,
        "max_ram_mb": max_ram_mb,
        "max_context_length": 200,
        "exclude_rules": "style, script"
    });

    let res = call_execute_cel(payload);
    save_test_output("test_fetch_valid_url", &res);
    let parsed: Value = serde_json::from_str(&res).unwrap();
    assert_eq!(parsed["status"], "success");
}

#[test]
fn test_fetch_invalid_url() {
    let (_, timeout_secs, _) = get_manifest_config();
    let payload = serde_json::json!({
        "action": "fetch",
        "target": "https://this-url-is-definitely-fake-cluaiz.dev",
        "timeout_secs": timeout_secs
    });

    let res = call_execute_cel(payload);
    save_test_output("test_fetch_invalid_url", &res);
    let parsed: Value = serde_json::from_str(&res).unwrap();
    assert_eq!(parsed["status"], "error");
}

#[test]
fn test_fetch_youtube_zero_bloat() {
    let (_, timeout_secs, _) = get_manifest_config();
    let payload = serde_json::json!({
        "action": "fetch",
        "target": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        "timeout_secs": timeout_secs
    });

    let res = call_execute_cel(payload);
    save_test_output("test_fetch_youtube_zero_bloat", &res);
    let parsed: Value = serde_json::from_str(&res).unwrap();

    if parsed["status"] == "success" {
        assert_eq!(parsed["source"], "youtube_og_only");
    }
}

#[test]
fn test_query_searxng_with_system_bindings() {
    let (max_ram_mb, timeout_secs, bindings) = get_manifest_config();

    let mut payload = serde_json::json!({
        "action": "query",
        "target": "Rust programming language",
        "timeout_secs": timeout_secs,
        "max_ram_mb": max_ram_mb,
        "max_context_length": 300,
    });

    // Inject dynamic system bindings from yaml
    for binding in bindings {
        payload[binding] = serde_json::json!("Off");
    }

    let res = call_execute_cel(payload);
    save_test_output("test_query_searxng_with_system_bindings", &res);
    let parsed: Value = serde_json::from_str(&res).unwrap();

    if parsed["status"] == "success" {
        let text = parsed["results"].as_str().unwrap();
        assert!(text.len() <= 300);
    }
}

#[test]
fn test_unknown_action() {
    let payload = serde_json::json!({
        "action": "hack_the_mainframe",
        "target": "localhost"
    });

    let res = call_execute_cel(payload);
    save_test_output("test_unknown_action", &res);
    let parsed: Value = serde_json::from_str(&res).unwrap();
    assert_eq!(parsed["status"], "error");
}
