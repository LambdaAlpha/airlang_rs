mod booleans;
mod bytes;

use crate::parser::lexer::{units::test::test_unit_lexing, LexerError};

use super::BytesLexer;

#[test]
fn test_lexing_booleans() -> Result<(), LexerError> {
    test_unit_lexing(
        include_str!("./booleans.air"),
        BytesLexer::new(),
        booleans::expected,
    )
}

#[test]
fn test_lexing_bytes() -> Result<(), LexerError> {
    test_unit_lexing(
        include_str!("./bytes.air"),
        BytesLexer::new(),
        bytes::expected,
    )
}
