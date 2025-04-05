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

pub(crate) const INLINE_COMMENT: &str = "_";
pub(crate) const MULTILINE_COMMENT: &str = "__";

pub(crate) const LEFT: &str = "<";
pub(crate) const RIGHT: &str = ">";

pub(crate) const ARITY_2: &str = "2";
pub(crate) const ARITY_3: &str = "3";

pub(crate) const TAG_CHAR: char = '#';
pub(crate) const TAG: &str = concatcp!(TAG_CHAR);

pub(crate) const UNIT: &str = ".";
pub(crate) const TRUE: &str = "true";
pub(crate) const FALSE: &str = "false";

pub(crate) const PAIR_CHAR: char = ':';
pub(crate) const PAIR: &str = concatcp!(PAIR_CHAR);
pub(crate) const CHANGE: &str = "->";
pub(crate) const CALL_CHAR: char = ';';
pub(crate) const CALL: &str = concatcp!(CALL_CHAR);

pub(crate) const OPTIMIZE_CHAR: char = '!';
pub(crate) const OPTIMIZE: &str = concatcp!(OPTIMIZE_CHAR);
pub(crate) const SOLVE_CHAR: char = '?';
pub(crate) const SOLVE: &str = concatcp!(SOLVE_CHAR);
pub(crate) const ABSTRACT: &str = "@";

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
    matches!(s, UNIT | TRUE | FALSE | PAIR | CHANGE | CALL)
}

pub(crate) mod pub1;

pub(crate) mod repr;

pub(crate) mod parser;

pub(crate) mod generator;

pub(crate) mod error;

#[cfg(test)]
mod test;
