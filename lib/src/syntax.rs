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

const SEPARATOR: char = ',';
const TOKENS_QUOTE: char = '`';
const LIST_LEFT: char = '[';
const LIST_RIGHT: char = ']';
const MAP_LEFT: char = '{';
const MAP_RIGHT: char = '}';
const WRAP_LEFT: char = '(';
const WRAP_RIGHT: char = ')';

const STRING_QUOTE: char = '"';
const SYMBOL_QUOTE: char = '\'';

const BYTES_PREFIX: char = '#';
const PRESERVED_PREFIX: char = '.';

const ANNOTATION_INFIX: char = '@';
const PAIR_INFIX: char = ':';
const CALL_INFIX: char = '$';
const REVERSE_INFIX: char = '?';

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

pub(crate) fn is_special(c: char) -> bool {
    matches!(
        c,
        LIST_LEFT
            | LIST_RIGHT
            | MAP_LEFT
            | MAP_RIGHT
            | WRAP_LEFT
            | WRAP_RIGHT
            | SEPARATOR
            | TOKENS_QUOTE
    )
}
