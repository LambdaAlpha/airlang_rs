pub mod config;
#[cfg(test)]
mod test;
mod units;

use crate::num::{Float, Integer};
use crate::val::Bytes;
use regex::{Captures, Regex};

pub struct Lexer<T: LexerConfig> {
    pub config: T,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Delimeter,
    Bool(bool),
    Int(Integer),
    Float(Float),
    Symbol(String),
    Letter(String),
    String(String),
    Bytes(Bytes),
}

#[derive(Debug)]
pub struct LexerError {
    pub msg: String,
}

pub trait LexerConfig {
    fn dispatch(&self, c: char) -> Result<&dyn UnitLexer, LexerError>;
}

pub trait UnitLexer {
    fn pattern(&self) -> &Regex;
    fn lexing(&self, captures: &Captures) -> Result<Token, LexerError>;
}

impl<T: LexerConfig> Lexer<T> {
    pub fn lexing(&self, src: &str) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::<Token>::new();
        let mut rest = &src[..];
        while !rest.is_empty() {
            let first = rest.chars().next().unwrap();

            let lexer = self.config.dispatch(first)?;

            let captures = lexer.pattern().captures(rest);
            if captures.is_none() {
                return Err(LexerError {
                    msg: "pattern matching failed".to_owned(),
                });
            }
            let captures = captures.unwrap();

            let m0 = captures.get(0).unwrap();
            rest = &rest[m0.end()..];
            let token = lexer.lexing(&captures)?;
            if !matches!(token, Token::Delimeter) {
                tokens.push(token);
            }
        }
        return Ok(tokens);
    }
}
