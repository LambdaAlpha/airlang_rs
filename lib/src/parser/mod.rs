use self::lexer::{config::AirLexerConfig, Lexer, Token};
use self::pass::{deep, flat, infix, postfix, prefix, val};
use crate::val::Val;

mod lexer;
mod pass;
mod stringify;
#[cfg(test)]
mod test;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ParseError {
    pub msg: String,
}

pub type ParseResult<T> = Result<T, ParseError>;

impl ParseError {
    pub fn err<T>(msg: String) -> ParseResult<T> {
        Err(ParseError { msg })
    }
}

pub fn parse(src: &str) -> ParseResult<Val> {
    let lexer = Lexer {
        config: AirLexerConfig::new(),
    };
    match lexer.lexing(src) {
        Ok(tokens) => {
            let flat = flat::parse(tokens);
            let deep = deep::parse(flat)?;
            let prefix = prefix::parse(deep)?;
            let postfix = postfix::parse(prefix)?;
            let infix = infix::parse(postfix)?;
            let val = val::parse(infix)?;
            Ok(val)
        }
        Err(e) => ParseError::err(e.msg),
    }
}

#[allow(dead_code)]
pub use stringify::{stringify_comfort, stringify_compat, stringify_pretty};
