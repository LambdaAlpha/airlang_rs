use crate::num::{Float, ops::CompleteRound};

use super::super::Token;

pub(crate) fn expected() -> Vec<Token> {
    vec![
        parse("0.0", None),
        parse("+0.0", None),
        parse("-0.0", None),
        parse("0.", None),
        parse("1.", None),
        parse("1e2", None),
        parse("1", Some(54)),
        parse("-123.455666e-123", Some(55)),
        parse("1.111111111111111111111111111111e100", None),
    ]
}

fn parse(s: &str, precision: Option<u32>) -> Token {
    Token::Float(Float::parse(s).unwrap().complete(precision.unwrap_or(53)))
}
