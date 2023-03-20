use {
    crate::repr::Repr,
    std::fmt::Debug,
    thiserror::Error,
};

mod generator;
mod parser;
#[cfg(test)]
mod test;

const COMMENT_PREFIX: char = '#';

const PRESERVE_PREFIX: char = '\'';

const SEPARATOR: char = ',';
const PAIR_SEPARATOR: char = ':';
const REVERSE_SEPARATOR: char = '?';
const LIST_LEFT: char = '(';
const LIST_RIGHT: char = ')';
const MAP_LEFT: char = '{';
const MAP_RIGHT: char = '}';
const WRAP_LEFT: char = '[';
const WRAP_RIGHT: char = ']';

#[derive(Error, Debug)]
#[error("ParseError:\n{msg}")]
pub struct ParseError {
    pub(crate) msg: String,
}

pub fn parse(src: &str) -> Result<Repr, ParseError> {
    parser::parse(src)
}

pub fn generate(src: &Repr) -> String {
    generator::generate_pretty(src)
}
