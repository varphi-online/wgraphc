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
    for i in 0..length {
        if skip_flag {
            skip_flag = false;
            continue;
        }
        let token = inp.clone().get(i).unwrap();
        if token.token_type == Token::Sub {
            #[warn(clippy::collapsible_if)]
            if i > 0 {
                if let (Some(a), Some(b)) = (inp.clone().get(i - 1), inp.clone().get(i + 1)) {
                    // Handles the case of a - sign between two parts of a complex number
                    if a.token_type == Token::Num && b.token_type == Token::Num {
                        match (&a.values, &b.values) {
                            (Value::Real(_), Value::Imag(_)) => {
                                let mut temporary = b.clone();
                                temporary.values =
                                    Value::Imag(-temporary.values.get_num().unwrap());
                                intermediate.push(temporary);
                                skip_flag = true;
                                continue;
                            }
                            (Value::Imag(_), Value::Real(_)) => {
                                let mut temporary = b.clone();
                                temporary.values =
                                    Value::Real(-temporary.values.get_num().unwrap());
                                intermediate.push(temporary);
                                skip_flag = true;
                                continue;
                            }
                            _ => (),
                        }
                    }
                    // Case of a - sign between a non-num and number
                    else if a.token_type != Token::Num && b.token_type == Token::Num {
                        match b.values {
                            Value::Real(_) => {
                                let mut temporary = b.clone();
                                temporary.values =
                                    Value::Real(-temporary.values.get_num().unwrap());
                                intermediate.push(temporary);
                                skip_flag = true;
                                continue;
                            }
                            Value::Imag(_) => {
                                let mut temporary = b.clone();
                                temporary.values =
                                    Value::Imag(-temporary.values.get_num().unwrap());
                                intermediate.push(temporary);
                                skip_flag = true;
                                continue;
                            }
                            _ => (),
                        }
                    }
                }
            } else if let Some(b) = inp.clone().get(i + 1) {
                if b.token_type == Token::Num {
                    match b.values {
                        Value::Real(_) => {
                            let mut temporary = b.clone();
                            temporary.values = Value::Real(-temporary.values.get_num().unwrap());
                            intermediate.push(temporary);
                            skip_flag = true;
                            continue;
                        }
                        Value::Imag(_) => {
                            let mut temporary = b.clone();
                            temporary.values = Value::Imag(-temporary.values.get_num().unwrap());
                            intermediate.push(temporary);
                            skip_flag = true;
                            continue;
                        }
                        _ => (),
                    }
                }
            }
        } else if token.token_type == Token::Add {
            if let (Some(a), Some(b)) = (inp.clone().get(i - 1), inp.clone().get(i + 1)) {
                // Handles the case of a - sign between two parts of a complex number
                if a.token_type == Token::Num && b.token_type == Token::Num {
                    match (&a.values, &b.values) {
                        (Value::Real(_), Value::Imag(_)) | (Value::Imag(_), Value::Real(_)) => {
                            continue;
                        }
                        _ => (),
                    }
                }
            }
        }

        intermediate.push(token);
    }

    // Collapses seperate real and imaginary components into one complex number
    let mut output: OpVec = OpVec::new();

    // Any number passed while traversing the stack that is without a complex part
    // will be stored in this variable
    let mut stored_num: (usize, Option<Operator>) = (0, None);
    skip_flag = false;

    for i in 0..intermediate.len() {
        if skip_flag {
            continue;
        }
        if let Some(token) = intermediate.get_mut(i) {
            if token.token_type == Token::Num {
                if let Some(mut tok) = stored_num.clone().1 {
                    match (&tok.values, &token.values) {
                        (Value::Real(_), Value::Imag(_)) => {
                            token.values = Value::Number(Complex64::new(
                                tok.values.get_num().unwrap(),
                                token.values.get_num().unwrap(),
                            ));
                            stored_num = (0, None);
                            output.push(token.clone());
                        }
                        (Value::Imag(_), Value::Real(_)) => {
                            token.values = Value::Number(Complex64::new(
                                token.values.get_num().unwrap(),
                                tok.values.get_num().unwrap(),
                            ));
                            stored_num = (0, None);
                            output.push(token.clone());
                        }
                        (Value::Imag(_), Value::Imag(_)) => {
                            tok.values =
                                Value::Number(Complex64::new(0.0, tok.values.get_num().unwrap()));
                            output.insert(stored_num.0, tok);
                            stored_num = (i, Some(token.clone()));
                        }
                        (Value::Real(_), Value::Real(_)) => {
                            tok.values =
                                Value::Number(Complex64::new(tok.values.get_num().unwrap(), 0.0));
                            output.insert(stored_num.0, tok);
                            stored_num = (i, Some(token.clone()));
                        }
                        _ => (),
                    }
                } else {
                    stored_num = (i, Some(token.clone()));
                }
            } else {
                output.push(token.clone());
            }
        }
    }
    if let Some(mut item) = stored_num.1 {
        match item.values {
            Value::Imag(_) => {
                item.values = Value::Number(Complex64::new(0.0, item.values.get_num().unwrap()))
            }
            Value::Real(_) => {
                item.values = Value::Number(Complex64::new(item.values.get_num().unwrap(), 0.0))
            }
            _ => (),
        }
        clog!("Inserting {} to output at {}", item, stored_num.0);
        output.insert(stored_num.0, item);
    }
    output
}
