use cluaiz_shared::hardware::governor::HardwareGovernor;
use engine_lmdb::env::LmdbEnv;
use engine_lmdb::ffi::{
    cluaizd_ffi_execute_parameterized, cluaizd_ffi_free_neuron, cluaizd_ffi_read_neuron,
    CluaizdFfiNeuron,
};
use sha2::Digest;
use std::ffi::{c_char, c_void, CStr, CString};
use std::slice;
use std::sync::OnceLock;
use serde::{Deserialize, Serialize};

static GLOBAL_LMDB_SHARDS: OnceLock<Vec<LmdbEnv>> = OnceLock::new();

#[derive(Deserialize)]
struct CelPayload {
    action: String,
    memory_id: Option<String>,
    payload: Option<String>,
    vector: Option<Vec<f32>>,
    shard_index: Option<usize>,
    query: Option<String>,
}

#[derive(Serialize)]
struct CelResponse {
    status: String,
    message: String,
    data: Option<String>,
}

fn internal_boot_environment() -> Result<String, String> {
    let base_dir = cluaiz_shared::environment::EnvironmentManager::current().global_dir.join("core").join("cluaiz-db");
    if !base_dir.exists() {
        let _ = std::fs::create_dir_all(&base_dir);
    }
    let base_path = base_dir;

    let mut sys = sysinfo::System::new();
    sys.refresh_memory();
    let total_memory_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);

    let num_shards = if total_memory_gb < 4.0 { 1 }
    else if total_memory_gb < 8.0 { 2 }
    else if total_memory_gb < 16.0 { 4 }
    else { ((total_memory_gb / 4.0) as usize).min(8) };

    let shard_capacity = if total_memory_gb < 4.0 { 64 * 1024 * 1024 }
    else if total_memory_gb < 8.0 { 128 * 1024 * 1024 }
    else { 256 * 1024 * 1024 };

    let mut shards = Vec::new();
    for i in 0..num_shards {
        let shard_path = base_path.join(format!("shard_{}", i));
        if !shard_path.exists() {
            let _ = std::fs::create_dir_all(&shard_path);
        }
        match LmdbEnv::open(&shard_path, shard_capacity) {
            Ok(env) => {
                shards.push(env);
                tracing::info!("🧠 LMDB Shard {} booted with {}MB capacity at {:?}", i, shard_capacity / (1024 * 1024), shard_path);
            }
            Err(e) => {
                tracing::error!("Failed to boot LMDB Shard {}: {:?}", i, e);
            }
        }
    }

    let actual_shards_len = shards.len();
    if actual_shards_len > 0 {
        let _ = GLOBAL_LMDB_SHARDS.set(shards);
        tracing::info!("🧠 All {} LMDB Shards fully booted via CEL Payload", actual_shards_len);
        Ok(format!("{} Shards booted successfully", actual_shards_len))
    } else {
        Err("Failed to boot any LMDB shards".to_string())
    }
}

fn internal_inject_context(memory_key_str: &str) -> Result<String, String> {
    if let Ok(control) = HardwareGovernor::load_system_control() {
        if !control.brain.is_enabled() {
            tracing::debug!("Cluaizd CEL Brain is disabled. Falling back.");
            return Err("Brain is disabled in system control".to_string());
        }
    } else {
        return Err("Failed to load system control".to_string());
    }

    let shards = match GLOBAL_LMDB_SHARDS.get() {
        Some(s) => s,
        None => return Err("Shards not initialized".to_string()),
    };

    let mut hasher = sha2::Sha256::new();
    hasher.update(memory_key_str.as_bytes());
    let hash_result = hasher.finalize();
    let mut id_array = [0u8; 16];
    id_array.copy_from_slice(&hash_result[..16]);

    let shard_idx = (id_array[0] as usize) % shards.len();
    let env = &shards[shard_idx];
    let env_ptr = env as *const LmdbEnv as *mut c_void;

    let mut out_neuron = CluaizdFfiNeuron {
        id: [0; 16],
        vector_ptr: std::ptr::null(),
        vector_len: 0,
        state_hash: [0; 32],
        payload_ptr: std::ptr::null(),
        payload_len: 0,
        handle: std::ptr::null_mut(),
    };

    let result = unsafe { cluaizd_ffi_read_neuron(env_ptr, id_array.as_ptr(), &mut out_neuron) };

    if result != 0 || out_neuron.payload_ptr.is_null() {
        tracing::debug!("CEL Brain lookup failed for key: {}", memory_key_str);
        return Err("Memory key not found".to_string());
    }

    let payload = unsafe {
        let slice = std::slice::from_raw_parts(out_neuron.payload_ptr, out_neuron.payload_len);
        let vec_data = slice.to_vec();
        cluaizd_ffi_free_neuron(out_neuron.handle);
        vec_data
    };

    tracing::info!("🧠 Successfully injected Neural Context via CEL Payload: {} bytes", payload.len());
    
    match String::from_utf8(payload) {
        Ok(s) => Ok(s),
        Err(_) => Err("Payload is not valid UTF-8".to_string())
    }
}

