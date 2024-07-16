use fancy_regex::Regex;
use lazy_static::lazy_static;
use num_complex::*;
//use once_cell::sync::lazy;

use super::token::{self, OpVec, Operator, Token, Value};

lazy_static! {
    static ref NUMERIC: Regex = Regex::new(r"\G((\d)+(\.)?(\d)*[i]?)$").unwrap();
    static ref ALPHABETIC: Regex = Regex::new(r"\G(([a-zA-Z]+)(_(\{(\w*(})?)?)?)?)$").unwrap();
    static ref OPERATIONAL: Regex = Regex::new(r"\G([\*/\-+\(\]\)\]<>^|])$").unwrap();
    static ref NUMERIC_CHAR: Regex = Regex::new(r"[i\.\d]").unwrap();
    static ref ALPHABETIC_CHAR: Regex = Regex::new(r"[\w\{\}\_]").unwrap();
    static ref WHITESPACE: Regex = Regex::new(r"\s").unwrap();
}

pub fn get_tester() -> Operator {
    let mut d = Operator::from_token(Token::Sqrt);
    let mut a = Operator::from_token(Token::Add);
    let mut b = Operator::from_token(Token::Num);
    b.values = Value::Number(c64(5, 0));
    let mut c = Operator::from_token(Token::Num);
    c.values = Value::Number(c64(23, 0));
    a.values.set_index(0, b);
    a.values.set_index(1, c);
    d.values.set_index(0, a);
    d
}
pub fn evaluate(lexemes: Vec<String>) -> OpVec {
    let mut tokens: OpVec = OpVec::new();
    for lexeme in lexemes {
        if ALPHABETIC.captures(&lexeme).unwrap().is_some() {
            let mut to_add = Operator::from_token(Token::ID);
            to_add.symbol.clone_from(&lexeme);
            tokens.push(to_add);
        } else if NUMERIC.captures(&lexeme).unwrap().is_some() {
            let mut to_add = Operator::from_token(Token::Num);
            to_add.symbol.clone_from(&lexeme);
            if lexeme.ends_with('i') {
                to_add.values = Value::Imag(lexeme[0..lexeme.len() - 1].parse::<f64>().unwrap());
            } else {
                to_add.values = Value::Real(lexeme.parse::<f64>().unwrap());
            }
            tokens.push(to_add);
        } else if token::OPERATORS.contains_key(&lexeme) {
            let to_add = Operator::from_token(token::OPERATORS.get(&lexeme).unwrap().clone());
            tokens.push(to_add);
        }
    }
    tokens
}

pub fn analyze(input: OpVec) -> OpVec {
    let mut intermediate: OpVec = OpVec::new();
    let mut output: OpVec = OpVec::new();

    // Collapses seperate real and imaginary components into one complex number
    let mut stored_num: Option<Operator> = None;
    for mut token in input {
        if token.token_type == Token::Num {
            if let Some(tok) = &stored_num {
                match (&token.values, &tok.values) {
                    (Value::Real(_), Value::Imag(_)) => {
                        token.values = Value::Number(Complex64::new(
                            token.values.get_num().unwrap(),
                            tok.values.get_num().unwrap(),
                        ))
                    }
                    (Value::Imag(_), Value::Real(_)) => {
                        token.values = Value::Number(Complex64::new(
                            tok.values.get_num().unwrap(),
                            token.values.get_num().unwrap(),
                        ))
                    }
                    _ => (),
                }
            } else {
                stored_num = Some(token.clone());
            }
        }
        output.push(token);
    }
    output
}
