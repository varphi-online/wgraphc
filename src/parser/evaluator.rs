use fancy_regex::Regex;
use lazy_static::lazy_static;
//use once_cell::sync::lazy;

use super::token::{self, make_token, Token, Value};

lazy_static! {
    static ref NUMERIC: Regex = Regex::new(r"\G([0-9]+(\.)?[0-9]*)$").unwrap();
    static ref ALPHABETIC: Regex = Regex::new(r"\G(([a-zA-Z]+)(_(\{(\w*(})?)?)?)?)$").unwrap();
    static ref OPERATIONAL: Regex = Regex::new(r"\G([\*/\-+\(\]\)\]<>^|])$").unwrap();
    static ref NUMERIC_CHAR: Regex = Regex::new(r"[\.\d]").unwrap();
    static ref ALPHABETIC_CHAR: Regex = Regex::new(r"[\w\{\}\_]").unwrap();
    static ref WHITESPACE: Regex = Regex::new(r"\s").unwrap();
}

pub fn evaluate(lexemes: Vec<String>) -> Vec<token::Operator> {
    let mut tokens: Vec<token::Operator> = Vec::new();
    for lexeme in lexemes {
        if ALPHABETIC.captures(&lexeme).unwrap().is_some() {
            let mut to_add = make_token(Token::ID);
            to_add.symbol.clone_from(&lexeme);
            tokens.push(to_add);
        } else if NUMERIC.captures(&lexeme).unwrap().is_some() {
            let mut to_add = make_token(Token::Num);
            to_add.symbol.clone_from(&lexeme);
            to_add.values = Value::Number(lexeme.parse::<f64>().unwrap());
            tokens.push(to_add);
        } else if token::OPERATORS.contains_key(&lexeme) {
            let to_add = make_token(token::OPERATORS.get(&lexeme).unwrap().clone());
            tokens.push(to_add);
        }
    }
    tokens
}
