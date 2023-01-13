use crate::grammar::{
    parse::lexer::units::{
        num::NumLexer,
        test::test_unit_lexing,
    },
    ParseResult,
};

mod bytes;
mod floats;
mod ints;

#[test]
fn test_lexing_ints() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./test/ints.air"),
        &NumLexer::new(),
        ints::expected,
    )
}

#[test]
fn test_lexing_floats() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./test/floats.air"),
        &NumLexer::new(),
        floats::expected,
    )
}

#[test]
fn test_lexing_bytes() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./test/bytes.air"),
        &NumLexer::new(),
        bytes::expected,
    )
}
