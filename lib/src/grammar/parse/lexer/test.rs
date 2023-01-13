use crate::grammar::{
    parse::lexer::Token,
    ParseResult,
};

mod tokens;

fn test_lexing<F: Fn() -> Vec<Token>>(src: &str, f: F) -> ParseResult<()> {
    let real_tokens = super::lexing(src)?;
    assert_eq!(real_tokens, f());
    Ok(())
}

#[test]
fn test_lexing_tokens() -> ParseResult<()> {
    let src = include_str!("./test/tokens.air");
    test_lexing(src, tokens::expected)
}
