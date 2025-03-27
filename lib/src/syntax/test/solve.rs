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
        solve(symbol("a"), symbol("b")),
        solve(symbol("a"), symbol("b")),
        solve(symbol("a"), solve(symbol("b"), symbol("c"))),
        solve(symbol("a"), pair(symbol("b"), symbol("c"))),
        pair(symbol("a"), solve(symbol("b"), symbol("c"))),
        solve(symbol("a"), infix_call(symbol("b"), symbol("c"), symbol("d"))),
        infix_call(symbol("a"), symbol("b"), solve(symbol("c"), symbol("d"))),
        solve(list(vec![symbol("a"), symbol("b")]), symbol("c")),
        solve(symbol("a"), list(vec![symbol("b"), symbol("c")])),
        list(vec![solve(symbol("a"), symbol("b"))]),
        map(vec![(solve(symbol("a"), symbol("b")), symbol("c"))]),
        map(vec![(symbol("a"), solve(symbol("b"), symbol("c")))]),
    ]
}
