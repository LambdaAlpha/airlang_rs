use regex::Regex;

use crate::grammar::lexer::ParseResult;

use super::super::Token;
use super::super::UnitLexer;

pub(crate) struct DelimeterLexer {
    pattern: Regex,
}

impl DelimeterLexer {
    pub(crate) fn new() -> DelimeterLexer {
        DelimeterLexer {
            pattern: Regex::new(r"[ \t\r\n]+").unwrap(),
        }
    }
}

impl UnitLexer for DelimeterLexer {
    fn pattern(&self) -> &Regex {
        &self.pattern
    }
    fn lexing(&self, captures: &regex::Captures) -> ParseResult<Token> {
        Ok(Token::Delimeter(
            captures.get(0).unwrap().as_str().to_owned(),
        ))
    }
}
