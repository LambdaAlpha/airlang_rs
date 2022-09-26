use super::super::test::test_unit_lexing;
use super::StringLexer;
use crate::grammar::lexer::ParseResult;

mod strings;

#[test]
fn test_lexing_strings() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./strings.air"),
        &StringLexer::new(),
        strings::expected,
    )
}
