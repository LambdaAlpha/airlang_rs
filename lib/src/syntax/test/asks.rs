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
        ask(symbol("a"), symbol("b")),
        ask(symbol("a"), list(vec![])),
        ask(symbol("a"), pair(symbol("b"), symbol("c"))),
        ask(symbol("a"), call(symbol("b"), symbol("c"))),
        ask(symbol("a"), call_list(symbol("b"), vec![])),
        ask(symbol("a"), ask(symbol("b"), symbol("c"))),
        ask(symbol("a"), infix(symbol("b"), symbol("c"), symbol("d"))),
        ask(list(vec![]), symbol("a")),
        ask(pair(symbol("a"), symbol("b")), symbol("c")),
        ask(call(symbol("a"), symbol("b")), symbol("c")),
        ask(call_list(symbol("a"), vec![]), symbol("b")),
        ask(ask(symbol("a"), symbol("b")), symbol("c")),
        ask(infix(symbol("a"), symbol("b"), symbol("c")), symbol("d")),
        ask(symbol("a"), symbol("a")),
    ]
}
