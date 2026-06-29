use serde_json::Value;

#[tokio::test]
async fn test_cel_execution_via_engine() {
    println!("=====================================");
    println!("🔥 EXECUTING CEL SCRIPT VIA ENGINE 🔥");
    println!("=====================================");

    // We write a CEL script that actually INVOKES the extension.
    // 1. The CEL Parser reads this string.
    // 2. The Engine packages everything (including system_booster) into a Bincode struct.
    // 3. The Engine passes the C-Pointer to cluaize-search DLL.
    // 4. The search DLL executes and returns a String.
    let cel_script = "use plugin::cluaize-search -> invoke(query, target: \"Rust test\", search_api_key: \"dummy_key\");";
    
    println!("CEL Script: {}", cel_script);

    let request = serde_json::json!({ "script": cel_script });
    
    // Send it to the Engine's CEL execution endpoint
    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:8000/v1/cel/execute")
        .json(&request)
        .send()
        .await
        .expect("Failed to connect to Engine API on port 8000. Make sure 'cargo run serve' is running!");

    let status = res.status();
    let response_body = res.text().await.unwrap();
    
    println!("HTTP Status: {}", status);
    println!("Engine Response: {}", response_body);
    
    // Check if the CEL language successfully retrieved the value
    if let Ok(json_res) = serde_json::from_str::<Value>(&response_body) {
        if json_res["success"].as_bool().unwrap_or(false) {
            println!("CEL execution successful: {:?}", json_res["result"]);
        } else {
            println!("CEL execution failed: {:?}", json_res["error"]);
        }
    }
}
