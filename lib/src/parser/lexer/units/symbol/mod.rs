#[cfg(test)]
mod test;

use regex::Regex;

use crate::parser::lexer::LexResult;
use crate::utils;

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
                (?P<single_punct>[[:punct:]&&[^']])
                |
                (?P<punct_postfix>'[[:punct:]]*_[a-zA-Z0-9_]*[a-zA-Z0-9]+[a-zA-Z0-9_]*)
                |
                (?P<multi_punct>'[[:punct:]]+)
                |
                '(?P<bytes>[0-9a-fA-F]{2}(?:(?:[0-9a-fA-F]{2}|_)*[0-9a-fA-F]{2})?|t|f|)
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
    fn lexing(&self, captures: &regex::Captures) -> LexResult<Token> {
        let single_punct = captures.name("single_punct");
        if single_punct.is_some() {
            return Ok(Token::Symbol(captures.get(0).unwrap().as_str().to_owned()));
        }
        let punct_postfix = captures.name("punct_postfix");
        if punct_postfix.is_some() {
            return Ok(Token::Symbol(captures.get(0).unwrap().as_str().to_owned()));
        }
        let multi_punct = captures.name("multi_punct");
        if multi_punct.is_some() {
            return Ok(Token::Symbol(captures.get(0).unwrap().as_str().to_owned()));
        }
        let token = match captures.name("bytes").unwrap().as_str() {
            "t" => Token::Bool(true),
            "f" => Token::Bool(false),
            hex => {
                let hex = hex.replace("_", "");
                Token::Bytes(utils::conversion::hex_str_to_vec_u8(hex.as_str()).unwrap())
            }
        };
        Ok(token)
    }
    fn stringify(&self, token: &Token, s: &mut String) {
        match token {
            Token::Bool(b) => s.push_str(if *b { "'t" } else { "'f" }),
            Token::Symbol(symbol) => s.push_str(symbol),
            Token::Bytes(bytes) => {
                s.push('\'');
                utils::conversion::u8_array_to_hex_string_mut(bytes, s);
            }
            _ => {}
        }
    }
}
