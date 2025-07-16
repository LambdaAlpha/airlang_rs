use crate::syntax::repr::Repr;
use crate::syntax::test::ctx_solve;
use crate::syntax::test::infix_call;
use crate::syntax::test::infix_solve;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;
use crate::syntax::test::symbol;

pub fn expected() -> Vec<Repr> {
    vec![
        symbol("a"),
        symbol("a"),
        infix_call(
            infix_call(symbol("a"), symbol("b"), infix_call(symbol("c"), symbol("d"), symbol("e"))),
            symbol("f"),
            symbol("g"),
        ),
        list(vec![symbol("a")]),
        map(vec![(pair(symbol("a"), symbol("b")), symbol("c"))]),
        map(vec![(symbol("a"), pair(symbol("b"), symbol("c")))]),
        infix_call(infix_call(symbol("a"), symbol("b"), symbol("c")), symbol("d"), symbol("e")),
        infix_call(infix_call(symbol("a"), symbol("b"), symbol("c")), symbol("d"), symbol("e")),
        list(vec![infix_call(
            infix_call(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        )]),
        map(vec![(
            infix_call(infix_call(symbol("a"), symbol("b"), symbol("c")), symbol("d"), symbol("e")),
            infix_call(infix_call(symbol("f"), symbol("g"), symbol("h")), symbol("i"), symbol("j")),
        )]),
        infix_call(
            pair(symbol("a"), infix_call(symbol("b"), symbol("c"), symbol("d"))),
            symbol("e"),
            symbol("f"),
        ),
        infix_call(symbol("a"), symbol("b"), infix_call(symbol("c"), symbol("d"), symbol("e"))),
        infix_call(symbol("a"), symbol("b"), infix_call(symbol("c"), symbol("d"), symbol("e"))),
        list(vec![infix_call(
            symbol("a"),
            symbol("b"),
            infix_call(symbol("c"), symbol("d"), symbol("e")),
        )]),
        map(vec![(
            infix_call(symbol("a"), symbol("b"), infix_call(symbol("c"), symbol("d"), symbol("e"))),
            infix_call(symbol("f"), symbol("g"), infix_call(symbol("h"), symbol("i"), symbol("j"))),
        )]),
        infix_call(symbol("a"), symbol("b"), infix_call(symbol("c"), symbol("d"), symbol("e"))),
        infix_solve(symbol("a"), symbol("b"), infix_solve(symbol("c"), symbol("d"), symbol("e"))),
        infix_solve(symbol("a"), symbol("b"), infix_call(symbol("c"), symbol("d"), symbol("e"))),
        ctx_solve(symbol("a"), symbol("f"), infix_call(symbol("b"), symbol("c"), symbol("d"))),
    ]
}
