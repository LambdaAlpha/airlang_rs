use crate::grammar::repr::Int;

use super::Token::{self, *};

pub(crate) fn expected() -> Vec<Token> {
    vec![
        String("abc".to_owned()),
        Letter("abc".to_owned()),
        Symbol("+".to_owned()),
        Symbol("+=".to_owned()),
        Int(Int::new(true, 10, "123".to_owned())),
        Symbol("'".to_owned()),
        Bool(true),
        Bytes(vec![0x11]),
        Symbol("'".to_owned()),
        Symbol(")".to_owned()),
        Symbol("#".to_owned()),
        Letter("abc".to_owned()),
        Symbol("%a".to_owned()),
        Symbol("(".to_owned()),
        Int(Int::new(true, 10, "123".to_owned())),
        Symbol(")".to_owned()),
    ]
}
