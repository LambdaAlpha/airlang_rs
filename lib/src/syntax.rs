use std::fmt::Debug;

use thiserror::Error;

pub use self::repr::{
    call::CallRepr,
    list::ListRepr,
    map::MapRepr,
    pair::PairRepr,
    reverse::ReverseRepr,
    Repr,
};

pub mod reserve;

pub(crate) mod repr;

pub(crate) mod parser;

pub(crate) mod generator;

#[cfg(test)]
mod test;

pub(crate) const SEPARATOR: char = ',';
pub(crate) const LIST_LEFT: char = '[';
pub(crate) const LIST_RIGHT: char = ']';
pub(crate) const MAP_LEFT: char = '{';
pub(crate) const MAP_RIGHT: char = '}';
pub(crate) const WRAP_LEFT: char = '(';
pub(crate) const WRAP_RIGHT: char = ')';

pub(crate) const STRING_QUOTE: char = '"';
pub(crate) const SYMBOL_QUOTE: char = '\'';

pub(crate) const BYTES_PREFIX: char = '#';

// keywords
pub(crate) const UNIT: &str = ".";
pub(crate) const TRUE: &str = "true";
pub(crate) const FALSE: &str = "false";
pub(crate) const SHIFT_PREFIX: &str = "^";
pub(crate) const ANNOTATION_INFIX: &str = "@";
pub(crate) const PAIR_INFIX: &str = ":";
pub(crate) const CALL_INFIX: &str = "!";
pub(crate) const REVERSE_INFIX: &str = "?";

#[derive(Error, Debug)]
#[error("ParseError:\n{msg}")]
pub struct ParseError {
    pub(crate) msg: String,
}

pub fn parse(src: &str) -> Result<Repr, ParseError> {
    parser::parse(src)
}

pub fn parse_reserve(src: &str) -> Result<reserve::Repr, ParseError> {
    parser::parse(src)
}

pub fn generate(src: &Repr) -> String {
    generator::generate_pretty(src).unwrap()
}

pub fn generate_reserve(src: &reserve::Repr) -> String {
    generator::generate_pretty(src).unwrap()
}

pub(crate) fn is_delimiter(c: char) -> bool {
    matches!(
        c,
        SEPARATOR
            | LIST_LEFT
            | LIST_RIGHT
            | MAP_LEFT
            | MAP_RIGHT
            | WRAP_LEFT
            | WRAP_RIGHT
            | STRING_QUOTE
            | SYMBOL_QUOTE
    )
}

pub(crate) fn maybe_keyword(s: &str) -> bool {
    matches!(
        s,
        UNIT | TRUE | FALSE | ANNOTATION_INFIX | PAIR_INFIX | CALL_INFIX | REVERSE_INFIX
    )
}
