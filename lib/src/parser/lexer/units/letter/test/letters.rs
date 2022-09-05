use crate::parser::lexer::Token::{self, *};

pub fn expected() -> Vec<Token> {
    vec![
        Letter("a".to_owned()),
        Letter("Abc".to_owned()),
        Letter("A_BB__CCC".to_owned()),
        Letter("A1B2C3".to_owned()),
    ]
}
