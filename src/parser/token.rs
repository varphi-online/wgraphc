use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};

pub struct Operator {
    arity: Arities,
    token_type: Token,
    pub symbol: String,
    pub values: Value,
    precedence: u8,
    katex_repr: String,
}

impl Default for Operator {
    fn default() -> Self {
        Operator {
            token_type: Token::Null,
            arity: Arities::NULLARY,
            symbol: format!(""),
            values: Value::Op(OpVec(Vec::new())),
            precedence: 255,
            katex_repr: format!(""),
        }
    }
}

impl Operator {
    fn new() -> Operator {
        Operator {
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
                Token::Num => write!(f, "{}({})", self.symbol, self.values),
                _ => write!(f, "{}({})", self.symbol, self.values.get_index(0).unwrap()),
            },
            Arities::NULLARY => {
                write!(f, "{}", self.symbol)
            }
        }
    }
}

pub struct OpVec(Vec<Operator>);

impl OpVec {
    fn push(&mut self, value: Operator) {
        self.0.push(value);
    }
}

impl fmt::Display for OpVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lol")
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

#[derive(Default)]
enum Arities {
    #[default]
    BINARY,
    UNARY,
    NULLARY,
}

pub enum Value {
    Op(OpVec),
    Number(f64),
}

impl Value {
    fn set_index(&mut self, i: usize, val: Operator) {
        if let Value::Op(ref mut opvector) = self {
            opvector.0[i] = val;
        }
    }
    fn get_index(&self, i: usize) -> Option<&Operator> {
        if let Value::Op(ref opvector) = self {
            Some(&opvector.0[i])
        } else {
            None
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Op(op) => write!(f, "{}", op),
            Value::Number(num) => write!(f, "{}", num),
        }
    }
}

#[derive(Default, Clone, PartialEq)]
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
}

pub fn make_token(type_of_token: Token) -> Operator {
    let mut out: Operator = Operator {
        ..Default::default()
    };
    out.token_type = type_of_token.clone();
    match type_of_token {
        Token::END => {
            out.arity = Arities::NULLARY;
        }
        Token::SENTINEL => {
            out.arity = Arities::NULLARY;
            out.precedence = 0;
        }
        Token::Null => {
            out.arity = Arities::NULLARY;
        }
        Token::Num => {
            out.arity = Arities::UNARY;
        }
        Token::ID => {
            out.arity = Arities::UNARY;
        }
        Token::Add => {
            out.arity = Arities::BINARY;
            out.symbol = format!("+");
            out.precedence = 4;
        }
        Token::Sub => {
            out.arity = Arities::BINARY;
            out.symbol = format!("-");
            out.precedence = 4;
        }
        Token::Mult => {
            out.arity = Arities::BINARY;
            out.symbol = format!("*");
            out.precedence = 2;
        }
        Token::Div => {
            out.arity = Arities::BINARY;
            out.symbol = format!("/");
            out.precedence = 2;
        }
        Token::Sqrt => {
            out.arity = Arities::UNARY;
            out.symbol = format!("√");
            out.precedence = 2;
        }
        Token::Pow => {
            out.arity = Arities::BINARY;
            out.symbol = format!("^");
            out.precedence = 2;
        }
        Token::OpenPar => {
            out.arity = Arities::NULLARY;
            out.symbol = format!("(");
            out.precedence = 2;
        }
        Token::ClosePar => {
            out.arity = Arities::NULLARY;
            out.symbol = format!(")");
            out.precedence = 2;
        }
    }
    match out.arity {
        Arities::BINARY => out.values = Value::Op(OpVec(vec![Operator::new(), Operator::new()])),
        Arities::UNARY => {
            if out.token_type == Token::Num {
                out.values = Value::Number(f64::from(0));
            } else {
                out.values = Value::Op(OpVec(vec![Operator::new()]));
            }
        }
        _ => (),
    }
    out
}
