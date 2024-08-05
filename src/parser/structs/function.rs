use std::collections::HashMap;

use super::{operator::Operator, value::Value};
pub enum Function {
    Constant(Value),
    Point(Value,Value),
    Variable(Operator)
}

/*
TODO: Change how JS handles variable declarations to output a hashmap of a
serialized operator and string to allow for functional variables.
*/

impl Function {
    pub fn from_operator(input: Operator) -> Function {
        todo!()
    }

    pub fn flatten(&self,map: HashMap<String,String>) -> Option<Operator> {
        match &self {
            Self::Variable(function) => {
                Some(function.flatten(map))
            },
            _ => None
        }
    }
}