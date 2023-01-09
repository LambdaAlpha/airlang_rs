use crate::grammar::{
    parse::lexer::units::{
        num::NumLexer,
        test::test_unit_lexing,
    },
    ParseResult,
};

mod floats;
mod ints;

#[test]
fn test_lexing_ints() -> ParseResult<()> {
    test_unit_lexing(include_str!("./ints.air"), &NumLexer::new(), ints::expected)
}

#[test]
fn test_lexing_floats() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./floats.air"),
        &NumLexer::new(),
        floats::expected,
    )
}
