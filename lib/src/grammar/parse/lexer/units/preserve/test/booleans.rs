use crate::grammar::parse::lexer::{
    Token,
    Token::Bool,
};

pub(crate) fn expected() -> Vec<Token> {
    vec![Bool(false), Bool(true)]
}
