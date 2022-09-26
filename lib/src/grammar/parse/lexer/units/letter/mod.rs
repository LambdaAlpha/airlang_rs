#[cfg(test)]
mod test;

use regex::Regex;

use crate::grammar::lexer::ParseResult;

use super::super::Token;
use super::super::UnitLexer;

pub(crate) struct LetterLexer {
    pattern: Regex,
}

impl LetterLexer {
    pub(crate) fn new() -> LetterLexer {
        LetterLexer {
            pattern: Regex::new("[a-zA-Z][a-zA-Z0-9_]*").unwrap(),
        }
    }
}

impl UnitLexer for LetterLexer {
    fn pattern(&self) -> &Regex {
        &self.pattern
    }
    fn lexing(&self, captures: &regex::Captures) -> ParseResult<Token> {
        Ok(Token::Letter(captures.get(0).unwrap().as_str().to_owned()))
    }
}
