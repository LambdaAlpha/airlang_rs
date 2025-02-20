use crate::syntax::{
    repr::Repr,
    test::{
        ask,
        change,
        infix_call,
        list,
        map,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        change(symbol("a"), symbol("b")),
        change(symbol("a"), symbol("b")),
        change(symbol("a"), change(symbol("b"), symbol("c"))),
        change(symbol("a"), ask(symbol("b"), symbol("c"))),
        ask(symbol("a"), change(symbol("b"), symbol("c"))),
        change(symbol("a"), infix_call(symbol("b"), symbol("c"), symbol("d"))),
        infix_call(symbol("a"), symbol("b"), change(symbol("c"), symbol("d"))),
        change(change(symbol("a"), symbol("b")), symbol("c")),
        change(symbol("a"), change(symbol("b"), symbol("c"))),
        list(vec![change(symbol("a"), symbol("b"))]),
        map(vec![(change(symbol("a"), symbol("b")), symbol("c"))]),
        map(vec![(symbol("a"), change(symbol("b"), symbol("c")))]),
    ]
}
