use serde_json::Value;
use std::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::windows::named_pipe::ClientOptions;

pub struct DynamicConfig {
    pub search_api_key: String,
    pub search_api_type: String,
    pub think_mode: bool,
    pub think_mode_str: String,
    pub response_length: String,
}

pub async fn get_dynamic_config() -> Result<DynamicConfig, String> {
    // 1. Read manifest to get the required API keys and settings
    let manifest_path = "manifest-extension.yaml";
    let mut search_api_key = String::new();
    let mut search_api_type = String::from("duckduckgo");

    if let Ok(yaml_str) = fs::read_to_string(manifest_path) {
        if let Ok(manifest) = serde_yaml::from_str::<Value>(&yaml_str) {
            if let Some(settings) = manifest.get("settings") {
                if let Some(key) = settings.get("search_api_key").and_then(|v| v.as_str()) {
                    search_api_key = key.to_string();
                }
                if let Some(api_type) = settings.get("search_api_type").and_then(|v| v.as_str()) {
                    search_api_type = api_type.to_string();
                }
            }
        }
    }

    // 2. Connect to the Engine directly via IPC
    let mut client = match ClientOptions::new().open(r"\\.\pipe\cluaize_engine_pipe") {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to connect to Engine IPC: {}", e)),
    };

    // 3. Send the request to the engine
    let request = serde_json::json!({ "action": "GET_SETTINGS" });
    if let Err(e) = client.write_all(request.to_string().as_bytes()).await {
        return Err(format!("Failed to send request to Engine: {}", e));
    }

    // 4. Read the Engine's response
    let mut response_bytes = Vec::new();
    let mut buf = vec![0; 8192];
    let engine_data: Value = loop {
        match client.read(&mut buf).await {
            Ok(n) if n == 0 => return Err("Engine closed pipe before sending full JSON".to_string()),
            Ok(n) => {
                response_bytes.extend_from_slice(&buf[..n]);
                if let Ok(parsed) = serde_json::from_slice::<Value>(&response_bytes) {
                    break parsed;
                }
            }
            Err(e) => return Err(format!("Failed to read from pipe: {}", e)),
        }
    };

    // 5. Extract system booster settings
    let booster_settings = &engine_data["booster"];
    
    let think_mode_str = booster_settings.get("think_mode")
        .and_then(|t| t.as_str())
        .or_else(|| booster_settings.get("think_mode").and_then(|t| t.get("state")).and_then(|s| s.as_str()))
        .unwrap_or("auto")
        .to_string();
        
    let think_mode = think_mode_str.to_lowercase() == "on";

    let response_length = booster_settings.get("response_length")
        .and_then(|r| r.as_str())
        .unwrap_or("auto")
        .to_string();

    Ok(DynamicConfig {
        search_api_key,
        search_api_type,
        think_mode,
        think_mode_str,
        response_length,
    })
}
