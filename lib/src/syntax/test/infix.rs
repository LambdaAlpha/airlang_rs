use crate::syntax::{
    repr::Repr,
    test::{
        ask,
        call,
        call_list,
        infix,
        list,
        pair,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        infix(symbol("a"), symbol("b"), symbol("c")),
        infix(list(vec![]), symbol("b"), symbol("c")),
        infix(pair(symbol("a"), symbol("b")), symbol("c"), symbol("d")),
        infix(call(symbol("a"), symbol("b")), symbol("c"), symbol("d")),
        infix(call_list(symbol("a"), vec![]), symbol("b"), symbol("c")),
        infix(ask(symbol("a"), symbol("b")), symbol("c"), symbol("d")),
        infix(
            infix(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        ),
        infix(symbol("a"), list(vec![]), symbol("b")),
        infix(symbol("a"), pair(symbol("b"), symbol("c")), symbol("d")),
        infix(symbol("a"), call(symbol("b"), symbol("c")), symbol("d")),
        infix(symbol("a"), call_list(symbol("b"), vec![]), symbol("c")),
        infix(symbol("a"), ask(symbol("b"), symbol("c")), symbol("d")),
        infix(
            symbol("a"),
            infix(symbol("b"), symbol("c"), symbol("d")),
            symbol("e"),
        ),
        infix(symbol("a"), symbol("b"), list(vec![])),
        infix(symbol("a"), symbol("b"), pair(symbol("c"), symbol("d"))),
        infix(symbol("a"), symbol("b"), call(symbol("c"), symbol("d"))),
        infix(symbol("a"), symbol("b"), call_list(symbol("c"), vec![])),
        infix(symbol("a"), symbol("b"), ask(symbol("c"), symbol("d"))),
        infix(
            symbol("a"),
            symbol("b"),
            infix(symbol("c"), symbol("d"), symbol("e")),
        ),
        infix(symbol("a"), symbol("b"), symbol("a")),
    ]
}
