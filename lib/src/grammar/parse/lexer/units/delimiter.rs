use {
    crate::grammar::{
        lexer::ParseResult,
        parse::lexer::{
            Token,
            UnitLexer,
        },
    },
    regex::Regex,
};

pub(crate) struct DelimiterLexer {
    pattern: Regex,
}

impl DelimiterLexer {
    pub(crate) fn new() -> DelimiterLexer {
        DelimiterLexer {
            pattern: Regex::new(r"[ \t\r\n]+").unwrap(),
        }
    }
}

impl UnitLexer for DelimiterLexer {
    fn pattern(&self) -> &Regex {
        &self.pattern
    }
    fn lexing(&self, captures: &regex::Captures) -> ParseResult<Token> {
        Ok(Token::Delimiter(
            captures.get(0).unwrap().as_str().to_owned(),
        ))
    }
}
