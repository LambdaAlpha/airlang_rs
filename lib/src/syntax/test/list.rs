use crate::syntax::repr::Repr;
use crate::syntax::test::list;
use crate::syntax::test::pair;
use crate::syntax::test::symbol;

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        list(vec![]),
        list(vec![]),
        list(vec![symbol("a")]),
        list(vec![symbol("a")]),
        list(vec![symbol("a")]),
        list(vec![symbol("a"), symbol("b")]),
        list(vec![symbol("a"), symbol("b")]),
        list(vec![pair(symbol("a"), symbol("b")), symbol("c")]),
        list(vec![symbol("a"), symbol("b")]),
        list(vec![symbol(":"), symbol(";"), symbol("!"), symbol("?")]),
        list(vec![list(vec![])]),
        list(vec![list(vec![]), list(vec![])]),
    ]
}
