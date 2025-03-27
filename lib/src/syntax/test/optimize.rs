use crate::syntax::{
    repr::Repr,
    test::{
        infix_call,
        list,
        map,
        optimize,
        pair,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        optimize(symbol("a"), symbol("b")),
        optimize(symbol("a"), symbol("b")),
        optimize(symbol("a"), optimize(symbol("b"), symbol("c"))),
        optimize(symbol("a"), pair(symbol("b"), symbol("c"))),
        pair(symbol("a"), optimize(symbol("b"), symbol("c"))),
        optimize(symbol("a"), infix_call(symbol("b"), symbol("c"), symbol("d"))),
        infix_call(symbol("a"), symbol("b"), optimize(symbol("c"), symbol("d"))),
        optimize(map(vec![(symbol("a"), symbol("b"))]), symbol("c")),
        optimize(symbol("a"), map(vec![(symbol("b"), symbol("c"))])),
        list(vec![optimize(symbol("a"), symbol("b"))]),
        map(vec![(optimize(symbol("a"), symbol("b")), symbol("c"))]),
        map(vec![(symbol("a"), optimize(symbol("b"), symbol("c")))]),
    ]
}
