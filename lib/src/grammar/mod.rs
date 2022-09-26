use self::parse::{deep, infix, lexer, postfix, prefix, val};
use crate::val::Val;

mod parse;
mod stringify;
#[cfg(test)]
mod test;

const SYMBOL_PREFIX: &str = "'";

const SEPERATOR: &str = ",";
const MAP_KV_SEPERATOR: &str = ":";
const LIST_LEFT: &str = "(";
const LIST_RIGHT: &str = ")";
const MAP_LEFT: &str = "{";
const MAP_RIGHT: &str = "}";
const WRAP_LEFT: &str = "[";
const WRAP_RIGHT: &str = "]";

const COMMENT_PREFIX: &str = "#";

#[cfg_attr(debug_assertions, derive(Debug))]
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

pub(crate) fn parse(src: &str) -> ParseResult<Val> {
    let flat = lexer::parse(src)?;
    let deep = deep::parse(flat)?;
    let prefix = prefix::parse(deep)?;
    let postfix = postfix::parse(prefix)?;
    let infix = infix::parse(postfix)?;
    let val = val::parse(infix)?;
    Ok(val)
}

#[allow(unused_imports)]
#[allow(dead_code)]
pub(crate) use stringify::{stringify_comfort, stringify_compat, stringify_pretty};
