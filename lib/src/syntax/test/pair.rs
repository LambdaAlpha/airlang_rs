use crate::syntax::repr::Repr;
use crate::syntax::test::call;
use crate::syntax::test::ctx_call;
use crate::syntax::test::infix_call;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;
use crate::syntax::test::symbol;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        pair(symbol("a"), symbol("b")),
        pair(symbol("a"), pair(symbol("b"), symbol("c"))),
        pair(symbol("a"), call(symbol("b"), symbol("c"))),
        call(symbol("a"), pair(symbol("b"), symbol("c"))),
        pair(symbol("a"), infix_call(symbol("b"), symbol("c"), symbol("d"))),
        infix_call(symbol("a"), symbol("b"), pair(symbol("c"), symbol("d"))),
        pair(symbol("a"), ctx_call(symbol("b"), symbol("c"), symbol("d"))),
        ctx_call(symbol("a"), symbol("b"), pair(symbol("c"), symbol("d"))),
        pair(pair(symbol("a"), symbol("b")), symbol("c")),
        pair(symbol("a"), pair(symbol("b"), symbol("c"))),
        list(vec![pair(symbol("a"), symbol("b"))]),
        map(vec![(pair(symbol("a"), symbol("b")), symbol("c"))]),
        map(vec![(symbol("a"), pair(symbol("b"), symbol("c")))]),
    ]
}
