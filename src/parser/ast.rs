use super::token::{Arities, OpVec, Operator, Token};
use crate::util::clog;
use serde::{Deserialize, Serialize};

/*
macro_rules! clog {
    ($($t:tt)*) => {};
}
*/
#[derive(Serialize, Deserialize)]
pub struct AST {
    token_stream: OpVec,
    pub operands: OpVec,
    operators: OpVec,
    next: Operator,
    token_index: usize,
    pub value: Operator,
}

impl Default for AST {
    fn default() -> Self {
        AST {
            token_stream: OpVec::new(),
            operands: OpVec::new(),
            operators: OpVec::new(),
            next: Operator::default(),
            token_index: 1,
            value: Operator::default(),
        }
    }
}

impl AST {
    pub fn from_shunting_yard(&mut self, stream: OpVec) -> Option<Operator> {
        if stream.len() > 1 {
            self.token_stream = stream;
            self.next = self.token_stream.get(0).unwrap();

            self.operators.push(Operator::from_token(Token::SENTINEL));
            self.e();
            self.expect(Token::END);
            self.value = self
                .operands
                .get(self.operands.len() - 1)
                .expect("Shunting yard should output something");
            Some(self.value.clone())
        } else {
            None
        }
    }

    fn consume(&mut self) {
        clog!("(from_shunting_yard,consume) Current next: {}", self.next);
        if self.token_index < self.token_stream.len() {
            self.next = self
                .token_stream
                .get(self.token_index)
                .expect("why'd we get here?");
            clog!(
                "(from_shunting_yard,consume) New current next: {}",
                self.next
            );
            self.token_index += 1;
        }
    }

    fn expect(&mut self, expected: Token) {
        if self.next.token_type == expected {
            self.consume();
        } else {
            clog!(
                "(from_shunting_yard,expect) Expected: {}, got {}",
                expected,
                self.next.token_type
            );
        }
    }

    fn e(&mut self) {
        self.p();
        while !(self.next.token_type == Token::END) && self.next.arity == Arities::BINARY {
            self.push_operator(self.next.clone());
            self.consume();
            self.p();
        }
        while !(self
            .operators
            .get(self.operators.len() - 1)
            .unwrap()
            .token_type
            == Token::SENTINEL)
        {
            self.pop_operator();
        }
    }

    fn p(&mut self) {
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

    fn pop_operator(&mut self) {
        if self.operators.get(self.operators.len() - 1).unwrap().arity == Arities::BINARY {
            let t1: Operator = self.operands.pop().unwrap();
            let t2: Operator = self.operands.pop().unwrap();
            let mut toadd: Operator = self.operators.pop().unwrap();
            toadd.values.set_index(1, t1);
            toadd.values.set_index(0, t2);
            self.operands.push(toadd);
        } else {
            let mut toadd = self.operators.pop().unwrap();
            toadd.values.set_index(0, self.operands.pop().unwrap());
            self.operands.push(toadd);
        }
    }

    fn push_operator(&mut self, op: Operator) {
        while !(self
            .operators
            .get(self.operators.len() - 1)
            .unwrap()
            .token_type
            == Token::SENTINEL)
            && self
                .operators
                .get(self.operators.len() - 1)
                .unwrap()
                .precedence
                < op.precedence
        {
            self.pop_operator();
        }
        self.operators.push(op)
    }
}
