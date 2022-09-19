use regex::Regex;

use crate::parser::lexer::LexResult;

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
    fn lexing(&self, captures: &regex::Captures) -> LexResult<Token> {
        Ok(Token::Delimeter(
            captures.get(0).unwrap().as_str().to_owned(),
        ))
    }
    fn stringify(&self, token: &Token, s: &mut String) {
        match token {
            Token::Delimeter(d) => s.push_str(&d),
            _ => {}
        }
    }
}
