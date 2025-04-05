use crate::syntax::{
    repr::Repr,
    test::{
        class,
        list,
        map,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        class(symbol("a")),
        class(class(symbol("a"))),
        class(list(vec![symbol("a"), symbol("b")])),
        list(vec![class(symbol("a"))]),
        map(vec![(class(symbol("a")), symbol("b"))]),
        map(vec![(symbol("a"), class(symbol("b")))]),
    ]
}
