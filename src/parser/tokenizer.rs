use crate::util::*;
use fancy_regex::Regex;
use lazy_static::lazy_static;
use num_complex::*;
//use once_cell::sync::lazy;

use super::structs::{op_vec::OpVec, operator::Operator, token::{Token,OPERATORS}, value::Value};

lazy_static! {
    static ref NUMERIC: Regex = Regex::new(r"\G((\d)+(\.)?(\d)*[i]?)$").unwrap();
    static ref ALPHABETIC: Regex = Regex::new(r"\G(([a-zA-Z]+)(_(\{(\w*(})?)?)?)?)$").unwrap();
    static ref OPERATIONAL: Regex = Regex::new(r"\G([\*/\-+\(\]\)\]<>^|])$").unwrap();
    static ref NUMERIC_CHAR: Regex = Regex::new(r"[i\.\d]").unwrap();
    static ref ALPHABETIC_CHAR: Regex = Regex::new(r"[\w\{\}\_]").unwrap();
    static ref WHITESPACE: Regex = Regex::new(r"\s").unwrap();
}

pub fn tokenize(lexemes: Vec<String>) -> OpVec {
    let mut out: OpVec = to_tokens(lexemes);
    out = apply_partial_grammar(out);
    out
}

fn to_tokens(lexemes: Vec<String>) -> OpVec {
    let mut tokens: OpVec = OpVec::new();
    for lexeme in lexemes {
        if ALPHABETIC.is_match(&lexeme).unwrap() {
            if OPERATORS.contains_key(&lexeme) {
                clog!("Adding something special");
                let to_add = Operator::from_token(OPERATORS.get(&lexeme).unwrap().clone());
                tokens.push(to_add);
            } else {
                let mut to_add = Operator::from_token(Token::ID);
                to_add.symbol.clone_from(&lexeme);
                tokens.push(to_add);
            }
        } else if NUMERIC.is_match(&lexeme).unwrap(){
            let mut to_add = Operator::from_token(Token::Num);
            to_add.symbol.clone_from(&lexeme);
            if lexeme.ends_with('i') {
                to_add.values = Value::Imag(lexeme[0..lexeme.len() - 1].parse::<f64>().unwrap());
            } else {
                to_add.values = Value::Real(lexeme.parse::<f64>().unwrap());
            }
            tokens.push(to_add);
        } else if OPERATORS.contains_key(&lexeme) {
            let to_add = Operator::from_token(OPERATORS.get(&lexeme).unwrap().clone());
            tokens.push(to_add);
        }
    }
    clog!("Made tokens");
    tokens
}

