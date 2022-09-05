use crate::parser::lexer::{units::test::test_unit_lexing, LexerError};

use super::StringLexer;

mod strings;

#[test]
fn test_lexing_strings() -> Result<(), LexerError> {
    test_unit_lexing(
        include_str!("./strings.air"),
        StringLexer::new(),
        strings::expected,
    )
}
