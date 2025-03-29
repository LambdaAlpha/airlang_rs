use crate::syntax::{
    Repr,
    test::{
        call,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        symbol("a"),
        symbol("a"),
        symbol("a"),
        call(symbol("a"), symbol("b")),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        call(symbol("a"), symbol("b")),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        call(symbol("a"), symbol("b")),
    ]
}
