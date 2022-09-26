use super::Token::{self, *};

pub(crate) fn expected() -> Vec<Token> {
    vec![Bool(false), Bool(true)]
}
