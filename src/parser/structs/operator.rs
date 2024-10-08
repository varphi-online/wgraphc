use super::{arity::Arities, op_vec::OpVec, token::Token, value::Value};
use crate::util::clog;
use num_complex::{c64, Complex64};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, f64::consts::{E, PI}};
use lazy_static::lazy_static;
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct Operator {
    pub arity: Arities,
    pub token_type: Token,
    pub symbol: String,
    pub values: Value,
    pub precedence: u8,
    katex_repr: String,
}

impl Default for Operator {
    fn default() -> Self {
        Operator {
            token_type: Token::Null,
            arity: Arities::NULLARY,
            symbol: "".to_string(),
            values: Value::Op(OpVec(Vec::new())),
            precedence: 255,
            katex_repr: "".to_string(),
        }
    }
}

impl Operator {
    fn new() -> Operator {
        Operator {
            ..Default::default()
        }
    }

    pub fn from_value(val_to_add: Value) -> Operator {
        let mut out: Operator = Operator::from_token(Token::Num);
        out.values = val_to_add;
        out
    }

    pub fn from_token(type_of_token: Token) -> Operator {
        let arity: Arities = match type_of_token {
            Token::END | Token::SENTINEL | Token::Null | Token::OpenPar | Token::ClosePar => {
                Arities::NULLARY
            }
            Token::Num
            | Token::ID
            | Token::Sqrt
            | Token::Sin
            | Token::Cos
            | Token::Tan
            | Token::Log
            | Token::Ln => Arities::UNARY,
            Token::Add | Token::Sub | Token::Mult | Token::Div | Token::Pow => Arities::BINARY,
        };
        let arity_copy = arity.clone();
        Operator {
            arity,
            token_type: type_of_token.clone(),
            precedence: match &type_of_token {
                Token::SENTINEL | Token::OpenPar | Token::ClosePar => 0,
                Token::Pow | Token::Sqrt | Token::Log | Token::Ln => 1,
                Token::Sin | Token::Cos | Token::Tan => 2,
                Token::Mult | Token::Div => 3,
                Token::Add | Token::Sub => 4,
                _ => Operator::default().precedence,
            },
            values: match type_of_token {
                Token::Num => Value::Number(c64(0, 0)),
                _ => match arity_copy {
                    Arities::BINARY => Value::Op(OpVec(vec![Operator::new(), Operator::new()])),
                    Arities::UNARY => Value::Op(OpVec(vec![Operator::new()])),
                    _ => Operator::default().values,
                },
            },
            symbol: (match type_of_token {
                Token::Add => "+",
                Token::Sub => "-",
                Token::Mult => "*",
                Token::Div => "/",
                Token::Sqrt => "sqrt",
                Token::Pow => "^",
                Token::Sin => "sin",
                Token::Cos => "cos",
                Token::Tan => "tan",
                Token::Log => "log",
                Token::Ln => "ln",
                Token::OpenPar => "(",
                Token::ClosePar => ")",
                Token::END => "END",
                _ => "",
            })
            .to_string(),
            ..Default::default()
        }
    }

    fn idx(&self, i: usize) -> Operator {
        self.values
            .get_index(i)
            .expect("Improperly constructed input")
            .clone()
    }

    pub fn eval(&self, val: Complex64) -> Complex64 {
        let error: &str = "Improperly constructed input";
        match self.token_type {
            Token::Num => self.values.get_complex().expect(error),
            Token::ID => {
                //let varmap = serde_json::from_str::<HashMap<String, String>>(&map).unwrap();
                //if let Some(mapped_var) = varmap.get(&self.symbol) {
                //    Complex64::from_str(mapped_var).unwrap()
                //} else {
                val
                //}
            }
            Token::Add => self.idx(0).eval(val) + self.idx(1).eval(val),
            Token::Sub => self.idx(0).eval(val) - self.idx(1).eval(val),
            Token::Mult => self.idx(0).eval(val) * self.idx(1).eval(val),
            Token::Div => self.idx(0).eval(val) / self.idx(1).eval(val),
            Token::Sqrt => self.idx(0).eval(val).sqrt(),
            Token::Pow => self.idx(0).eval(val).powc(self.idx(1).eval(val)),
            Token::Sin => self.idx(0).eval(val).sin(),
            Token::Cos => self.idx(0).eval(val).cos(),
            Token::Tan => self.idx(0).eval(val).tan(),
            Token::Log => self.idx(0).eval(val).log10(),
            Token::Ln => self.idx(0).eval(val).ln(),
            Token::SENTINEL => {
                clog!("What?");
                Complex64::new(f64::INFINITY, f64::INFINITY)
            }
            _ => panic!("{}", error),
        }
    }

