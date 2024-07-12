use crate::util::*;
use fancy_regex::Regex;
use lazy_static::lazy_static;
//use once_cell::sync::lazy;

use super::token;

lazy_static! {
    static ref NUMERIC: Regex = Regex::new(r"\G([0-9]+(\.)?[0-9]*)$").unwrap();
    static ref ALPHABETIC: Regex = Regex::new(r"\G(([a-zA-Z]+)(_(\{(\w*(})?)?)?)?)$").unwrap();
    static ref OPERATIONAL: Regex = Regex::new(r"\G([\*/\-+\(\]\)\]<>^|])$").unwrap();
    static ref NUMERIC_CHAR: Regex = Regex::new(r"[\.\d]").unwrap();
    static ref ALPHABETIC_CHAR: Regex = Regex::new(r"[\w\{\}\_]").unwrap();
    static ref WHITESPACE: Regex = Regex::new(r"\s").unwrap();
}

//pub fn evaluate(lexemes: Vec<String>) -> Vec<token::Operator> {}
