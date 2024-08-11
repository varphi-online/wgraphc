use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Default, Clone, PartialEq, Hash, Eq, Deserialize, Serialize)]
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
    Sin,
    Cos,
    Tan,
    Log,
    Ln,
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
            Token::Sin => "sin",
            Token::Cos => "cos",
            Token::Tan => "tan",
            Token::Log => "log",
            Token::Ln => "ln",
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
        out.insert("sin".to_string(), Token::Sin);
        out.insert("cos".to_string(), Token::Cos);
        out.insert("tan".to_string(), Token::Tan);
        out.insert("log".to_string(), Token::Log);
        out.insert("ln".to_string(), Token::Ln);
        out.insert("^".to_string(), Token::Pow);
        out.insert("(".to_string(), Token::OpenPar);
        out.insert(")".to_string(), Token::ClosePar);
        out
    };
}
