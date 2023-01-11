use {
    crate::grammar::{
        lexer::ParseResult,
        parse::lexer::{
            Token,
            UnitLexer,
        },
        PRESERVE_PREFIX,
    },
    regex::Regex,
};

#[cfg(test)]
mod test;

pub(crate) struct PreserveLexer {
    pattern: Regex,
}

impl PreserveLexer {
    pub(crate) fn new() -> PreserveLexer {
        PreserveLexer {
            pattern: Regex::new(&format!(
                "(?x)
                {0}(?P<bool>t|f)
                |
                {0}(?P<unit>u)
                ",
                PRESERVE_PREFIX
            ))
            .unwrap(),
        }
    }
}

impl UnitLexer for PreserveLexer {
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

        unreachable!()
    }
}
