use crate::grammar::{
    parse::lexer::units::{
        preserve::PreserveLexer,
        test::test_unit_lexing,
    },
    ParseResult,
};

mod booleans;
mod units;

#[test]
fn test_lexing_units() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./units.air"),
        &PreserveLexer::new(),
        units::expected,
    )
}

#[test]
fn test_lexing_booleans() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./booleans.air"),
        &PreserveLexer::new(),
        booleans::expected,
    )
}