    pub fn flatten(&self, map: HashMap<String, Operator>) -> Operator {
        /*
        Removes unnessecary reads of pre-defined variables by flattening all
        constants into a num-type instead of ID type.
        */
        match self.token_type {
            Token::Num => self.clone(),
            Token::ID => {
                if let Some(mapped_var) = map.get(&self.symbol) {
                    mapped_var.clone()
                } else if let Some(mapped_var) = CONSTANTS.get(&self.symbol){
                    mapped_var.clone()
                } else {
                    self.clone()
                }
            }
            _ => match self.arity {
                Arities::BINARY => {
                    let mut out = self.clone();
                    out.values
                        .set_index(0, self.values.get_index(0).unwrap().flatten(map.clone()));
                    out.values
                        .set_index(1, self.values.get_index(1).unwrap().flatten(map.clone()));
                    out
                }
                Arities::UNARY => {
                    let mut out = self.clone();
                    out.values
                        .set_index(0, self.values.get_index(0).unwrap().flatten(map.clone()));
                    out
                }
                _ => self.clone(),
            },
        }
    }

    pub fn is_constant(&self) -> bool {
        clog!("Is constant, looking at: {}", self);
        match self.token_type {
            Token::Num => true,
            Token::ID => false,
            _ => match self.arity {
                Arities::BINARY => self.idx(0).is_constant() & self.idx(1).is_constant(),
                Arities::UNARY => self.idx(0).is_constant(),
                _ => true,
            },
        }
    }

    pub fn dependencies(&self) -> Vec<String> {
        let mut out = Vec::new();
        match self.token_type {
            Token::Num => out,
            Token::ID => {
                out.push(self.symbol.clone());
                out
            }
            _ => match self.arity {
                Arities::BINARY => {
                    out.append(&mut self.idx(0).dependencies());
                    out.append(&mut self.idx(1).dependencies());
                    out
                }
                Arities::UNARY => {
                    out.append(&mut self.idx(0).dependencies());
                    out
                }
                _ => out,
            },
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f)
        match self.arity {
            Arities::BINARY => {
                write!(
                    f,
                    "{}({},{})",
                    self.symbol,
                    self.values.get_index(0).unwrap(),
                    self.values.get_index(1).unwrap()
                )
            }
            Arities::UNARY => match self.token_type {
                Token::Num => write!(f, "{}", self.values),
                _ => write!(f, "{}({})", self.symbol, self.values.get_index(0).unwrap()),
            },
            Arities::NULLARY => {
                write!(f, "{}", self.symbol)
            }
        }
    }
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f)
        match self.arity {
            Arities::BINARY => {
                write!(
                    f,
                    "{}({},{})",
                    self.symbol,
                    self.values.get_index(0).unwrap(),
                    self.values.get_index(1).unwrap()
                )
            }
            Arities::UNARY => match self.token_type {
                Token::Num => write!(f, "{}", self.values),
                _ => write!(f, "{}({})", self.symbol, self.values.get_index(0).unwrap()),
            },
            Arities::NULLARY => {
                write!(f, "{}", self.symbol)
            }
        }
    }
}

lazy_static!{
    pub static ref CONSTANTS: HashMap<String,Operator> = {
        let mut out = HashMap::new();
        out.insert("i".to_string(), Operator::from_value(Value::Number(Complex64::i())));
        out.insert("pi".to_string(), Operator::from_value(Value::Number(Complex64::new(PI, 0.0))));
        out.insert("e".to_string(), Operator::from_value(Value::Number(Complex64::new(E, 0.0))));
        out.insert("phi".to_string(), Operator::from_value(Value::Number(Complex64::new(
            1.618033988749894848204586834365, 0.0))));
        out
    };
}