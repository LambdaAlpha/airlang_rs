mod floats;
mod ints;

use crate::parser::lexer::{units::test::test_unit_lexing, LexerError};

use super::NumLexer;

#[test]
fn test_lexing_ints() -> Result<(), LexerError> {
    test_unit_lexing(include_str!("./ints.air"), NumLexer::new(), ints::expected)
}

#[test]
fn test_lexing_floats() -> Result<(), LexerError> {
    test_unit_lexing(
        include_str!("./floats.air"),
        NumLexer::new(),
        floats::expected,
    )
}
