pub use self::error::ParseError;
pub use self::generator::FmtCtx;
pub use self::generator::FmtRepr;
pub use self::parser::ParseRepr;
pub use self::parser::parse;

_____!();

use derive_more::IsVariant;

// delimiters {
pub(crate) const LIST_LEFT: char = '[';
pub(crate) const LIST_RIGHT: char = ']';
pub(crate) const MAP_LEFT: char = '{';
pub(crate) const MAP_RIGHT: char = '}';
pub(crate) const SCOPE_LEFT: char = '(';
pub(crate) const SCOPE_RIGHT: char = ')';

pub(crate) const SPACE: char = ' ';
pub(crate) const SEPARATOR: char = ',';

pub(crate) const TEXT_QUOTE: char = '"';
pub(crate) const KEY_QUOTE: char = '\'';
// } delimiters

// keywords {
pub(crate) const EMPTY: &str = "_";
pub(crate) const UNIT: &str = ".";
pub(crate) const PAIR: &str = ":";

pub(crate) const TRUE: &str = "true";
pub(crate) const FALSE: &str = "false";
// } keywords

// prefixes {
pub(crate) const LEFT: &str = "<";
pub(crate) const RIGHT: &str = ">";

pub(crate) const INT: &str = "integer";
pub(crate) const DECIMAL: &str = "decimal";
pub(crate) const BYTE: &str = "byte";
// } prefixes

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
            | KEY_QUOTE
    )
}

pub(crate) fn keyword(s: &str) -> bool {
    matches!(s, EMPTY | UNIT | PAIR | TRUE | FALSE)
}

#[derive(Default, Copy, Clone, PartialEq, Eq, IsVariant)]
enum Direction {
    Left,
    #[default]
    Right,
}

pub mod repr;

mod parser;

mod generator;

mod error;

#[cfg(test)]
mod test;
