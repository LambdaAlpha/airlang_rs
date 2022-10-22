use crate::grammar::{
    parse::lexer::{Token, UnitLexer, units::delimiter::DelimiterLexer},
    ParseError, ParseResult,
};

pub(crate) fn test_unit_lexing(
    src: &str,
    unit_lexer: &impl UnitLexer,
    expected: impl Fn() -> Vec<Token>,
) -> ParseResult<()> {
    let tokens = unit_lexing(src, unit_lexer)?;
    assert_eq!(tokens, expected());
    Ok(())
}

fn unit_lexing(src: &str, unit_lexer: &impl UnitLexer) -> ParseResult<Vec<Token>> {
    let delimiter_lexer = DelimiterLexer::new();

    let mut tokens = Vec::<Token>::new();
    let mut rest = &src[..];
    while !rest.is_empty() {
        let first = rest.chars().next().unwrap();
        let is_delimiter = matches!(first, ' ' | '\t' | '\r' | '\n');

        let lexer: &dyn UnitLexer = if is_delimiter {
            &delimiter_lexer
        } else {
            unit_lexer
        };

        let captures = lexer.pattern().captures(rest);
        if captures.is_none() {
            return ParseError::err("pattern matching failed".to_owned());
        }
        let captures = captures.unwrap();

        let m0 = captures.get(0).unwrap();
        rest = &rest[m0.end()..];
        if !is_delimiter {
            let token = unit_lexer.lexing(&captures)?;
            tokens.push(token);
        }
    }
    Ok(tokens)
}
