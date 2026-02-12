use crate::syntax::repr::Repr;
use crate::syntax::test::infix_call;
use crate::syntax::test::key;
use crate::syntax::test::list;
use crate::syntax::test::map;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        key("a"),
        key("a"),
        key("a"),
        infix_call(key("a"), key("b"), key("c")),
        key("a"),
        key("a"),
        key("a"),
        key("a"),
        key("a"),
        key("a"),
        key("a"),
        infix_call(key("a"), key("b"), key("c")),
        key("a"),
        key("a"),
        list(vec![key("a"), key("d")]),
        map(vec![("a", key("b"))]),
        key("c"),
        key("a"),
    ]
}
