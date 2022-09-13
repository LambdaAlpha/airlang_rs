mod booleans;
mod bytes;
mod symbols;

use crate::parser::lexer::{units::test::test_unit_lexing, LexerError};

use super::SymbolLexer;

#[test]
fn test_lexing_symbols() -> Result<(), LexerError> {
    test_unit_lexing(
        include_str!("./symbols.air"),
        SymbolLexer::new(),
        symbols::expected,
    )
}

#[test]
fn test_lexing_booleans() -> Result<(), LexerError> {
    test_unit_lexing(
        include_str!("./booleans.air"),
        SymbolLexer::new(),
        booleans::expected,
    )
}

#[test]
fn test_lexing_bytes() -> Result<(), LexerError> {
    test_unit_lexing(
        include_str!("./bytes.air"),
        SymbolLexer::new(),
        bytes::expected,
    )
}
