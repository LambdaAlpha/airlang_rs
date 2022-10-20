use crate::grammar::repr::Bytes;
use crate::num::{Float, Integer};

use super::{ParseError, ParseResult};

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
    Int(Integer),
    Float(Float),
    Bytes(Bytes),
    String(String),
    Letter(String),
}
