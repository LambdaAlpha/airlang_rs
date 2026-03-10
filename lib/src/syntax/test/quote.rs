use crate::syntax::repr::Repr;
use crate::syntax::test::key;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;
use crate::syntax::test::quote;
use crate::syntax::test::text;
use crate::syntax::test::unit;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        quote(key("")),
        quote(text("")),
        quote(list(vec![])),
        quote(map(vec![])),
        quote(unit()),
        quote(quote(key("a"))),
        quote(pair(key("a"), key("b"))),
        quote(list(vec![key("a"), key("b")])),
        quote(map(vec![("a", key("b"))])),
        pair(key("a"), quote(key("b"))),
        list(vec![quote(key("a"))]),
        map(vec![("a", quote(key("b")))]),
    ]
}