fn internal_save_context(memory_id_str: &str, payload_str: &str, vector: Option<&[f32]>) -> Result<String, String> {
    if let Ok(control) = HardwareGovernor::load_system_control() {
        if !control.brain.is_enabled() {
            tracing::debug!("Cluaizd CEL Brain is disabled. Skipping save.");
            return Ok("Brain disabled, skipped save".to_string());
        }
    } else {
        return Err("Failed to load system control".to_string());
    }

    let shards = match GLOBAL_LMDB_SHARDS.get() {
        Some(s) => s,
        None => return Err("Shards not initialized".to_string()),
    };

    let mut hasher = sha2::Sha256::new();
    hasher.update(memory_id_str.as_bytes());
    let hash_result = hasher.finalize();
    let mut id_array = [0u8; 16];
    id_array.copy_from_slice(&hash_result[..16]);

    let shard_idx = (id_array[0] as usize) % shards.len();
    let env = &shards[shard_idx];
    let env_ptr = env as *const LmdbEnv as *mut c_void;

    let query = format!("insert into Context(id: \"{}\", payload: \"{}\", vector: ?)\0", memory_id_str, payload_str);
    
    let (raw_vector_ptr, raw_vector_len) = if let Some(vec) = vector {
        (vec.as_ptr() as *const u8, vec.len() * 4)
    } else {
        (std::ptr::null(), 0)
    };

    let result = unsafe { cluaizd_ffi_execute_parameterized(env_ptr, query.as_ptr() as *const std::ffi::c_char, raw_vector_ptr, raw_vector_len) };

    if result != 0 {
        return Err(format!("FFI execute failed with code {}", result));
    }

    tracing::info!("🧠 Successfully saved contextual vector to Engine Brain ({}).", memory_id_str);
    Ok("Saved successfully".to_string())
}

#[repr(C)]
pub enum PayloadType {
    Json,
    Cdql,
    WasmBinary,
    RawBytes,
    Bincode,
}

#[repr(C)]
pub struct ExtensionPayload {
    pub payload_type: PayloadType,
    pub data_ptr: *const u8,
    pub data_len: usize,
}

#[no_mangle]
pub extern "C" fn execute_cel(payload_ptr: *const ExtensionPayload) -> *mut c_char {
    if payload_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let payload_ref = unsafe { &*payload_ptr };
    let incoming_bytes = unsafe {
        std::slice::from_raw_parts(payload_ref.data_ptr, payload_ref.data_len)
    };

    let json_str = match std::str::from_utf8(incoming_bytes) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let payload: CelPayload = match serde_json::from_str(json_str) {
        Ok(p) => p,
        Err(e) => {
            let res = CelResponse { status: "error".into(), message: format!("Failed to parse CEL payload: {}", e), data: None };
            return CString::new(serde_json::to_string(&res).unwrap()).unwrap().into_raw();
        }
    };

    let result = match payload.action.as_str() {
        "boot" => internal_boot_environment(),
        "inject" => {
            if let Some(key) = payload.memory_id {
                internal_inject_context(&key)
            } else {
                Err("Missing memory_id for inject".to_string())
            }
        },
        "save" => {
            if let (Some(id), Some(data)) = (payload.memory_id, payload.payload) {
                internal_save_context(&id, &data, payload.vector.as_deref())
            } else {
                Err("Missing memory_id or payload for save".to_string())
            }
        },
        _ => Err(format!("Unknown action: {}", payload.action)),
    };

    let response = match result {
        Ok(data) => CelResponse { status: "success".into(), message: "Execution successful".into(), data: Some(data) },
        Err(e) => CelResponse { status: "error".into(), message: e, data: None },
    };

    let response_str = serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string());
    CString::new(response_str).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn cluaiz_free_payload(ptr: *mut c_char, _len: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
