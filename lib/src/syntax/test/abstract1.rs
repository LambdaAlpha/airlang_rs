use crate::syntax::{
    repr::Repr,
    test::{
        abstract1,
        infix_call,
        list,
        map,
        pair,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        abstract1(symbol("a"), symbol("b")),
        abstract1(symbol("a"), symbol("b")),
        abstract1(symbol("a"), abstract1(symbol("b"), symbol("c"))),
        abstract1(symbol("a"), pair(symbol("b"), symbol("c"))),
        pair(symbol("a"), abstract1(symbol("b"), symbol("c"))),
        abstract1(
            symbol("a"),
            infix_call(symbol("b"), symbol("c"), symbol("d")),
        ),
        infix_call(
            symbol("a"),
            symbol("b"),
            abstract1(symbol("c"), symbol("d")),
        ),
        abstract1(map(vec![(symbol("a"), symbol("b"))]), symbol("c")),
        abstract1(symbol("a"), map(vec![(symbol("b"), symbol("c"))])),
        list(vec![abstract1(symbol("a"), symbol("b"))]),
        map(vec![(abstract1(symbol("a"), symbol("b")), symbol("c"))]),
        map(vec![(symbol("a"), abstract1(symbol("b"), symbol("c")))]),
    ]
}
