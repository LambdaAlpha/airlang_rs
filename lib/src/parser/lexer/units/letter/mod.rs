#[cfg(test)]
mod test;

use regex::Regex;

use crate::parser::lexer::LexResult;

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
    fn lexing(&self, captures: &regex::Captures) -> LexResult<Token> {
        Ok(Token::Letter(captures.get(0).unwrap().as_str().to_owned()))
    }
    fn stringify(&self, token: &Token, s: &mut String) {
        match token {
            Token::Letter(l) => s.push_str(&l),
            _ => {}
        }
    }
}
