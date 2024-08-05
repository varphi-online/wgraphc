use serde_json;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn set_var(key: String, value: String, map: String) -> String {
    let mut out: HashMap<String, String> = serde_json::from_str(&map).unwrap();
    out.insert(key, value);
    serde_json::to_string(&out).unwrap()
}

#[wasm_bindgen]
pub fn del_var(key: String, map: String) -> String {
    let mut out: HashMap<String, String> = serde_json::from_str(&map).unwrap();
    out.remove(&key);
    serde_json::to_string(&out).unwrap()
}
