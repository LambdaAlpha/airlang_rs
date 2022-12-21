use regex::Regex;

use crate::grammar::ParseResult;
use crate::grammar::SYMBOL_PREFIX;
use crate::utils;

use super::super::Token;
use super::super::UnitLexer;

#[cfg(test)]
mod test;

pub(crate) struct SymbolLexer {
    pattern: Regex,
}

impl SymbolLexer {
    pub(crate) fn new() -> SymbolLexer {
        SymbolLexer {
            pattern: Regex::new(&format!(
                "(?x)
                {0}(?P<bytes>[0-9a-fA-F]{{2}}(?:(?:[0-9a-fA-F]{{2}}|_)*[0-9a-fA-F]{{2}})?|n)
                |
                {0}(?P<bool>t|f)
                |
                {0}(?P<unit>u)
                |
                (?P<symbol>[{0}()\\[\\]{{}},:]|[[:punct:]&&[^{0}()\\[\\]{{}},:]]([[:punct:]&&[^()\\[\\]{{}},:]]|[a-zA-Z0-9])*)
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
        let unit = captures.name("unit");
        if unit.is_some() {
            return Ok(Token::Unit);
        }

        let boolean = captures.name("bool");
        if boolean.is_some() {
            return Ok(Token::Bool(matches!(boolean.unwrap().as_str(), "t")));
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

        let symbol = captures.name("symbol").unwrap();
        Ok(Token::Symbol(symbol.as_str().to_string()))
    }
}
