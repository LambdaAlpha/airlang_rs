use crate::grammar::{
    lexer::ParseResult,
    parse::lexer::units::{
        letter::LetterLexer,
        test::test_unit_lexing,
    },
};

mod letters;

#[test]
fn test_lexing_letters() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./letters.air"),
        &LetterLexer::new(),
        letters::expected,
    )
}
