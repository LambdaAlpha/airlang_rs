#[cfg(test)]
mod test;

use regex::Regex;

use crate::utils;

use super::super::LexerError;
use super::super::Token;
use super::super::UnitLexer;

pub struct BytesLexer {
    pattern: Regex,
}

impl BytesLexer {
    pub fn new() -> BytesLexer {
        BytesLexer {
            pattern: Regex::new(
                r"'(?P<code>[0-9a-fA-F]{2}(?:(?:[0-9a-fA-F]{2}|_)*[0-9a-fA-F]{2})?|t|f|)",
            )
            .unwrap(),
        }
    }
}

impl UnitLexer for BytesLexer {
    fn pattern(&self) -> &Regex {
        &self.pattern
    }
    fn lexing(&self, captures: &regex::Captures) -> Result<Token, LexerError> {
        let token = match captures.name("code").unwrap().as_str() {
            "t" => Token::Bool(true),
            "f" => Token::Bool(false),
            hex => {
                let hex = hex.replace("_", "");
                Token::Bytes(utils::conversion::hex_str_to_vec_u8(hex.as_str()).unwrap())
            }
        };
        Ok(token)
    }
}
