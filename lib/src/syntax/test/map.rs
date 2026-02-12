use crate::syntax::repr::Repr;
use crate::syntax::test::key;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;
use crate::syntax::test::unit;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        map(vec![]),
        map(vec![]),
        map(vec![("a", key("b"))]),
        map(vec![("a", key("b"))]),
        map(vec![("a", key("b"))]),
        map(vec![("a", key("b")), ("c", key("d"))]),
        map(vec![("a", key("b")), ("c", key("d"))]),
        map(vec![("a", key("b")), ("c", pair(key("d"), key("e")))]),
        map(vec![("a", key("b")), ("c", key("d"))]),
        map(vec![("a", map(vec![]))]),
        list(vec![map(vec![]), map(vec![])]),
        map(vec![("a", unit())]),
        map(vec![("a", unit()), ("b", key("c"))]),
        map(vec![("a", pair(key("b"), key("c")))]),
    ]
}
