use super::super::test::test_unit_lexing;
use super::LetterLexer;
use crate::grammar::lexer::ParseResult;

mod letters;

#[test]
fn test_lexing_letters() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./letters.air"),
        &LetterLexer::new(),
        letters::expected,
    )
}
