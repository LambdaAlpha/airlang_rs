use crate::grammar::parse::lexer::units::preserve::PreserveLexer;

use super::{ParseResult, super::test::test_unit_lexing, Token};

mod booleans;
mod bytes;
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

#[test]
fn test_lexing_bytes() -> ParseResult<()> {
    test_unit_lexing(
        include_str!("./bytes.air"),
        &PreserveLexer::new(),
        bytes::expected,
    )
}