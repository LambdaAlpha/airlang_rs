use crate::parser::lexer::Token::{self, *};

pub fn expected() -> Vec<Token> {
    vec![Bool(false), Bool(true)]
}
