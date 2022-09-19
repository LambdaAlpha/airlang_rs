use crate::parser::lexer::{
    units::test::{test_token_to_string, test_unit_lexing},
    LexResult,
};

use super::LetterLexer;

mod letters;

#[test]
fn test_lexing_letters() -> LexResult<()> {
    test_unit_lexing(
        include_str!("./letters.air"),
        &LetterLexer::new(),
        letters::expected,
    )
}

#[test]
fn test_letters_to_string() -> LexResult<()> {
    test_token_to_string(include_str!("./letters.air"), &LetterLexer::new())
}
