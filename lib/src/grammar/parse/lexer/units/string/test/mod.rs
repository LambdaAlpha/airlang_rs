use crate::grammar::lexer::ParseResult;

use super::StringLexer;
use super::super::test::test_unit_lexing;

mod strings;

#[test]
fn test_lexing_strings() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./strings.air"),
        &StringLexer::new(),
        strings::expected,
    )
}
