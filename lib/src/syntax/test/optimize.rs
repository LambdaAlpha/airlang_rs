use crate::syntax::{
    repr::Repr,
    test::{
        list,
        map,
        optimize,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        optimize(symbol("a")),
        optimize(optimize(symbol("a"))),
        optimize(list(vec![symbol("a"), symbol("b")])),
        list(vec![optimize(symbol("a"))]),
        map(vec![(optimize(symbol("a")), symbol("b"))]),
        map(vec![(symbol("a"), optimize(symbol("b")))]),
    ]
}
