use crate::syntax::repr::Repr;
use crate::syntax::test::call;
use crate::syntax::test::infix_call;
use crate::syntax::test::key;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        pair(key("a"), key("b")),
        pair(key("a"), pair(key("b"), key("c"))),
        pair(key("a"), call(key("b"), key("c"))),
        call(key("a"), pair(key("b"), key("c"))),
        pair(key("a"), infix_call(key("b"), key("c"), key("d"))),
        infix_call(key("a"), key("b"), pair(key("c"), key("d"))),
        pair(pair(key("a"), key("b")), key("c")),
        pair(key("a"), pair(key("b"), key("c"))),
        list(vec![pair(key("a"), key("b"))]),
        map(vec![("a", pair(key("b"), key("c")))]),
    ]
}
