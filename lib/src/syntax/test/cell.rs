use crate::syntax::repr::Repr;
use crate::syntax::test::cell;
use crate::syntax::test::key;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;
use crate::syntax::test::unit;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        cell(unit()),
        cell(cell(key("a"))),
        cell(pair(key("a"), key("b"))),
        cell(list(vec![key("a"), key("b")])),
        cell(map(vec![("a", key("b"))])),
        pair(key("a"), cell(key("b"))),
        list(vec![cell(key("a"))]),
        map(vec![("a", cell(key("b")))]),
    ]
}
