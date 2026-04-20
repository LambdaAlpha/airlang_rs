use crate::syntax::repr::Repr;
use crate::syntax::test::infix_call;
use crate::syntax::test::key;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;
use crate::syntax::test::solve;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        solve(key("a"), key("b")),
        solve(key("b"), key("a")),
        infix_call(solve(key("a"), key("b")), key("c"), key("d")),
        pair(solve(key("a"), key("b")), key("d")),
        infix_call(solve(key("b"), key("a")), key("c"), key("d")),
        pair(solve(key("b"), key("a")), key("d")),
        solve(key("d"), infix_call(key("a"), key("b"), key("c"))),
        solve(key("d"), pair(key("a"), key("c"))),
        solve(key("c"), solve(key("a"), key("b"))),
        solve(key("c"), solve(key("b"), key("a"))),
        solve(key("a"), infix_call(key("b"), key("c"), key("d"))),
        solve(key("a"), pair(key("b"), key("d"))),
        infix_call(key("a"), key("b"), solve(key("c"), key("d"))),
        pair(key("a"), solve(key("c"), key("d"))),
        infix_call(key("a"), key("b"), solve(key("d"), key("c"))),
        pair(key("a"), solve(key("d"), key("c"))),
        solve(key("a"), solve(key("c"), key("b"))),
        solve(key("a"), solve(key("b"), key("c"))),
        solve(list(vec![key("a"), key("b")]), key("c")),
        solve(key("a"), list(vec![key("b"), key("c")])),
        list(vec![solve(key("a"), key("b"))]),
        map(vec![("a", solve(key("b"), key("c")))]),
        solve(key("?"), key("?")),
    ]
}
