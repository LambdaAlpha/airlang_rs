use crate::parser::lexer::{
    units::delimeter::DelimeterLexer, LexError, LexResult, Token, UnitLexer,
};

pub fn test_unit_lexing(
    src: &str,
    unit_lexer: &impl UnitLexer,
    expected: impl Fn() -> Vec<Token>,
) -> LexResult<()> {
    let tokens = unit_lexing(src, unit_lexer)?;
    assert_eq!(tokens, expected());
    Ok(())
}

pub fn test_token_to_string(src: &str, unit_lexer: &impl UnitLexer) -> LexResult<()> {
    let tokens = unit_lexing(src, unit_lexer)?;
    let mut s = String::new();
    for token in &tokens {
        unit_lexer.stringify(token, &mut s);
        s.push(' ');
    }
    let new_tokens = unit_lexing(&s, unit_lexer)?;
    assert_eq!(tokens, new_tokens);
    Ok(())
}

fn unit_lexing(src: &str, unit_lexer: &impl UnitLexer) -> LexResult<Vec<Token>> {
    let delimeter_lexer = DelimeterLexer::new();

    let mut tokens = Vec::<Token>::new();
    let mut rest = &src[..];
    while !rest.is_empty() {
        let first = rest.chars().next().unwrap();
        let is_delimeter = matches!(first, ' ' | '\t' | '\r' | '\n');

        let lexer: &dyn UnitLexer = if is_delimeter {
            &delimeter_lexer
        } else {
            unit_lexer
        };

        let captures = lexer.pattern().captures(rest);
        if captures.is_none() {
            return LexError::err("pattern matching failed".to_owned());
        }
        let captures = captures.unwrap();

        let m0 = captures.get(0).unwrap();
        rest = &rest[m0.end()..];
        if !is_delimeter {
            let token = unit_lexer.lexing(&captures)?;
            tokens.push(token);
        }
    }
    Ok(tokens)
}
