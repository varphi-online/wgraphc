use super::{op_vec::OpVec, operator::Operator};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub enum Value {
    Op(OpVec),
    Number(Complex64),
    Real(f64),
    Imag(f64),
}

impl Value {
    pub fn set_index(&mut self, i: usize, val: Operator) {
        if let Value::Op(ref mut opvector) = self {
            opvector.set(i, val);
        }
    }
    pub fn get_index(&self, i: usize) -> Option<Operator> {
        if let Value::Op(ref opvector) = self {
            opvector.get(i)
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
