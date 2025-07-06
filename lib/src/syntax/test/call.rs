use crate::syntax::repr::Repr;
use crate::syntax::test::call;
use crate::syntax::test::infix_call;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;
use crate::syntax::test::reverse;
use crate::syntax::test::symbol;
use crate::syntax::test::unit;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        call(symbol("a"), symbol("b")),
        call(symbol("a"), call(symbol("b"), symbol("c"))),
        call(symbol("a"), pair(symbol("b"), symbol("c"))),
        pair(symbol("a"), call(symbol("b"), symbol("c"))),
        call(symbol("a"), infix_call(symbol("b"), symbol("c"), symbol("d"))),
        infix_call(symbol("a"), symbol("b"), call(symbol("c"), symbol("d"))),
        call(list(vec![symbol("a"), symbol("b")]), symbol("c")),
        call(symbol("a"), list(vec![symbol("b"), symbol("c")])),
        list(vec![call(symbol("a"), symbol("b"))]),
        map(vec![(call(symbol("a"), symbol("b")), symbol("c"))]),
        map(vec![(symbol("a"), call(symbol("b"), symbol("c")))]),
        reverse(symbol("a"), symbol("b")),
        call(symbol("f"), symbol("a")),
        call(symbol("f"), symbol("a")),
        call(symbol("f"), unit()),
    ]
}
