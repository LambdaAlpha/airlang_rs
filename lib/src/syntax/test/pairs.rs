use crate::{
    repr::Repr,
    syntax::test::{
        call,
        infix,
        list,
        ltree,
        map,
        pair,
        reverse,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        pair(symbol("a"), symbol("b")),
        pair(symbol("a"), list(vec![])),
        pair(symbol("a"), pair(symbol("b"), symbol("c"))),
        pair(symbol("a"), call(symbol("b"), symbol("c"))),
        pair(symbol("a"), ltree(symbol("b"), vec![])),
        pair(symbol("a"), reverse(symbol("b"), symbol("c"))),
        pair(symbol("a"), infix(symbol("b"), symbol("c"), symbol("d"))),
        pair(list(vec![]), symbol("a")),
        pair(pair(symbol("a"), symbol("b")), symbol("c")),
        pair(call(symbol("a"), symbol("b")), symbol("c")),
        pair(ltree(symbol("a"), vec![]), symbol("b")),
        pair(reverse(symbol("a"), symbol("b")), symbol("c")),
        pair(infix(symbol("a"), symbol("b"), symbol("c")), symbol("d")),
        list(vec![pair(symbol("a"), symbol("b"))]),
        map(vec![(symbol("a"), symbol("b"))]),
        map(vec![(pair(symbol("a"), symbol("b")), symbol("c"))]),
    ]
}
