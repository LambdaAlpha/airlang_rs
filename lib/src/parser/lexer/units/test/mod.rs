use crate::parser::lexer::{units::delimeter::DelimeterLexer, LexerError, Token, UnitLexer};

pub fn test_unit_lexing(
    src: &str,
    unit_lexer: impl UnitLexer,
    expected: impl Fn() -> Vec<Token>,
) -> Result<(), LexerError> {
    let delimeter_lexer = DelimeterLexer::new();

    let mut tokens = Vec::<Token>::new();
    let mut rest = &src[..];
    while !rest.is_empty() {
        let first = rest.chars().next().unwrap();
        let is_delimeter = matches!(first, ' ' | '\t' | '\r' | '\n');

        let lexer: &dyn UnitLexer = if is_delimeter {
            &delimeter_lexer
        } else {
            &unit_lexer
        };

        let captures = lexer.pattern().captures(rest);
        if captures.is_none() {
            return LexerError::err("pattern matching failed".to_owned());
        }
        let captures = captures.unwrap();

        let m0 = captures.get(0).unwrap();
        rest = &rest[m0.end()..];
        if !is_delimeter {
            let token = unit_lexer.lexing(&captures)?;
            tokens.push(token);
        }
    }
    assert_eq!(tokens, expected());
    Ok(())
}
