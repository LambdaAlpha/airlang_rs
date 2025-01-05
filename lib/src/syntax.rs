use const_format::concatcp;
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

pub(crate) const LEFT: char = '<';
pub(crate) const RIGHT: char = '>';

pub(crate) const ARITY_1: char = '-';
pub(crate) const ARITY_2: char = '=';

pub(crate) const RAW: &str = "raw";
pub(crate) const ESCAPE: char = '\\';
pub(crate) const ESCAPE_STR: &str = concatcp!(ESCAPE);

pub(crate) const UNIT: &str = ".";
pub(crate) const TRUE: &str = "true";
pub(crate) const FALSE: &str = "false";

pub(crate) const PAIR: char = ':';
pub(crate) const PAIR_STR: &str = concatcp!(PAIR);
pub(crate) const CALL: char = ';';
pub(crate) const CALL_STR: &str = concatcp!(CALL);
pub(crate) const ABSTRACT: char = '!';
pub(crate) const ABSTRACT_STR: &str = concatcp!(ABSTRACT);
pub(crate) const ASK: char = '?';
pub(crate) const ASK_STR: &str = concatcp!(ASK);

pub(crate) const INT: &str = "integer";
pub(crate) const NUMBER: &str = "number";
pub(crate) const BYTE: &str = "byte";

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
    matches!(
        s,
        UNIT | TRUE | FALSE | PAIR_STR | CALL_STR | ABSTRACT_STR | ASK_STR
    )
}

pub(crate) mod pub1;

pub(crate) mod repr;

pub(crate) mod parser;

pub(crate) mod generator;

pub(crate) mod error;

#[cfg(test)]
mod test;
