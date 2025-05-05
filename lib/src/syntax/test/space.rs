use crate::syntax::{
    Repr,
    test::{
        call,
        list,
        map,
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
        list(vec![symbol("a"), symbol("d")]),
        map(vec![(symbol("a"), symbol("b"))]),
        symbol("c"),
        symbol("a"),
        symbol("a"),
    ]
}
