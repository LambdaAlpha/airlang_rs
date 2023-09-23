use crate::syntax::{
    repr::Repr,
    test::{
        call,
        infix,
        list,
        ltree,
        pair,
        reverse,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        reverse(symbol("a"), symbol("b")),
        reverse(symbol("a"), list(vec![])),
        reverse(symbol("a"), pair(symbol("b"), symbol("c"))),
        reverse(symbol("a"), call(symbol("b"), symbol("c"))),
        reverse(symbol("a"), ltree(symbol("b"), vec![])),
        reverse(symbol("a"), reverse(symbol("b"), symbol("c"))),
        reverse(symbol("a"), infix(symbol("b"), symbol("c"), symbol("d"))),
        reverse(list(vec![]), symbol("a")),
        reverse(pair(symbol("a"), symbol("b")), symbol("c")),
        reverse(call(symbol("a"), symbol("b")), symbol("c")),
        reverse(ltree(symbol("a"), vec![]), symbol("b")),
        reverse(reverse(symbol("a"), symbol("b")), symbol("c")),
        reverse(infix(symbol("a"), symbol("b"), symbol("c")), symbol("d")),
        reverse(symbol("a"), symbol("a")),
    ]
}
