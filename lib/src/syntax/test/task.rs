use crate::syntax::repr::Repr;
use crate::syntax::test::call;
use crate::syntax::test::ctx_call;
use crate::syntax::test::ctx_solve;
use crate::syntax::test::infix_call;
use crate::syntax::test::infix_solve;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;
use crate::syntax::test::solve;
use crate::syntax::test::symbol;
use crate::syntax::test::unit;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        infix_call(symbol("a"), symbol("b"), symbol("c")),
        call(symbol("a"), symbol("b")),
        call(symbol("b"), symbol("a")),
        call(symbol("a"), unit()),
        infix_call(infix_call(symbol("a"), symbol("b"), symbol("c")), symbol("d"), symbol("e")),
        infix_call(pair(symbol("a"), symbol("c")), symbol("d"), symbol("e")),
        pair(infix_call(symbol("a"), symbol("b"), symbol("c")), symbol("e")),
        infix_call(call(symbol("a"), symbol("b")), symbol("c"), symbol("d")),
        pair(call(symbol("a"), symbol("b")), symbol("d")),
        infix_call(call(symbol("b"), symbol("a")), symbol("c"), symbol("d")),
        pair(call(symbol("b"), symbol("a")), symbol("d")),
        infix_call(call(symbol("a"), unit()), symbol("b"), symbol("c")),
        pair(call(symbol("a"), unit()), symbol("c")),
        call(symbol("d"), infix_call(symbol("a"), symbol("b"), symbol("c"))),
        call(symbol("d"), pair(symbol("a"), symbol("c"))),
        call(symbol("c"), call(symbol("a"), symbol("b"))),
        call(symbol("c"), call(symbol("b"), symbol("a"))),
        call(symbol("b"), call(symbol("a"), unit())),
        infix_call(symbol("a"), symbol("b"), infix_call(symbol("c"), symbol("d"), symbol("e"))),
        pair(symbol("a"), infix_call(symbol("c"), symbol("d"), symbol("e"))),
        infix_call(symbol("a"), symbol("b"), pair(symbol("c"), symbol("e"))),
        call(symbol("a"), infix_call(symbol("b"), symbol("c"), symbol("d"))),
        call(symbol("a"), pair(symbol("b"), symbol("d"))),
        infix_call(symbol("a"), symbol("b"), call(symbol("c"), symbol("d"))),
        pair(symbol("a"), call(symbol("c"), symbol("d"))),
        infix_call(symbol("a"), symbol("b"), call(symbol("d"), symbol("c"))),
        pair(symbol("a"), call(symbol("d"), symbol("c"))),
        infix_call(symbol("a"), symbol("b"), call(symbol("c"), unit())),
        pair(symbol("a"), call(symbol("c"), unit())),
        call(symbol("a"), call(symbol("c"), symbol("b"))),
        call(symbol("a"), call(symbol("b"), symbol("c"))),
        call(symbol("a"), call(symbol("b"), unit())),
        ctx_call(symbol("a"), symbol("b"), symbol("c")),
        ctx_call(symbol("a"), symbol("b"), pair(symbol("c"), symbol("d"))),
        ctx_call(symbol("a"), symbol("b"), infix_call(symbol("c"), symbol("d"), symbol("e"))),
        ctx_call(symbol("a"), symbol("c"), pair(symbol("b"), symbol("d"))),
        ctx_call(symbol("a"), symbol("b"), call(symbol("c"), symbol("d"))),
        ctx_call(symbol("a"), symbol("b"), call(symbol("d"), symbol("c"))),
        ctx_call(symbol("a"), symbol("b"), call(symbol("c"), unit())),
        ctx_call(infix_call(symbol("a"), symbol("b"), symbol("c")), symbol("d"), symbol("e")),
        infix_call(symbol("a"), symbol("b"), ctx_call(symbol("c"), symbol("d"), symbol("e"))),
        ctx_call(call(symbol("a"), symbol("b")), symbol("c"), symbol("d")),
        call(symbol("a"), ctx_call(symbol("b"), symbol("c"), symbol("d"))),
        ctx_call(call(symbol("b"), symbol("a")), symbol("c"), symbol("d")),
        ctx_call(call(symbol("a"), unit()), symbol("c"), symbol("d")),
        infix_call(symbol("a"), symbol("b"), infix_call(symbol("c"), symbol("d"), symbol("e"))),
        pair(symbol("a"), infix_call(symbol("c"), symbol("d"), symbol("e"))),
        ctx_call(symbol("a"), symbol("b"), symbol("c")),
        solve(symbol("a"), symbol("b")),
        solve(symbol("b"), symbol("a")),
        solve(symbol("a"), unit()),
        infix_solve(symbol("a"), symbol("b"), infix_solve(symbol("c"), symbol("d"), symbol("e"))),
        pair(symbol("a"), infix_solve(symbol("c"), symbol("d"), symbol("e"))),
        ctx_solve(symbol("a"), symbol("b"), symbol("c")),
        call(list(vec![symbol("a"), symbol("b")]), symbol("c")),
        call(symbol("a"), list(vec![symbol("b"), symbol("c")])),
        list(vec![call(symbol("a"), symbol("b"))]),
        map(vec![(call(symbol("a"), symbol("b")), symbol("c"))]),
        map(vec![(symbol("a"), call(symbol("b"), symbol("c")))]),
    ]
}
