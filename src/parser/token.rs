use crate::util::*;
use lazy_static::lazy_static;
use num_complex::*;
use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
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
    pub fn from_token(type_of_token: Token) -> Operator {
        let arity: Arities = match type_of_token {
            Token::END | Token::SENTINEL | Token::Null | Token::OpenPar | Token::ClosePar => {
                Arities::NULLARY
            }
            Token::Num | Token::ID | Token::Sqrt => Arities::UNARY,
            Token::Add | Token::Sub | Token::Mult | Token::Div | Token::Pow => Arities::BINARY,
        };
        let arity_copy = arity.clone();
        Operator {
            arity,
            token_type: type_of_token.clone(),
            precedence: match &type_of_token {
                Token::SENTINEL | Token::OpenPar | Token::ClosePar => 0,
                Token::Pow | Token::Sqrt => 1,
                Token::Mult | Token::Div => 2,
                Token::Add | Token::Sub => 3,
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
                Token::OpenPar => "(",
                Token::ClosePar => ")",
                _ => "",
            })
            .to_string(),
            ..Default::default()
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

#[derive(Clone)]
pub struct OpVec(Vec<Operator>);

impl OpVec {
    pub fn new() -> OpVec {
        OpVec(Vec::new())
    }
    pub fn push(&mut self, value: Operator) {
        self.0.push(value);
    }
}

impl fmt::Display for OpVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out: String = "".to_string();
        for entry in &self.0 {
            out += &entry.to_string();
            out += ",";
        }
        write!(f, "{}", out)
    }
}
impl Index<usize> for OpVec {
    type Output = Operator;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for OpVec {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl IntoIterator for OpVec {
    type Item = Operator;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Default, Clone)]
enum Arities {
    #[default]
    BINARY,
    UNARY,
    NULLARY,
}

#[derive(Clone)]
pub enum Value {
    Op(OpVec),
    Number(Complex64),
    Real(f64),
    Imag(f64),
}

impl Value {
    pub fn set_index(&mut self, i: usize, val: Operator) {
        if let Value::Op(ref mut opvector) = self {
            opvector.0[i] = val;
        }
    }
    pub fn get_index(&self, i: usize) -> Option<&Operator> {
        if let Value::Op(ref opvector) = self {
            Some(&opvector.0[i])
        } else {
            None
        }
    }
    pub fn set_num(&mut self, number: Complex64) {
        if let Value::Number(ref mut num) = *self {
            *num = number;
        }
    }
    pub fn get_num(&self) -> Option<f64> {
        if let Value::Real(ref number) = self {
            Some(*number)
        } else if let Value::Imag(ref number) = self {
            Some(*number)
        } else {
            None
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Real(out) => write!(f, "{}", out),
            Value::Imag(num) => write!(f, "{}i", num),
            Value::Op(out) => write!(f, "{}", out),
            Value::Number(out) => write!(f, "{}", out),
        }
    }
}

#[derive(Default, Clone, PartialEq, Hash, Eq)]
pub enum Token {
    #[default]
    END,
    SENTINEL,
    Null,
    Num,
    ID,
    Add,
    Sub,
    Mult,
    Div,
    Sqrt,
    Pow,
    OpenPar,
    ClosePar,
}

lazy_static! {
    pub static ref OPERATORS: HashMap<String, Token> = {
        let mut out = HashMap::new();
        out.insert("+".to_string(), Token::Add);
        out.insert("-".to_string(), Token::Sub);
        out.insert("*".to_string(), Token::Mult);
        out.insert("/".to_string(), Token::Div);
        out.insert("sqrt".to_string(), Token::Sqrt);
        out.insert("^".to_string(), Token::Pow);
        out.insert("(".to_string(), Token::OpenPar);
        out.insert(")".to_string(), Token::ClosePar);
        out
    };
    static ref SYMBOLS: HashMap<Token, String> = OPERATORS
        .iter()
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect();
}
