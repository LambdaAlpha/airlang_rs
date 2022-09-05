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
