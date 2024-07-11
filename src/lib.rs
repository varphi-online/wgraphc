use fancy_regex::Regex;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

lazy_static! {
    static ref NUMERIC: Regex = Regex::new(r"\G([0-9]+(\.)?[0-9]*)$").unwrap();
    static ref ALPHABETIC: Regex = Regex::new(r"\G(([a-zA-Z]+)(_(\{(\w*(})?)?)?)?)$").unwrap();
    static ref OPERATIONAL: Regex = Regex::new(r"\G([\*/\-+\(\]\)\]<>^|])$").unwrap();
    static ref NUMERIC_CHAR: Regex = Regex::new(r"[\.\d]").unwrap();
    static ref ALPHABETIC_CHAR: Regex = Regex::new(r"[\w\{\}\_]").unwrap();
    static ref WHITESPACE: Regex = Regex::new(r"\s").unwrap();
}

#[wasm_bindgen]
pub fn return_string(input: String) -> Vec<String> {
    let mut lexemes: Vec<String> = Vec::new();
    lexemes.push(String::from(""));

    for c in input.chars() {
        let char = c.to_string();
        if WHITESPACE.is_match(&char).unwrap() {
            continue;
        };
        let character_type = char_type(char.clone());
        let combined_type: u8;
        if let Some(working_lexeme) = lexemes.last_mut() {
            console_log!("Current lexeme:{}", working_lexeme);
            let combination = format!("{}{}", working_lexeme, char);
            console_log!("Testing: {}+{} ({})", working_lexeme, char, combination);
            combined_type = string_type(combination);
            console_log!(
                "{} & {} = {} ({})",
                combined_type,
                character_type,
                combined_type & character_type,
                (combined_type & character_type) == combined_type
            );
            if (combined_type & character_type) == combined_type || working_lexeme == "" {
                console_log!("Adding to previous lexeme");
                working_lexeme.push(c);
            } else {
                console_log!("Pushing to new lexeme");
                lexemes.push(char);
            }
        }
    }
    let out: Vec<String> = Vec::from(lexemes);
    out
}

fn string_type(string: String) -> u8 {
    let mut out: u8 = 0;
    if NUMERIC.captures(&string).unwrap().is_some() {
        out += 4;
    }
    if ALPHABETIC.captures(&string).unwrap().is_some() {
        out += 2;
    }
    if OPERATIONAL.captures(&string).unwrap().is_some() {
        out += 1;
    }
    if out != 0 {
        out
    } else {
        8
    }
}

fn char_type(string: String) -> u8 {
    let mut out: u8 = 0;
    if NUMERIC_CHAR.captures(&string).unwrap().is_some() {
        out += 4;
    }
    if ALPHABETIC_CHAR.captures(&string).unwrap().is_some() {
        out += 2;
    }
    if OPERATIONAL.captures(&string).unwrap().is_some() {
        out += 1;
    }
    out
}
