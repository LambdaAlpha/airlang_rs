use regex::Regex;

use super::super::LexerError;
use super::super::Token;
use super::super::UnitLexer;

pub struct DelimeterLexer {
    pattern: Regex,
}

impl DelimeterLexer {
    pub fn new() -> DelimeterLexer {
        DelimeterLexer {
            pattern: Regex::new(r"[ \t\r\n]+").unwrap(),
        }
    }
}

impl UnitLexer for DelimeterLexer {
    fn pattern(&self) -> &Regex {
        &self.pattern
    }
    fn lexing(&self, _: &regex::Captures) -> Result<Token, LexerError> {
        Ok(Token::Delimeter)
    }
}
