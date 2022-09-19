mod floats;
mod ints;

use crate::parser::lexer::{
    units::test::{test_token_to_string, test_unit_lexing},
    LexResult,
};

use super::NumLexer;

#[test]
fn test_lexing_ints() -> LexResult<()> {
    test_unit_lexing(include_str!("./ints.air"), &NumLexer::new(), ints::expected)
}

#[test]
fn test_lexing_floats() -> LexResult<()> {
    test_unit_lexing(
        include_str!("./floats.air"),
        &NumLexer::new(),
        floats::expected,
    )
}

#[test]
fn test_ints_to_string() -> LexResult<()> {
    test_token_to_string(include_str!("./ints.air"), &NumLexer::new())
}

#[test]
fn test_floats_to_string() -> LexResult<()> {
    test_token_to_string(include_str!("./floats.air"), &NumLexer::new())
}
