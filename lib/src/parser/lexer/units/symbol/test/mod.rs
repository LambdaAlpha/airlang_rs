mod booleans;
mod bytes;
mod symbols;

use crate::parser::lexer::{
    units::test::{test_token_to_string, test_unit_lexing},
    LexResult,
};

use super::SymbolLexer;

#[test]
fn test_lexing_symbols() -> LexResult<()> {
    test_unit_lexing(
        include_str!("./symbols.air"),
        &SymbolLexer::new(),
        symbols::expected,
    )
}

#[test]
fn test_lexing_booleans() -> LexResult<()> {
    test_unit_lexing(
        include_str!("./booleans.air"),
        &SymbolLexer::new(),
        booleans::expected,
    )
}

#[test]
fn test_lexing_bytes() -> LexResult<()> {
    test_unit_lexing(
        include_str!("./bytes.air"),
        &SymbolLexer::new(),
        bytes::expected,
    )
}

#[test]
fn test_symbols_to_string() -> LexResult<()> {
    test_token_to_string(include_str!("./symbols.air"), &SymbolLexer::new())
}

#[test]
fn test_booleans_to_string() -> LexResult<()> {
    test_token_to_string(include_str!("./booleans.air"), &SymbolLexer::new())
}

#[test]
fn test_bytes_to_string() -> LexResult<()> {
    test_token_to_string(include_str!("./bytes.air"), &SymbolLexer::new())
}
