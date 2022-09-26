mod booleans;
mod bytes;
mod symbols;

use super::{super::test::test_unit_lexing, ParseResult, SymbolLexer, Token};

#[test]
fn test_lexing_symbols() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./symbols.air"),
        &SymbolLexer::new(),
        symbols::expected,
    )
}

#[test]
fn test_lexing_booleans() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./booleans.air"),
        &SymbolLexer::new(),
        booleans::expected,
    )
}

#[test]
fn test_lexing_bytes() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./bytes.air"),
        &SymbolLexer::new(),
        bytes::expected,
    )
}
