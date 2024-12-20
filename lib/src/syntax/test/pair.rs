use crate::syntax::{
    repr::Repr,
    test::{
        ask,
        call,
        call_list,
        infix,
        list,
        map,
        pair,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        pair(symbol("a"), symbol("b")),
        pair(symbol("a"), list(vec![])),
        pair(symbol("a"), pair(symbol("b"), symbol("c"))),
        pair(symbol("a"), call(symbol("b"), symbol("c"))),
        pair(symbol("a"), call_list(symbol("b"), vec![])),
        pair(symbol("a"), ask(symbol("b"), symbol("c"))),
        pair(symbol("a"), infix(symbol("b"), symbol("c"), symbol("d"))),
        pair(list(vec![]), symbol("a")),
        pair(pair(symbol("a"), symbol("b")), symbol("c")),
        pair(call(symbol("a"), symbol("b")), symbol("c")),
        pair(call_list(symbol("a"), vec![]), symbol("b")),
        pair(ask(symbol("a"), symbol("b")), symbol("c")),
        pair(infix(symbol("a"), symbol("b"), symbol("c")), symbol("d")),
        list(vec![pair(symbol("a"), symbol("b"))]),
        map(vec![(symbol("a"), symbol("b"))]),
        map(vec![(pair(symbol("a"), symbol("b")), symbol("c"))]),
    ]
}