fn apply_partial_grammar(inp: OpVec) -> OpVec {
    // Loop through a grouping of token vectors to apply basic
    // grammars we do not want to worry about when building our AST
    // Should handle most if not all edge cases
    let length: usize = inp.len();
    let mut intermediate: OpVec = OpVec::new();

    let mut skip_flag: bool = false;

    // Apply unary -
    for i in 0..length {
        let token: Operator = inp.clone().get(i).unwrap();

        if skip_flag {
            skip_flag = false;
            continue;
        }

        if token.token_type == Token::Sub {
            let previous_token: Option<Operator> = if ((i as i32) - 1) < 0 {
                None
            } else {
                inp.clone().get(i - 1)
            };
            match (previous_token, inp.clone().get(i + 1)) {
                (Some(prev), Some(next)) => match (&prev.token_type, &next.token_type) {
                    // These cases we handle like a normal - sign
                    (Token::ID, Token::Num)
                    | (Token::Num, Token::ID)
                    | (Token::ID, Token::ID)
                    | (Token::Num, Token::OpenPar)
                    | (Token::ID, Token::OpenPar) => (),
                    // Two non-same dimension numbers with - between
                    (Token::Num, Token::Num) => {
                        if prev.values.get_type() != next.values.get_type() {
                            intermediate.pop();
                            intermediate.push(create_complex_from_two(prev, next, -1.0));
                            skip_flag = true;
                            continue;
                        }
                    }
                    // A non-terminal and number seperated by a -
                    (_, Token::Num) => {
                        intermediate.push(negate(next));
                        skip_flag = true;
                        continue;
                    }
                    //HACK: Check this logic concerning ids and non terminals
                    // A non-terminal and a var or open parentheses seperated by a -
                    /*(_, Token::ID) | */ (_, Token::OpenPar) => {
                        let mut temp: Operator = Operator::from_token(Token::Num);
                        temp.values = Value::Real(-1.0);
                        intermediate.push(temp);
                        intermediate.push(Operator::from_token(Token::Mult));
                        intermediate.push(next);
                        skip_flag = true;
                        continue;
                    }
                    _ => (),
                },
                // This case should only ever be reached in the event that a -
                // is the first token in the input stream, and will only handle
                // the three cases where it should be converted to something else
                (None, Some(next)) => match next.token_type {
                    Token::Num => {
                        intermediate.push(negate(next));
                        skip_flag = true;
                        continue;
                    }
                    Token::ID | Token::OpenPar => {
                        let mut temp: Operator = Operator::from_token(Token::Num);
                        temp.values = Value::Real(-1.0);
                        intermediate.push(temp);
                        intermediate.push(Operator::from_token(Token::Mult));
                        intermediate.push(next);
                        skip_flag = true;
                        continue;
                    }
                    _ => (),
                },
                _ => (),
            }
        }
        intermediate.push(token);
    }

    let mut intermediate_2: OpVec = OpVec::new();

    let mut skip_num: bool = false;

    // Iterate and collapse any unresolved complex numbers
    // in the token stack, which should only be ones directly
    // adjacent to one another or seperated by a "+" symbol
    for i in 0..intermediate.len() {
        let token: Operator = intermediate.clone().get(i).unwrap();

        if skip_flag {
            skip_flag = false;
            continue;
        }

        if token.token_type == Token::Num && token.values.get_type() != 3 {
            if skip_num {
                skip_num = false;
                continue;
            }
            if let Some(next_token) = intermediate.clone().get(i + 1) {
                match next_token.token_type {
                    Token::Add => {
                        if let Some(second_num) = intermediate.clone().get(i + 2) {
                            if second_num.token_type == Token::Num
                                && token.values.get_type() != second_num.values.get_type()
                            {
                                intermediate_2
                                    .push(create_complex_from_two(token, second_num, 1.0));
                                skip_flag = true;
                                skip_num = true;
                                continue;
                            }
                        }
                    }
                    Token::Num => {
                        if token.values.get_type() != next_token.values.get_type() {
                            intermediate_2.push(create_complex_from_two(token, next_token, 1.0));
                            skip_flag = true;
                            continue;
                        }
                    }

                    _ => (),
                }
            }
            intermediate_2.push(create_complex(token, 1.0));
            continue;
        }
        intermediate_2.push(token);
    }

    let mut output: OpVec = OpVec::new();
    // Handle implicit multiplication
    for i in 0..intermediate_2.len() {
        let token: Operator = intermediate_2.clone().get(i).unwrap();

        if let Some(next) = intermediate_2.clone().get(i + 1) {
            match (&token.token_type, next.token_type) {
                (Token::Num, Token::ID)
                | (Token::Num, Token::OpenPar)
                | (Token::ID, Token::OpenPar) => {
                    output.push(token);
                    output.push(Operator::from_token(Token::Mult));
                    continue;
                }
                _ => (),
            }
        }
        output.push(token);
    }
    output.push(Operator::from_token(Token::END));
    output
}

fn negate(input: Operator) -> Operator {
    let mut out: Operator = Operator::from_token(Token::Num);
    match input.values.get_type() {
        1 => out.values = Value::Real(-1.0 * input.values.get_num().unwrap()),
        2 => out.values = Value::Imag(-1.0 * input.values.get_num().unwrap()),
        _ => {
            clog!("Unaccounted token: {}", input);
            panic!("This should never happen")
        }
    }
    out
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
        _ => {
            clog!("Unaccounted tokens: {} and {}", input_a, input_b);
            panic!("This should never happen")
        }
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
        _ => {
            clog!("Unaccounted token: {}", input_a);
            panic!("This should never happen")
        }
    }
}
