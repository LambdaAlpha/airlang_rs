use crate::syntax::{
    repr::Repr,
    test::{
        list,
        map,
        solve,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        solve(symbol("a")),
        solve(solve(symbol("a"))),
        solve(list(vec![symbol("a"), symbol("b")])),
        list(vec![solve(symbol("a"))]),
        map(vec![(solve(symbol("a")), symbol("b"))]),
        map(vec![(symbol("a"), solve(symbol("b")))]),
    ]
}
