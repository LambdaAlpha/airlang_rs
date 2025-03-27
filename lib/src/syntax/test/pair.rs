use crate::syntax::{
    repr::Repr,
    test::{
        infix_call,
        list,
        map,
        pair,
        solve,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        pair(symbol("a"), symbol("b")),
        pair(symbol("a"), symbol("b")),
        pair(symbol("a"), pair(symbol("b"), symbol("c"))),
        pair(symbol("a"), solve(symbol("b"), symbol("c"))),
        solve(symbol("a"), pair(symbol("b"), symbol("c"))),
        pair(symbol("a"), infix_call(symbol("b"), symbol("c"), symbol("d"))),
        infix_call(symbol("a"), symbol("b"), pair(symbol("c"), symbol("d"))),
        pair(pair(symbol("a"), symbol("b")), symbol("c")),
        pair(symbol("a"), pair(symbol("b"), symbol("c"))),
        list(vec![pair(symbol("a"), symbol("b"))]),
        map(vec![(pair(symbol("a"), symbol("b")), symbol("c"))]),
        map(vec![(symbol("a"), pair(symbol("b"), symbol("c")))]),
    ]
}
