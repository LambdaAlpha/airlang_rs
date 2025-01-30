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

pub(crate) const ARITY_2: char = '2';
pub(crate) const ARITY_3: char = '3';

pub(crate) const RAW: &str = "raw";
pub(crate) const ESCAPE_CHAR: char = '\\';
pub(crate) const ESCAPE: &str = concatcp!(ESCAPE_CHAR);

pub(crate) const UNIT: &str = ".";
pub(crate) const TRUE: &str = "true";
pub(crate) const FALSE: &str = "false";

pub(crate) const PAIR_CHAR: char = ':';
pub(crate) const PAIR: &str = concatcp!(PAIR_CHAR);
pub(crate) const CALL_CHAR: char = ';';
pub(crate) const CALL: &str = concatcp!(CALL_CHAR);
pub(crate) const ABSTRACT_CHAR: char = '!';
pub(crate) const ABSTRACT: &str = concatcp!(ABSTRACT_CHAR);
pub(crate) const ASK_CHAR: char = '?';
pub(crate) const ASK: &str = concatcp!(ASK_CHAR);
pub(crate) const CHANGE: &str = "->";

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
        UNIT | TRUE | FALSE | PAIR | CALL | ABSTRACT | ASK | CHANGE
    )
}

pub(crate) mod pub1;

pub(crate) mod repr;

pub(crate) mod parser;

pub(crate) mod generator;

pub(crate) mod error;

#[cfg(test)]
mod test;
