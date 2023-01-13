use {
    crate::grammar::{
        parse::lexer::{
            Token,
            UnitLexer,
        },
        ParseResult,
        PRESERVE_PREFIX,
    },
    regex::Regex,
};

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
                (?P<symbol>[{0}()\\[\\]{{}},:]|[[:punct:]&&[^{0}()\\[\\]{{}},:]]([[:punct:]&&[^()\\[\\]{{}},:]]|[a-zA-Z0-9])*)
                ",
                PRESERVE_PREFIX
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
        let symbol = captures.name("symbol").unwrap();
        Ok(Token::Symbol(symbol.as_str().to_string()))
    }
}
