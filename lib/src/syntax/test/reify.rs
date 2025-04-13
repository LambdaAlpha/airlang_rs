use crate::syntax::{
    repr::Repr,
    test::{
        list,
        map,
        reify,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        reify(symbol("a")),
        reify(reify(symbol("a"))),
        reify(list(vec![symbol("a"), symbol("b")])),
        list(vec![reify(symbol("a"))]),
        map(vec![(reify(symbol("a")), symbol("b"))]),
        map(vec![(symbol("a"), reify(symbol("b")))]),
    ]
}
