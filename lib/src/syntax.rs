use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Formatter,
    },
};

pub use self::repr::{
    ask::AskRepr,
    call::CallRepr,
    list::ListRepr,
    map::MapRepr,
    pair::PairRepr,
    Repr,
};

// delimiters

pub(crate) const LIST_LEFT: char = '[';
pub(crate) const LIST_RIGHT: char = ']';
pub(crate) const MAP_LEFT: char = '{';
pub(crate) const MAP_RIGHT: char = '}';
pub(crate) const WRAP_LEFT: char = '(';
pub(crate) const WRAP_RIGHT: char = ')';

pub(crate) const SEPARATOR: char = ',';

pub(crate) const TEXT_QUOTE: char = '"';
pub(crate) const SYMBOL_QUOTE: char = '\'';

// prefixes

pub(crate) const BYTE_PREFIX: char = '#';

// keywords

pub(crate) const UNIT: &str = ".";
pub(crate) const TRUE: &str = "true";
pub(crate) const FALSE: &str = "false";

pub(crate) const MIDDLE: &str = "^";
pub(crate) const LEFT: &str = "<";
pub(crate) const RIGHT: &str = ">";
pub(crate) const LEFT_CALL_PREFIX: &str = "<!";
pub(crate) const LEFT_ASK_PREFIX: &str = "<?";
pub(crate) const RIGHT_CALL_PREFIX: &str = ">!";
pub(crate) const RIGHT_ASK_PREFIX: &str = ">?";

pub(crate) const COMMENT: &str = "@";
pub(crate) const PAIR: &str = ":";
pub(crate) const CALL: &str = "!";
pub(crate) const ASK: &str = "?";

#[derive(Debug)]
pub struct ParseError {
    pub(crate) msg: String,
}

pub fn parse(src: &str) -> Result<Repr, ParseError> {
    parser::parse(src)
}

pub fn generate_pretty(src: &Repr) -> String {
    generator::generate_pretty(src).unwrap()
}

pub fn generate_compact(src: &Repr) -> String {
    generator::generate_compact(src).unwrap()
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
            | TEXT_QUOTE
            | SYMBOL_QUOTE
    )
}

pub(crate) fn maybe_keyword(s: &str) -> bool {
    matches!(s, UNIT | TRUE | FALSE | COMMENT | PAIR | CALL | ASK)
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
