use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Formatter,
    },
};

pub use self::repr::{
    Repr,
    ask::AskRepr,
    call::CallRepr,
    list::ListRepr,
    map::MapRepr,
    pair::PairRepr,
};
use crate::syntax::generator::{
    COMPACT_FMT,
    PRETTY_FMT,
};

// delimiters

pub(crate) const LIST_LEFT: char = '[';
pub(crate) const LIST_RIGHT: char = ']';
pub(crate) const MAP_LEFT: char = '{';
pub(crate) const MAP_RIGHT: char = '}';
pub(crate) const SCOPE_LEFT: char = '(';
pub(crate) const SCOPE_RIGHT: char = ')';

pub(crate) const SPACE: char = ' ';
pub(crate) const SEPARATOR: char = ',';

pub(crate) const TEXT_QUOTE: char = '"';
pub(crate) const SYMBOL_QUOTE: char = '\'';

// keywords

pub(crate) const UNIT: &str = ".";
pub(crate) const TRUE: &str = "true";
pub(crate) const FALSE: &str = "false";

pub(crate) const LEFT: &str = "<";
pub(crate) const RIGHT: &str = ">";

pub(crate) const ADAPT: &str = ";";
pub(crate) const PAIR: &str = ":";
pub(crate) const CALL: &str = "!";
pub(crate) const ASK: &str = "?";

pub(crate) const INT: &str = "i";
pub(crate) const NUMBER: &str = "n";
pub(crate) const BYTE: &str = "b";

pub(crate) const RAW: &str = "/";
pub(crate) const RICH: &str = "\\";

#[derive(Debug)]
pub struct ParseError {
    pub(crate) msg: String,
}

pub fn parse(src: &str) -> Result<Repr, ParseError> {
    parser::parse(src)
}

pub fn generate_pretty(src: &Repr) -> String {
    generator::generate(src.try_into().unwrap(), PRETTY_FMT)
}

pub fn generate_compact(src: &Repr) -> String {
    generator::generate(src.try_into().unwrap(), COMPACT_FMT)
}

pub(crate) fn is_delimiter(c: char) -> bool {
    matches!(
        c,
        SPACE
            | SEPARATOR
            | LIST_LEFT
            | LIST_RIGHT
            | MAP_LEFT
            | MAP_RIGHT
            | SCOPE_LEFT
            | SCOPE_RIGHT
            | TEXT_QUOTE
            | SYMBOL_QUOTE
    )
}

pub(crate) fn ambiguous(s: &str) -> bool {
    matches!(s, UNIT | TRUE | FALSE | ADAPT | PAIR | CALL | ASK)
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError\n{}", self.msg)
    }
}

impl Error for ParseError {}

pub(crate) mod repr;

pub(crate) mod parser;

pub(crate) mod generator;

#[cfg(test)]
mod test;
