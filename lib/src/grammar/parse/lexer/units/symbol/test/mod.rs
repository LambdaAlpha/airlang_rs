use crate::grammar::{
    parse::lexer::units::{
        symbol::SymbolLexer,
        test::test_unit_lexing,
    },
    ParseResult,
};

mod symbols;

#[test]
fn test_lexing_symbols() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./symbols.air"),
        &SymbolLexer::new(),
        symbols::expected,
    )
}
