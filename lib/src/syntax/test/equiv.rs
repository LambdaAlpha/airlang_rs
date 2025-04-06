use crate::syntax::{
    repr::Repr,
    test::{
        equiv,
        list,
        map,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        equiv(symbol("a")),
        equiv(equiv(symbol("a"))),
        equiv(list(vec![symbol("a"), symbol("b")])),
        list(vec![equiv(symbol("a"))]),
        map(vec![(equiv(symbol("a")), symbol("b"))]),
        map(vec![(symbol("a"), equiv(symbol("b")))]),
    ]
}
