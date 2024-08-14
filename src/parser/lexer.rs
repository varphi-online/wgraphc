use super::expressions::*;
use crate::util::*;
//use once_cell::sync::lazy;

macro_rules! clog {
    ($($t:tt)*) => {};
}

pub fn scan(input: String) -> Vec<String> {
    let mut lexemes: Vec<String> = Vec::new();
    lexemes.push(String::from(""));

    if input.len() <= 0{
        // fail on empty input
        lexemes.push("+".to_string());
        lexemes.push("-".to_string());
        return lexemes;
    }

    for c in input.chars() {
        if c == ' '{
            continue;
        };
        let char = c.to_string();
        let character_type = char_type(char.clone().as_str());
        let combined_type: u8;
        if let Some(working_lexeme) = lexemes.last_mut() {
            clog!("Current lexeme:{}", working_lexeme);
            let combination = format!("{}{}", working_lexeme, char);
            clog!("Testing: {}+{} ({})", working_lexeme, char, combination);
            combined_type = string_type(combination.as_str());
            clog!(
                "{} & {} = {} ({})",
                combined_type,
                character_type,
                combined_type & character_type,
                (combined_type & character_type) == combined_type
            );
            if (combined_type & character_type) == combined_type || working_lexeme == "" {
                clog!("Adding to previous lexeme");
                working_lexeme.push(c);
            } else {
                clog!("Pushing to new lexeme");
                lexemes.push(char);
            }
        }
    }
    let out: Vec<String> = lexemes;
    out
}

fn string_type(string: &str) -> u8 {
    let mut out: u8 = 0;
    if is_number(&string) {
        out += 4;
    }
    if is_variable_id(&string) {
        out += 2;
    }
    if is_operational(&string) {
        out += 1;
    }
    if out != 0 {
        out
    } else {
        8
    }
}

fn char_type(string: &str) -> u8 {
    let mut out: u8 = 0;
    if is_numeric(&string) {
        out += 4;
    }
    if is_alphabetic(&string) {
        out += 2;
    }
    if is_operational(&string) {
        out += 1;
    }
    out
}
