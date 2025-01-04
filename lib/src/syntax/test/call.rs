use crate::syntax::{
    repr::Repr,
    test::{
        call,
        infix,
        list,
        map,
        pair,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        call(symbol("a"), symbol("b")),
        call(symbol("a"), symbol("b")),
        call(symbol("a"), call(symbol("b"), symbol("c"))),
        call(symbol("a"), pair(symbol("b"), symbol("c"))),
        pair(symbol("a"), call(symbol("b"), symbol("c"))),
        call(symbol("a"), infix(symbol("b"), symbol("c"), symbol("d"))),
        infix(symbol("a"), symbol("b"), call(symbol("c"), symbol("d"))),
        call(list(vec![symbol("a"), symbol("b")]), symbol("c")),
        call(symbol("a"), list(vec![symbol("b"), symbol("c")])),
        list(vec![call(symbol("a"), symbol("b"))]),
        map(vec![(call(symbol("a"), symbol("b")), symbol("c"))]),
        map(vec![(symbol("a"), call(symbol("b"), symbol("c")))]),
    ]
}
