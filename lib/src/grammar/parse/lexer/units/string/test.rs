use crate::grammar::{
    lexer::ParseResult,
    parse::lexer::units::{
        string::StringLexer,
        test::test_unit_lexing,
    },
};

mod strings;

#[test]
fn test_lexing_strings() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./test/strings.air"),
        &StringLexer::new(),
        strings::expected,
    )
}
