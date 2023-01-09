use crate::grammar::parse::lexer::{
    Token,
    Token::Unit,
};

pub(crate) fn expected() -> Vec<Token> {
    vec![Unit]
}
