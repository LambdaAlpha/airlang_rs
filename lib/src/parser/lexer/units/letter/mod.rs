#[cfg(test)]
mod test;

use regex::Regex;

use super::super::LexerError;
use super::super::Token;
use super::super::UnitLexer;

pub struct LetterLexer {
    pattern: Regex,
}

impl LetterLexer {
    pub fn new() -> LetterLexer {
        LetterLexer {
            pattern: Regex::new("[a-zA-Z][a-zA-Z0-9_]*").unwrap(),
        }
    }
}

impl UnitLexer for LetterLexer {
    fn pattern(&self) -> &Regex {
        &self.pattern
    }
    fn lexing(&self, captures: &regex::Captures) -> Result<Token, LexerError> {
        Ok(Token::Letter(captures.get(0).unwrap().as_str().to_owned()))
    }
}
