use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn return_string() -> String {
    "hello".into()
}
