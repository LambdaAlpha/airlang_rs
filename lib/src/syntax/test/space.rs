use crate::syntax::Repr;
use crate::syntax::test::call;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::symbol;

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        symbol("a"),
        symbol("a"),
        symbol("a"),
        call(symbol("a"), symbol("b")),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        call(symbol("a"), symbol("b")),
        symbol("a"),
        symbol("a"),
        list(vec![symbol("a"), symbol("d")]),
        map(vec![(symbol("a"), symbol("b"))]),
        symbol("c"),
        symbol("a"),
        symbol("a"),
    ]
}
