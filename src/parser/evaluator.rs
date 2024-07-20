use crate::util::*;
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
    c.values = Value::Number(c64(0, -23));
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

pub fn analyze(inp: OpVec) -> OpVec {
    let length: usize = inp.len();
    let mut intermediate: OpVec = OpVec::new();

    let mut skip_flag: bool = false;
    let mut num_skip_flag: bool = false;

    for i in 0..length {
        let token = inp.clone().get(i).unwrap();
        if skip_flag {
            skip_flag = false;
            continue;
        }
        match token.token_type {
            Token::Sub => {
                if let Some(next_token) = inp.clone().get(i + 1) {
                    if next_token.token_type == Token::Num {
                        intermediate.push(create_complex(next_token, -1.0));
                        num_skip_flag = true;
                        continue;
                    }
                }
            }
            // While iterating through a raw list of tokens handle number ones
            Token::Num => {
                if num_skip_flag {
                    num_skip_flag = false;
                    continue;
                }

                if let Some(next_token) = inp.clone().get(i + 1) {
                    match next_token.token_type {
                        Token::Add => {
                            if let Some(second_num) = inp.clone().get(i + 2) {
                                if token.values.get_type() != second_num.values.get_type() {
                                    intermediate
                                        .push(create_complex_from_two(token, second_num, 1.0));
                                    num_skip_flag = true;
                                    skip_flag = true;
                                    continue;
                                }
                            }
                        }
                        Token::Sub => {
                            if let Some(second_num) = inp.clone().get(i + 2) {
                                if token.values.get_type() != second_num.values.get_type() {
                                    intermediate
                                        .push(create_complex_from_two(token, second_num, -1.0));
                                    num_skip_flag = true;
                                    skip_flag = true;
                                    continue;
                                }
                            }
                        }
                        _ => (),
                    }
                }
                intermediate.push(create_complex(token, 1.0));
                continue;
            }
            _ => (),
        }
        intermediate.push(token);
    }
    intermediate
}

fn create_complex_from_two(input_a: Operator, input_b: Operator, mult: f64) -> Operator {
    match (input_a.values.get_type(), input_b.values.get_type()) {
        (1, 2) => {
            let mut out: Operator = Operator::from_token(Token::Num);
            out.values = Value::Number(Complex64::new(
                input_a.values.get_num().unwrap(),
                mult * input_b.values.get_num().unwrap(),
            ));
            out.symbol = format!("{}", out.values);
            out
        }
        (2, 1) => {
            let mut out: Operator = Operator::from_token(Token::Num);
            out.values = Value::Number(Complex64::new(
                input_b.values.get_num().unwrap(),
                mult * input_a.values.get_num().unwrap(),
            ));
            out.symbol = format!("{}", out.values);
            out
        }
        _ => panic!("This should never happen"),
    }
}

fn create_complex(input_a: Operator, mult: f64) -> Operator {
    match input_a.values.get_type() {
        1 => {
            let mut out: Operator = Operator::from_token(Token::Num);
            out.values = Value::Number(Complex64::new(
                mult * input_a.values.get_num().unwrap(),
                0.0,
            ));
            out.symbol = format!("{}", out.values);
            out
        }
        2 => {
            let mut out: Operator = Operator::from_token(Token::Num);
            out.values = Value::Number(Complex64::new(
                0.0,
                mult * input_a.values.get_num().unwrap(),
            ));
            out.symbol = format!("{}", out.values);
            out
        }
        3 => input_a,
        _ => panic!("This should never happen"),
    }
}
