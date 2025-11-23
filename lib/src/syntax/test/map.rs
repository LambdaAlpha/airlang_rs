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
        map(vec![(key("a"), key("b"))]),
        map(vec![(key("a"), key("b"))]),
        map(vec![(key("a"), key("b"))]),
        map(vec![(key("a"), key("b")), (key("c"), key("d"))]),
        map(vec![(key("a"), key("b")), (key("c"), key("d"))]),
        map(vec![(pair(key("a"), key("b")), key("c")), (key("d"), pair(key("e"), key("f")))]),
        map(vec![(key("a"), key("b")), (key("c"), key("d"))]),
        map(vec![(map(vec![]), map(vec![]))]),
        list(vec![map(vec![]), map(vec![])]),
        map(vec![(key("a"), unit())]),
        map(vec![(key("a"), unit()), (key("b"), key("c"))]),
        map(vec![(pair(key("a"), key("b")), key("c"))]),
        map(vec![(key("a"), pair(key("b"), key("c")))]),
    ]
}
