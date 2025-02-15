use crate::syntax::{
    repr::Repr,
    test::{
        list,
        map,
        pair,
        symbol,
        unit,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        map(vec![]),
        map(vec![(symbol("a"), symbol("b"))]),
        map(vec![(symbol("a"), symbol("b"))]),
        map(vec![(symbol("a"), symbol("b")), (symbol("c"), symbol("d"))]),
        map(vec![(symbol("a"), symbol("b")), (symbol("c"), symbol("d"))]),
        map(vec![(map(vec![]), map(vec![]))]),
        list(vec![map(vec![]), map(vec![])]),
        map(vec![(symbol("a"), unit())]),
        map(vec![(symbol("a"), unit()), (symbol("b"), symbol("c"))]),
        map(vec![(symbol("a"), symbol("c"))]),
        map(vec![(pair(symbol("a"), symbol("b")), symbol("c"))]),
        map(vec![(symbol("a"), pair(symbol("b"), symbol("c")))]),
    ]
}
