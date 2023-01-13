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

#[cfg(test)]
mod test;

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
