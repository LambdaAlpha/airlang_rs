use super::{ParseError, ParseResult};
use crate::num::{Float, Integer};
use crate::val::Bytes;

pub(crate) mod deep;
pub(crate) mod infix;
pub(crate) mod lexer;
pub(crate) mod postfix;
pub(crate) mod prefix;
pub(crate) mod val;

#[derive(Debug)]
pub(crate) enum AtomNode {
    Bool(bool),
    Int(Integer),
    Float(Float),
    Bytes(Bytes),
    String(String),
    Letter(String),
}
