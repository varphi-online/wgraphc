use serde::{Deserialize, Serialize};
use super::operator::Operator;
use std::ops::{Index, IndexMut};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct OpVec(pub Vec<Operator>);

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
    pub fn get(&self, index: usize) -> Option<Operator> {
        self.0.get(index).cloned()
    }
    pub fn set(&mut self, index: usize, value: Operator){
        self.0[index] = value
    }
    pub fn is_empty(&self)-> bool{
        self.0.is_empty()
    }
    //pub fn get_mut(&mut self, index: usize) -> Option<&mut Operator> {
    //    self.0.get_mut(index)
    //}
    //pub fn remove(&mut self, index: usize) -> Operator {
    //    self.0.remove(index)
    //}
    //pub fn insert(&mut self, index: usize, to_insert: Operator) {
    //    if index > self.len() {
    //        self.push(to_insert);
    //        clog!(
    //            "{}",
    //            format!("Insert index out of bounds: Vector too small, added to end.")
    //        );
    //    } else {
    //        self.0.insert(index, to_insert)
    //    }
    //}
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