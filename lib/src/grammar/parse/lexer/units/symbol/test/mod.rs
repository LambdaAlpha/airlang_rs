use super::{ParseResult, super::test::test_unit_lexing, SymbolLexer, Token};

mod symbols;


#[test]
fn test_lexing_symbols() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./symbols.air"),
        &SymbolLexer::new(),
        symbols::expected,
    )
}


