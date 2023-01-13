use crate::grammar::repr::{
    Bytes,
    Float,
    Int,
};

pub(crate) mod deep;
pub(crate) mod infix;
pub(crate) mod lexer;
pub(crate) mod postfix;
pub(crate) mod prefix;
pub(crate) mod repr;

#[derive(Debug)]
pub(crate) enum AtomNode {
    Unit,
    Bool(bool),
    Int(Int),
    Float(Float),
    Bytes(Bytes),
    String(String),
    Letter(String),
}
