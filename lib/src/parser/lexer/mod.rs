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

#[cfg_attr(debug_assertions, derive(Debug, PartialEq))]
pub enum Token {
    Delimeter(String),
    Bool(bool),
    Int(Integer),
    Float(Float),
    Symbol(String),
    Letter(String),
    String(String),
    Bytes(Bytes),
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct LexError {
    pub msg: String,
}

pub type LexResult<T> = Result<T, LexError>;

impl LexError {
    pub fn err<T>(msg: String) -> LexResult<T> {
        Err(LexError { msg })
    }
}

pub trait LexerConfig {
    fn dispatch_char(&self, c: char) -> LexResult<&dyn UnitLexer>;
    fn dispatch_token(&self, token: &Token) -> &dyn UnitLexer;
}

pub trait UnitLexer {
    fn pattern(&self) -> &Regex;
    fn lexing(&self, captures: &Captures) -> LexResult<Token>;
    fn stringify(&self, token: &Token, s: &mut String);
}

impl<T: LexerConfig> Lexer<T> {
    pub fn lexing(&self, src: &str) -> LexResult<Vec<Token>> {
        let mut tokens = Vec::<Token>::new();
        let mut rest = &src[..];
        while !rest.is_empty() {
            let first = rest.chars().next().unwrap();

            let lexer = self.config.dispatch_char(first)?;

            let captures = lexer.pattern().captures(rest);
            if captures.is_none() {
                return LexError::err("pattern matching failed".to_owned());
            }
            let captures = captures.unwrap();

            let m0 = captures.get(0).unwrap();
            rest = &rest[m0.end()..];
            let token = lexer.lexing(&captures)?;
            if !matches!(token, Token::Delimeter(_)) {
                tokens.push(token);
            }
        }
        return Ok(tokens);
    }

    pub fn stringify_tokens(&self, tokens: &Vec<Token>) -> String {
        let mut s = String::new();
        for token in tokens {
            let lexer = self.config.dispatch_token(&token);
            lexer.stringify(token, &mut s)
        }
        s
    }
}
