use std::str::FromStr;

use crate::num::Integer;

use super::Token::{self, *};

pub(crate) fn expected() -> Vec<Token> {
    vec![
        String("abc".to_owned()),
        Letter("abc".to_owned()),
        Symbol("+".to_owned()),
        Int(Integer::from_str("123").unwrap()),
        Int(Integer::from_str("-123").unwrap()),
        Bool(true),
        Symbol("#".to_owned()),
        Letter("abc".to_owned()),
        Symbol("%a".to_owned()),
        Symbol("(".to_owned()),
        Int(Integer::from_str("123").unwrap()),
        Symbol(")".to_owned()),
    ]
}
