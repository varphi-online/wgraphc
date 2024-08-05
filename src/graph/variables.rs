use serde_json;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use fancy_regex::Regex;

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

#[wasm_bindgen]
pub fn input_type(input: String)-> String {
    let function_id: Regex = Regex::new(r"((([a-zA-Z]+)(_(\{(\w*(})?)?)?)?)\(.*\))=").unwrap();
    let variable_id: Regex = Regex::new(r"(([a-zA-Z]+)(_(\{(\w*(})?)?)?)?)=").unwrap();
    let function_argument: Regex = Regex::new(r"\((([a-zA-Z]+)(_(\{(\w*(})?)?)?)?)(,(([a-zA-Z]+)(_(\{(\w*(})?)?)?)?))*\)").unwrap();
    if input.contains("=") {
        if function_id.captures(&input).is_ok(){
            if function_argument.captures(&input).is_ok(){
                return "function".to_string();
            } 
        } else if variable_id.captures(&input).is_ok(){
           return "variable".to_string()
        }
        "malformed".to_string()
    } else {
        "expression".to_string()
    }
}

fn parse_variable(input: String) -> String {
    let variable_id: Regex = Regex::new(r"(([a-zA-Z]+)(_(\{(\w*(})?)?)?)?)=").unwrap();
    let clean_input: String = variable_id.replace(&input, "").to_string();
    "lol".to_string()
}