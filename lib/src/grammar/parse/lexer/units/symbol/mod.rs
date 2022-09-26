#[cfg(test)]
mod test;

use regex::Regex;

use crate::grammar::ParseResult;
use crate::grammar::SYMBOL_PREFIX;
use crate::utils;

use super::super::Token;
use super::super::UnitLexer;

pub(crate) struct SymbolLexer {
    pattern: Regex,
}

impl SymbolLexer {
    pub(crate) fn new() -> SymbolLexer {
        SymbolLexer {
            pattern: Regex::new(&format!(
                "(?x)
                (?P<single_punct>[[:punct:]&&[^{0}]])
                |
                {0}(?P<punct_postfix>[[:punct:]]*_[a-zA-Z0-9_]*[a-zA-Z0-9]+[a-zA-Z0-9_]*)
                |
                {0}(?P<multi_punct>[[:punct:]]+)
                |
                {0}(?P<bytes>[0-9a-fA-F]{{2}}(?:(?:[0-9a-fA-F]{{2}}|_)*[0-9a-fA-F]{{2}})?|n)
                |
                {0}(?P<bool>t|f)
                |
                {0}
                ",
                SYMBOL_PREFIX
            ))
            .unwrap(),
        }
    }
}

impl UnitLexer for SymbolLexer {
    fn pattern(&self) -> &Regex {
        &self.pattern
    }
    fn lexing(&self, captures: &regex::Captures) -> ParseResult<Token> {
        let boolean = captures.name("bool");
        if boolean.is_some() {
            return Ok(Token::Bool(matches!(boolean.unwrap().as_str(), "t")));
        }

        let single_punct = captures.name("single_punct");
        if single_punct.is_some() {
            return Ok(Token::Symbol(single_punct.unwrap().as_str().to_owned()));
        }

        let punct_postfix = captures.name("punct_postfix");
        if punct_postfix.is_some() {
            return Ok(Token::Symbol(punct_postfix.unwrap().as_str().to_owned()));
        }

        let multi_punct = captures.name("multi_punct");
        if multi_punct.is_some() {
            return Ok(Token::Symbol(multi_punct.unwrap().as_str().to_owned()));
        }

        let bytes = captures.name("bytes");
        if bytes.is_some() {
            let bytes = match bytes.unwrap().as_str() {
                "n" => vec![],
                hex => {
                    let hex = hex.replace("_", "");
                    utils::conversion::hex_str_to_vec_u8(hex.as_str()).unwrap()
                }
            };
            return Ok(Token::Bytes(bytes));
        }
        Ok(Token::Symbol(SYMBOL_PREFIX.to_owned()))
    }
}
