use crate::syntax::{
    repr::Repr,
    test::{
        inverse,
        list,
        map,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        inverse(symbol("a")),
        inverse(inverse(symbol("a"))),
        inverse(list(vec![symbol("a"), symbol("b")])),
        list(vec![inverse(symbol("a"))]),
        map(vec![(inverse(symbol("a")), symbol("b"))]),
        map(vec![(symbol("a"), inverse(symbol("b")))]),
    ]
}
