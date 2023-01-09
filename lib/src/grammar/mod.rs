use std::fmt::Display;

#[allow(unused_imports)]
#[allow(dead_code)]
pub(crate) use stringify::{stringify_comfort, stringify_compat, stringify_pretty};

use self::parse::{deep, infix, lexer, postfix, prefix};
use self::repr::Repr;

mod parse;
pub mod repr;
mod stringify;
#[cfg(test)]
mod test;

const PRESERVE_PREFIX: &str = "'";

const SEPARATOR: &str = ",";
const MAP_KV_SEPARATOR: &str = ":";
const LIST_LEFT: &str = "(";
const LIST_RIGHT: &str = ")";
const MAP_LEFT: &str = "{";
const MAP_RIGHT: &str = "}";
const WRAP_LEFT: &str = "[";
const WRAP_RIGHT: &str = "]";

const COMMENT_PREFIX: &str = "#";

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct ParseError {
    pub(crate) msg: String,
}

pub(crate) type ParseResult<T> = Result<T, ParseError>;

impl ParseError {
    pub(crate) fn err<T>(msg: String) -> ParseResult<T> {
        Err(ParseError { msg })
    }
}

pub(crate) fn parse(src: &str) -> ParseResult<Repr> {
    let flat = lexer::parse(src)?;
    let deep = deep::parse(flat)?;
    let prefix = prefix::parse(deep)?;
    let postfix = postfix::parse(prefix)?;
    let infix = infix::parse(postfix)?;
    let repr = parse::repr::parse(infix)?;
    Ok(repr)
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
