use crate::num::Integer;
use crate::parser::lexer::Token::{self, *};
use std::str::FromStr;

pub fn expected() -> Vec<Token> {
    vec![
        String("abc".to_owned()),
        Letter("abc".to_owned()),
        Symbol("+".to_owned()),
        Int(Integer::from_str("123").unwrap()),
        Int(Integer::from_str("-123").unwrap()),
        Bool(true),
        Symbol("#".to_owned()),
        Letter("abc".to_owned()),
        Symbol("%".to_owned()),
        Letter("a".to_owned()),
        Symbol("(".to_owned()),
        Int(Integer::from_str("123").unwrap()),
        Symbol(")".to_owned()),
    ]
}
