#[cfg(test)]
mod test;

use regex::Regex;

use super::super::LexerError;
use super::super::Token;
use super::super::UnitLexer;

pub struct SymbolLexer {
    pattern: Regex,
}

impl SymbolLexer {
    pub fn new() -> SymbolLexer {
        SymbolLexer {
            pattern: Regex::new(
                "(?x)
                [[:punct:]]+_[a-zA-Z0-9_]*[a-zA-Z0-9]+[a-zA-Z0-9_]*
                |
                _[a-zA-Z0-9_]*[a-zA-Z0-9]+[a-zA-Z0-9_]*
                |
                [[:punct:]]+
                ",
            )
            .unwrap(),
        }
    }
}

impl UnitLexer for SymbolLexer {
    fn pattern(&self) -> &Regex {
        &self.pattern
    }
    fn lexing(&self, captures: &regex::Captures) -> Result<Token, LexerError> {
        Ok(Token::Symbol(captures.get(0).unwrap().as_str().to_owned()))
    }
}
