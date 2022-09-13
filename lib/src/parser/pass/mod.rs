use crate::num::{Float, Integer};
use crate::val::Bytes;

pub mod deep;
pub mod flat;
pub mod infix;
pub mod postfix;
pub mod prefix;
pub mod val;

pub enum AtomNode {
    Bool(bool),
    Int(Integer),
    Float(Float),
    Bytes(Bytes),
    String(String),
    Letter(String),
}
