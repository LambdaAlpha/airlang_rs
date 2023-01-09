use {
    crate::{
        grammar::{
            lexer::ParseResult,
            parse::lexer::{
                Token,
                UnitLexer,
            },
            PRESERVE_PREFIX,
        },
        utils,
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
                {0}(?P<bytes>[0-9a-fA-F]{{2}}(?:(?:[0-9a-fA-F]{{2}}|_)*[0-9a-fA-F]{{2}})?|n)
                |
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

        let bytes = captures.name("bytes");
        let bytes = match bytes.unwrap().as_str() {
            "n" => vec![],
            hex => {
                let hex = hex.replace("_", "");
                utils::conversion::hex_str_to_vec_u8(hex.as_str()).unwrap()
            }
        };
        return Ok(Token::Bytes(bytes));
    }
}
