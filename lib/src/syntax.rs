use {
    std::fmt::Debug,
    thiserror::Error,
};

pub use self::repr::{
    call::CallRepr,
    list::ListRepr,
    map::MapRepr,
    pair::PairRepr,
    reverse::ReverseRepr,
    Repr,
};

pub(crate) mod repr;

pub(crate) mod parser;

pub(crate) mod generator;

#[cfg(test)]
mod test;

const SEPARATOR: char = ',';
const LIST_LEFT: char = '[';
const LIST_RIGHT: char = ']';
const MAP_LEFT: char = '{';
const MAP_RIGHT: char = '}';
const WRAP_LEFT: char = '(';
const WRAP_RIGHT: char = ')';

const STRING_QUOTE: char = '"';
const ESCAPED_PREFIX: char = '\'';
const COMMENT_SEPARATOR: char = '@';
const PAIR_SEPARATOR: char = ':';
const CALL_SEPARATOR: char = '$';
const REVERSE_SEPARATOR: char = '?';

#[derive(Error, Debug)]
#[error("ParseError:\n{msg}")]
pub struct ParseError {
    pub(crate) msg: String,
}

pub fn parse(src: &str) -> Result<Repr, ParseError> {
    parser::parse(src)
}

pub fn generate(src: &Repr) -> String {
    generator::generate_pretty(src).unwrap()
}
