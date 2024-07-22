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
                Token::END => "END",
                _ => "",
            })
            .to_string(),
            ..Default::default()
        }
    }
    pub fn eval(&self, variable: Complex64) -> Complex64 {
        let error: &str = "Improperly constructed input";
        match self.token_type {
            Token::Num => self.values.get_complex().expect(error),
            Token::ID => variable,
            Token::Add => {
                self.values.get_index(0).expect(error).eval(variable)
                    + self.values.get_index(1).expect(error).eval(variable)
            }
            Token::Sub => {
                self.values.get_index(0).expect(error).eval(variable)
                    - self.values.get_index(1).expect(error).eval(variable)
            }
            Token::Mult => {
                self.values.get_index(0).expect(error).eval(variable)
                    * self.values.get_index(1).expect(error).eval(variable)
            }
            Token::Div => {
                self.values.get_index(0).expect(error).eval(variable)
                    / self.values.get_index(1).expect(error).eval(variable)
            }
            Token::Sqrt => self.values.get_index(0).expect(error).eval(variable).sqrt(),
            Token::Pow => self
                .values
                .get_index(0)
                .expect(error)
                .eval(variable)
                .powc(self.values.get_index(1).expect(error).eval(variable)),
            Token::SENTINEL => {
                clog!("What?");
                Complex64::new(f64::INFINITY, f64::INFINITY)
            }
            _ => panic!("{}", error),
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
#[derive(Clone)]
pub struct OpVec(Vec<Operator>);

impl OpVec {
    pub fn new() -> OpVec {
        OpVec(Vec::new())
    }
    pub fn push(&mut self, value: Operator) {
        self.0.push(value);
    }
    pub fn pop(&mut self) -> Option<Operator> {
        self.0.pop()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Operator> {
        self.0.get_mut(index)
    }
    pub fn get(&self, index: usize) -> Option<Operator> {
        self.0.get(index).cloned()
    }
    pub fn remove(&mut self, index: usize) -> Operator {
        self.0.remove(index)
    }
    pub fn insert(&mut self, index: usize, to_insert: Operator) {
        if index > self.len() {
            self.push(to_insert);
            clog!(
                "{}",
                format!("Insert index out of bounds: Vector too small, added to end.")
            );
        } else {
            self.0.insert(index, to_insert)
        }
    }
}

impl fmt::Display for OpVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out: String = "".to_string();
        for i in 0..self.0.len() {
            out += &self[i].to_string();
            if i + 1 < self.0.len() {
                out += ",";
            }
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

impl FromIterator<Operator> for OpVec {
    fn from_iter<I: IntoIterator<Item = Operator>>(iter: I) -> Self {
        let mut c = OpVec::new();
        for i in iter {
            c.push(i);
        }
        c
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub enum Arities {
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
    pub fn get_complex(&self) -> Option<Complex64> {
        if let Value::Number(ref number) = self {
            Some(*number)
        } else {
            None
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
    pub fn get_type(&self) -> u8 {
        match self {
            Value::Number(_) => 3,
            Value::Real(_) => 1,
            Value::Imag(_) => 2,
            _ => 4,
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: String = (match self {
            Token::Add => "+",
            Token::Sub => "-",
            Token::Mult => "*",
            Token::Div => "/",
            Token::Sqrt => "sqrt",
            Token::Pow => "^",
            Token::OpenPar => "(",
            Token::ClosePar => ")",
            Token::END => "END",
            Token::SENTINEL => "SENTINEL",
            Token::Num => "Num",
            Token::ID => "ID",
            Token::Null => "Null",
        })
        .to_string();

        write!(f, "{}", out)
    }
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
