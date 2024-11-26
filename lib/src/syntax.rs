pub use pub1::*;

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

pub(crate) const LEFT: &str = "<";
pub(crate) const RIGHT: &str = ">";

pub(crate) const ARITY_1: &str = "-";
pub(crate) const ARITY_2: &str = "=";

pub(crate) const UNIT: &str = ".";
pub(crate) const TRUE: &str = "true";
pub(crate) const FALSE: &str = "false";

pub(crate) const PAIR: &str = ":";
pub(crate) const CALL: &str = ";";
pub(crate) const ABSTRACT: &str = "!";
pub(crate) const ASK: &str = "?";

pub(crate) const INT: &str = "i";
pub(crate) const NUMBER: &str = "n";
pub(crate) const BYTE: &str = "b";

pub(crate) const RAW: &str = "/";
pub(crate) const RICH: &str = "\\";

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
    matches!(s, UNIT | TRUE | FALSE | ABSTRACT | PAIR | CALL | ASK)
}

pub(crate) mod pub1;

pub(crate) mod repr;

pub(crate) mod parser;

pub(crate) mod generator;

pub(crate) mod error;

#[cfg(test)]
mod test;
