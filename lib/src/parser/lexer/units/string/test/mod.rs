use crate::parser::lexer::{
    units::test::{test_token_to_string, test_unit_lexing},
    LexResult,
};

use super::StringLexer;

mod strings;

#[test]
fn test_lexing_strings() -> LexResult<()> {
    test_unit_lexing(
        include_str!("./strings.air"),
        &StringLexer::new(),
        strings::expected,
    )
}

#[test]
fn test_strings_to_string() -> LexResult<()> {
    test_token_to_string(include_str!("./strings.air"), &StringLexer::new())
}
