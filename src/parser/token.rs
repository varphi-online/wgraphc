use std::fmt;

#[derive( Clone)]
pub struct Operator {
    arity: Arities,
    token_type: Token,
    symbol: String,
    values: Vec<Operator>,
    precedence: u8,
    katex_repr: String,
}

impl Default for Operator{
    fn default() -> Self{
        Operator{
        token_type: Token::Null,
        arity: Arities::NULLARY,
        symbol: format!(""),
        values: Vec::new(),
        precedence: 255,
        katex_repr:  format!("")
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f)
        write!(f, "{}", self.symbol)
    }
}

#[derive(Default, Clone)]
enum Arities {
    #[default]
    BINARY,
    UNARY,
    NULLARY,
}

//pub trait evaluator {
//    fn eval(&self) -> evaluator;
//}

#[derive(Default, Clone)]
pub enum Token {
    #[default]
    Null,
    Num,
    ID,
    Add,
    Sub,
    Mult,
    Div,
    Sqrt,
    Pow,
    END,
    SENTINEL,
    OpenPar,
    ClosePar,
}

pub fn make_token(type_of_token: Token) -> Operator {
    match type_of_token {
        Token::Num => {
            return Operator {
                token_type: type_of_token,
                arity: Arities::UNARY,
                ..Default::default()
            };
        }
        Token::ID => {
            return Operator {
                token_type: type_of_token,
                arity: Arities::UNARY,
                ..Default::default()
            };
        }
        Token::Add => {
            return Operator {
                token_type: type_of_token,
                arity: Arities::BINARY,
                symbol: format!("+"),
                precedence: 4,
                ..Default::default()
            };
        }
        _ => todo!()
    }
}
