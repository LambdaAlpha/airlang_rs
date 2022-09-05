use crate::parser::lexer::{units::test::test_unit_lexing, LexerError};

use super::LetterLexer;

mod letters;

#[test]
fn test_lexing_letters() -> Result<(), LexerError> {
    test_unit_lexing(
        include_str!("./letters.air"),
        LetterLexer::new(),
        letters::expected,
    )
}
