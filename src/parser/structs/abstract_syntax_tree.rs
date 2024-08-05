use super::{arity::Arities, op_vec::OpVec, operator::Operator, token::Token};
use crate::util::clog;
use serde::{Deserialize, Serialize};

/*
macro_rules! clog {
    ($($t:tt)*) => {};
}
*/

#[derive(Serialize, Deserialize)]
pub struct AbstractSyntaxTree {
    token_stream: OpVec,
    operands: OpVec,
    operators: OpVec,
    next: Operator,
    token_index: usize,
    error: bool,
    pub value: Operator,
}

impl Default for AbstractSyntaxTree {
    fn default() -> Self {
        AbstractSyntaxTree {
            token_stream: OpVec::new(),
            operands: OpVec::new(),
            operators: OpVec::new(),
            next: Operator::default(),
            token_index: 1,
            error: false,
            value: Operator::default(),
        }
    }
}

impl AbstractSyntaxTree {
    pub fn from_shunting_yard(&mut self, stream: OpVec) -> Option<Operator> {
        if stream.len() > 1 {
            //clog!("SHUNTING YARD START ------------------");
            self.token_stream = stream.clone();
            //clog!("Token stream: {}", self.token_stream);
            self.next = self.token_stream.get(0).unwrap_or_else(|| self.error());
            //clog!("First token err status: {}",self.error);

            self.operators.push(Operator::from_token(Token::SENTINEL));
            self.e();
            self.expect(Token::END);

            if !self.operands.is_empty() {
                self.value = self
                    .operands
                    .get(self.operands.len() - 1)
                    .unwrap_or_else(|| self.error());
            } else {
                self.error();
            }
            //clog!("Error status: {}", self.error);
            if !self.error {
                Some(self.value.clone())
            } else {
                clog!("Error somehwere in AST creation, \n\nAST dump:\n{}",serde_json::to_string(self).unwrap());
                None
            }
        } else {
            None
        }
    }

    fn consume(&mut self) {
        //clog!("(consume) Dumping self: {}",serde_json::to_string(self).unwrap());
        //clog!("(from_shunting_yard,consume) Current next: {}", self.next);
        if self.token_index < self.token_stream.len() && !self.error {
            self.next = self
                .token_stream
                .get(self.token_index)
                .unwrap_or_else(|| self.error());
            //clog!("(from_shunting_yard,consume) New current next: {}",self.next);
            self.token_index += 1;
        }
    }

    fn expect(&mut self, expected: Token) {
        //clog!("(expext) Dumping self: {}",serde_json::to_string(self).unwrap());
        if self.next.token_type == expected && !self.error {
            self.consume();
        } else {
            self.error();
            //clog!("(from_shunting_yard,expect) Expected: {}, got {}",expected,self.next.token_type);
        }
    }

    fn e(&mut self) {
        //clog!("(e) Dumping self: {}",serde_json::to_string(self).unwrap());
        if !self.error {
            self.p();
            while !self.error
                && !(self.next.token_type == Token::END)
                && self.next.arity == Arities::BINARY
            {
                self.push_operator(self.next.clone());
                self.consume();
                self.p();
            }
            while !self.error
                && !(self
                    .operators
                    .get(self.operators.len() - 1)
                    .unwrap_or_else(|| self.error())
                    .token_type
                    == Token::SENTINEL)
            {
                self.pop_operator();
            }
        }
    }

    fn p(&mut self) {
        //clog!("(p) Dumping self: {}",serde_json::to_string(self).unwrap());
        if !self.error {
            if self.next.token_type == Token::ID || self.next.token_type == Token::Num {
                self.operands.push(self.next.clone());
                self.consume();
            } else if self.next.token_type == Token::OpenPar {
                self.consume();
                self.operators.push(Operator::from_token(Token::SENTINEL));
                self.e();
                self.expect(Token::ClosePar);
                self.operators.pop();
            } else if self.next.arity == Arities::UNARY {
                self.push_operator(self.next.clone());
                self.consume();
                self.p();
            }
        }
    }

    fn pop_operator(&mut self) {
        //clog!("(popop) Dumping self: {}",serde_json::to_string(self).unwrap());
        if !self.error {
            if self
                .operators
                .get(self.operators.len() - 1)
                .unwrap_or_else(|| self.error())
                .arity
                == Arities::BINARY
            {
                let t1: Operator = self.operands.pop().unwrap_or_else(|| self.error());
                let t2: Operator = self.operands.pop().unwrap_or_else(|| self.error());
                let mut toadd: Operator = self.operators.pop().unwrap_or_else(|| self.error());
                toadd.values.set_index(1, t1);
                toadd.values.set_index(0, t2);
                self.operands.push(toadd);
            } else {
                let mut toadd = self.operators.pop().unwrap_or_else(|| self.error());
                toadd
                    .values
                    .set_index(0, self.operands.pop().unwrap_or_else(|| self.error()));
                self.operands.push(toadd);
            }
        }
    }

    fn push_operator(&mut self, op: Operator) {
        //clog!("(pushop) Dumping self: {}",serde_json::to_string(self).unwrap());
        if !self.error {
            while !self.error
                && !(self
                    .operators
                    .get(self.operators.len() - 1)
                    .unwrap_or_else(|| self.error())
                    .token_type
                    == Token::SENTINEL)
                && self
                    .operators
                    .get(self.operators.len() - 1)
                    .unwrap_or_else(|| self.error())
                    .precedence
                    < op.precedence
            {
                self.pop_operator();
            }
            self.operators.push(op);
        }
    }

    fn error(&mut self) -> Operator {
        self.error = true;
        //clog!("Throwing error, dumping self: {}",serde_json::to_string(self).unwrap());
        Operator::from_token(Token::Null)
    }
}
