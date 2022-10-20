use super::{ParseResult, super::test::test_unit_lexing, SymbolLexer, Token};

mod booleans;
mod bytes;
mod symbols;
mod units;

#[test]
fn test_lexing_symbols() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./symbols.air"),
        &SymbolLexer::new(),
        symbols::expected,
    )
}

#[test]
fn test_lexing_units() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./units.air"),
        &SymbolLexer::new(),
        units::expected,
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
