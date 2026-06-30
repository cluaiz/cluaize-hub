use std::ffi::{c_char, CStr, CString};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CelPayload {
    action: String,
    values: Option<Vec<f64>>,
}

#[derive(Serialize)]
struct CelResponse {
    status: String,
    message: String,
    data: Option<f64>,
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

    let values = match payload.values {
        Some(v) => v,
        None => {
            let res = CelResponse { status: "error".into(), message: "Missing values array".into(), data: None };
            return CString::new(serde_json::to_string(&res).unwrap()).unwrap().into_raw();
        }
    };

    if values.is_empty() {
        let res = CelResponse { status: "error".into(), message: "Values array is empty".into(), data: None };
        return CString::new(serde_json::to_string(&res).unwrap()).unwrap().into_raw();
    }

    let result = match payload.action.as_str() {
        "mean" => {
            let sum: f64 = values.iter().sum();
            Ok(sum / values.len() as f64)
        }
        "median" => {
            let mut sorted = values.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let mid = sorted.len() / 2;
            if sorted.len() % 2 == 0 {
                Ok((sorted[mid - 1] + sorted[mid]) / 2.0)
            } else {
                Ok(sorted[mid])
            }
        }
        "std_dev" => {
            let sum: f64 = values.iter().sum();
            let mean = sum / values.len() as f64;
            let variance: f64 = values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
            Ok(variance.sqrt())
        }
        _ => Err(format!("Unknown action: {}", payload.action)),
    };

    let response = match result {
        Ok(val) => CelResponse { status: "success".into(), message: "Execution successful".into(), data: Some(val) },
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
