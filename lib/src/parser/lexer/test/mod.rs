mod tokens;

use super::{config::AirLexerConfig, LexResult, Lexer, Token};

fn lexer() -> Lexer<AirLexerConfig> {
    Lexer {
        config: AirLexerConfig::new(),
    }
}

fn test_lexing<F: Fn() -> Vec<Token>>(src: &str, f: F) -> LexResult<()> {
    let real_tokens = lexer().lexing(src)?;
    assert_eq!(real_tokens, f());
    Ok(())
}

#[test]
fn test_lexing_tokens() -> LexResult<()> {
    let src = include_str!("./tokens.air");
    test_lexing(src, tokens::expected)
}
