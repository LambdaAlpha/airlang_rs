mod floats;
mod ints;

use super::{super::test::test_unit_lexing, NumLexer, ParseResult};

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
