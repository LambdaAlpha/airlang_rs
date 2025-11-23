use crate::syntax::repr::Repr;
use crate::syntax::test::key;
use crate::syntax::test::list;
use crate::syntax::test::pair;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        list(vec![]),
        list(vec![]),
        list(vec![key("a")]),
        list(vec![key("a")]),
        list(vec![key("a")]),
        list(vec![key("a"), key("b")]),
        list(vec![key("a"), key("b")]),
        list(vec![pair(key("a"), key("b")), key("c")]),
        list(vec![key("a"), key("b")]),
        list(vec![key(":"), key(";"), key("!"), key("?")]),
        list(vec![list(vec![])]),
        list(vec![list(vec![]), list(vec![])]),
    ]
}
