use crate::syntax::repr::Repr;
use crate::syntax::test::infix_call;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::symbol;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        symbol("a"),
        symbol("a"),
        symbol("a"),
        infix_call(symbol("a"), symbol("b"), symbol("c")),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        symbol("a"),
        infix_call(symbol("a"), symbol("b"), symbol("c")),
        symbol("a"),
        symbol("a"),
        list(vec![symbol("a"), symbol("d")]),
        map(vec![(symbol("a"), symbol("b"))]),
        symbol("c"),
        symbol("a"),
        symbol("a"),
    ]
}
