use self::lexer::{config::AirLexerConfig, Lexer, Token};
use self::pass::{deep, flat, infix, postfix, prefix, val};
use crate::val::Val;

mod lexer;
mod pass;
#[cfg(test)]
mod test;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ParserError {
    pub msg: String,
}

impl ParserError {
    pub fn new(msg: String) -> ParserError {
        ParserError { msg }
    }

    pub fn err<T>(msg: String) -> Result<T, ParserError> {
        Err(ParserError { msg })
    }
}

pub fn parse(src: &str) -> Result<Val, ParserError> {
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
        Err(e) => ParserError::err(e.msg),
    }
}
