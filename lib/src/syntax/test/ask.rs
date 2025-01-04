use crate::syntax::{
    repr::Repr,
    test::{
        ask,
        infix,
        list,
        map,
        pair,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        ask(symbol("a"), symbol("b")),
        ask(symbol("a"), symbol("b")),
        ask(symbol("a"), ask(symbol("b"), symbol("c"))),
        ask(symbol("a"), pair(symbol("b"), symbol("c"))),
        pair(symbol("a"), ask(symbol("b"), symbol("c"))),
        ask(symbol("a"), infix(symbol("b"), symbol("c"), symbol("d"))),
        infix(symbol("a"), symbol("b"), ask(symbol("c"), symbol("d"))),
        ask(list(vec![symbol("a"), symbol("b")]), symbol("c")),
        ask(symbol("a"), list(vec![symbol("b"), symbol("c")])),
        list(vec![ask(symbol("a"), symbol("b"))]),
        map(vec![(ask(symbol("a"), symbol("b")), symbol("c"))]),
        map(vec![(symbol("a"), ask(symbol("b"), symbol("c")))]),
    ]
}
