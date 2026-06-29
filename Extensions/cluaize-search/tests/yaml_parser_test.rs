use serde_json::Value;
use std::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::windows::named_pipe::ClientOptions;

#[tokio::test]
async fn test_yaml_system_bindings() {
    println!("=====================================");
    println!("🔥 EXTRACTING VALUES VIA ENGINE IPC 🔥");
    println!("=====================================");

    // 1. Read manifest to get the required system_bindings
    let yaml_str = fs::read_to_string("manifest-extension.yaml").expect("Failed to read manifest");
    let manifest: Value = serde_yaml::from_str(&yaml_str).expect("Failed to parse YAML");
    
    let bindings = manifest["system_bindings"]
        .as_array()
        .expect("Missing system_bindings in manifest")
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect::<Vec<String>>();

    // 2. Connect to the Engine directly via IPC (No file paths, no mocks!)
    let mut client = ClientOptions::new()
        .open(r"\\.\pipe\cluaize_engine_pipe")
        .expect("Failed to connect to Cluaize Engine Named Pipe. Make sure 'cargo run serve' is running!");

    // 3. Send the request to the engine
    let request = serde_json::json!({ "action": "GET_SETTINGS" });
    client.write_all(request.to_string().as_bytes()).await.expect("Failed to send request to Engine");

    // 4. Read the Engine's response (handles partial reads)
    let mut response_bytes = Vec::new();
    let mut buf = vec![0; 8192];
    let engine_data: Value = loop {
        let n = client.read(&mut buf).await.expect("Failed to read response from Engine");
        if n == 0 {
            panic!("Engine closed pipe before sending full JSON");
        }
        response_bytes.extend_from_slice(&buf[..n]);
        // Try parsing the accumulated bytes. Once valid, we break.
        if let Ok(parsed) = serde_json::from_slice::<Value>(&response_bytes) {
            break parsed;
        }
    };
    let booster_settings = &engine_data["booster"];

    // 5. Extract exactly what manifest demands from the Engine's response
    for binding in &bindings {
        let parts: Vec<&str> = binding.split('.').collect();
        // Since the engine returns the booster settings object, we skip the first part ("system_booster")
        let mut current_val: Option<&Value> = Some(booster_settings);
        
        for part in parts.iter().skip(1) {
            if let Some(val) = current_val {
                current_val = val.get(*part);
            }
        }
        
        if let Some(extracted) = current_val {
            println!("CEL Path: {} => Real Extracted Value: {:?}", binding, extracted);
        } else {
            println!("CEL Path: {} => Not Found in engine payload", binding);
        }
    }
}
