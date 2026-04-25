use airlang::syntax::repr::Repr;

use crate::key;
use crate::list;
use crate::map;
use crate::pair;
use crate::unit;

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
