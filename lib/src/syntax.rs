pub use self::error::ParseError;
pub use self::error::ReprError;
pub use self::generator::GenRepr;
pub use self::generator::escape_symbol;
pub use self::generator::escape_text;
pub use self::generator::escape_text_symbol;
pub use self::parser::ParseRepr;
pub use self::parser::parse;

_____!();

use derive_more::IsVariant;

use self::generator::COMPACT_FMT;
use self::generator::PRETTY_FMT;
use self::generator::SYMBOL_FMT;

pub fn generate_pretty(src: GenRepr) -> String {
    generator::generate(src, PRETTY_FMT)
}

pub fn generate_compact(src: GenRepr) -> String {
    generator::generate(src, COMPACT_FMT)
}

pub fn generate_symbol(src: GenRepr) -> String {
    generator::generate(src, SYMBOL_FMT)
}

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

pub(crate) const COMMENT: &str = "_";

pub(crate) const LEFT: &str = "<";
pub(crate) const RIGHT: &str = ">";

pub(crate) const UNIT: &str = ".";
pub(crate) const TRUE: &str = "true";
pub(crate) const FALSE: &str = "false";

pub(crate) const INT: &str = "integer";
pub(crate) const NUMBER: &str = "number";
pub(crate) const BYTE: &str = "byte";
pub(crate) const CALL: &str = "call";
pub(crate) const SOLVE: &str = "solve";

pub(crate) const PAIR: &str = ":";
pub(crate) const CTX: &str = "|";

pub(crate) const QUOTE: char = '`';

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
    matches!(s, UNIT | TRUE | FALSE | PAIR | CTX)
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
