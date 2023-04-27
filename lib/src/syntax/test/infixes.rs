use crate::{
    repr::Repr,
    syntax::test::{
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
        infix(symbol("a"), symbol("b"), symbol("c")),
        infix(list(vec![]), symbol("b"), symbol("c")),
        infix(pair(symbol("a"), symbol("b")), symbol("c"), symbol("d")),
        infix(call(symbol("a"), symbol("b")), symbol("c"), symbol("d")),
        infix(ltree(symbol("a"), vec![]), symbol("b"), symbol("c")),
        infix(reverse(symbol("a"), symbol("b")), symbol("c"), symbol("d")),
        infix(
            infix(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        ),
        infix(symbol("a"), list(vec![]), symbol("b")),
        infix(symbol("a"), pair(symbol("b"), symbol("c")), symbol("d")),
        infix(symbol("a"), call(symbol("b"), symbol("c")), symbol("d")),
        infix(symbol("a"), ltree(symbol("b"), vec![]), symbol("c")),
        infix(symbol("a"), reverse(symbol("b"), symbol("c")), symbol("d")),
        infix(
            symbol("a"),
            infix(symbol("b"), symbol("c"), symbol("d")),
            symbol("e"),
        ),
        infix(symbol("a"), symbol("b"), list(vec![])),
        infix(symbol("a"), symbol("b"), pair(symbol("c"), symbol("d"))),
        infix(symbol("a"), symbol("b"), call(symbol("c"), symbol("d"))),
        infix(symbol("a"), symbol("b"), ltree(symbol("c"), vec![])),
        infix(symbol("a"), symbol("b"), reverse(symbol("c"), symbol("d"))),
        infix(
            symbol("a"),
            symbol("b"),
            infix(symbol("c"), symbol("d"), symbol("e")),
        ),
    ]
}
