use serde_json::Value;
use std::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(windows)]
use tokio::net::windows::named_pipe::ClientOptions;

#[cfg(unix)]
use tokio::net::UnixStream;

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

    // 2. Connect to the Engine directly via IPC (Cross-Platform)
    let engine_data = fetch_settings_from_ipc().await?;

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

async fn fetch_settings_from_ipc() -> Result<Value, String> {
    let request = serde_json::json!({ "action": "GET_SETTINGS" });
    let mut response_bytes = Vec::new();
    let mut buf = vec![0; 8192];
    let timeout_duration = std::time::Duration::from_secs(2);

    #[cfg(windows)]
    {
        let mut client = match ClientOptions::new().open(r"\\.\pipe\cluaiz_engine_pipe") {
            Ok(c) => c,
            Err(e) => return Err(format!("Failed to connect to Engine IPC on Windows: {}", e)),
        };
        
        if let Err(e) = tokio::time::timeout(timeout_duration, client.write_all(request.to_string().as_bytes())).await {
            return Err(format!("IPC Write Timeout: {}", e));
        }
        
        loop {
            match tokio::time::timeout(timeout_duration, client.read(&mut buf)).await {
                Ok(Ok(n)) if n == 0 => return Err("Engine closed pipe before sending full JSON".to_string()),
                Ok(Ok(n)) => {
                    response_bytes.extend_from_slice(&buf[..n]);
                    if let Ok(parsed) = serde_json::from_slice::<Value>(&response_bytes) {
                        return Ok(parsed);
                    }
                }
                Ok(Err(e)) => return Err(format!("Failed to read from pipe: {}", e)),
                Err(_) => return Err("IPC Read Timeout: Engine did not respond in 2 seconds".to_string()),
            }
        }
    }

    #[cfg(unix)]
    {
        let mut client = match UnixStream::connect("/tmp/cluaiz_engine.sock").await {
            Ok(c) => c,
            Err(e) => return Err(format!("Failed to connect to Engine Unix Socket: {}", e)),
        };
        
        if let Err(e) = tokio::time::timeout(timeout_duration, client.write_all(request.to_string().as_bytes())).await {
            return Err(format!("IPC Write Timeout: {}", e));
        }
        
        loop {
            match tokio::time::timeout(timeout_duration, client.read(&mut buf)).await {
                Ok(Ok(n)) if n == 0 => return Err("Engine closed socket before sending full JSON".to_string()),
                Ok(Ok(n)) => {
                    response_bytes.extend_from_slice(&buf[..n]);
                    if let Ok(parsed) = serde_json::from_slice::<Value>(&response_bytes) {
                        return Ok(parsed);
                    }
                }
                Ok(Err(e)) => return Err(format!("Failed to read from socket: {}", e)),
                Err(_) => return Err("IPC Read Timeout: Engine did not respond in 2 seconds".to_string()),
            }
        }
    }

    #[cfg(not(any(windows, unix)))]
    return Err("Unsupported OS architecture for IPC communication".to_string());
}
