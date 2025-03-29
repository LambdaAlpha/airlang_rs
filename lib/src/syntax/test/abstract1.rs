use crate::syntax::{
    Repr,
    test::{
        abstract1,
        pair,
        symbol,
    },
};

pub fn expected() -> Vec<Repr> {
    vec![abstract1(symbol("a")), abstract1(pair(symbol("a"), symbol("b")))]
}
