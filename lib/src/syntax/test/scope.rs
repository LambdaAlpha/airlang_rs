use crate::syntax::repr::Repr;
use crate::syntax::test::call;
use crate::syntax::test::infix_call;
use crate::syntax::test::infix_reverse;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;
use crate::syntax::test::reverse;
use crate::syntax::test::symbol;
use crate::syntax::test::tag_call;
use crate::syntax::test::tag_reverse;

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
        call(symbol("a"), symbol("b")),
        infix_reverse(
            symbol("a"),
            symbol("b"),
            infix_reverse(symbol("c"), symbol("d"), symbol("e")),
        ),
        reverse(symbol("a"), symbol("b")),
        call(symbol("a"), call(symbol("b"), symbol("c"))),
        call(symbol("a"), call(symbol("b"), call(symbol("c"), symbol("d")))),
        call(call(symbol("a"), symbol("b")), symbol("c")),
        infix_call(symbol("a"), symbol("b"), infix_call(symbol("c"), symbol("d"), symbol("e"))),
        tag_call("t", vec![symbol("a")]),
        tag_call("t", vec![symbol("a")]),
        tag_call("t", vec![tag_call("t", vec![symbol("a")])]),
        tag_call("t", vec![tag_call("t", vec![symbol("a")])]),
        tag_call("a", vec![symbol("b")]),
        tag_call("1", vec![symbol("a")]),
        tag_call("t", vec![symbol("a"), symbol("b")]),
        tag_call("t", vec![symbol(":"), symbol(";"), symbol("!"), symbol("?")]),
        tag_call("t", vec![
            tag_call("t", vec![symbol("a"), symbol("b"), symbol("c")]),
            symbol("d"),
            symbol("e"),
        ]),
        tag_reverse("t", vec![symbol("a")]),
        tag_call("t", vec![
            list(vec![tag_call("t", vec![symbol("a")]), tag_call("t", vec![symbol("b")])]),
            list(vec![]),
        ]),
        tag_call("t", vec![
            map(vec![(tag_call("t", vec![symbol("a")]), tag_call("t", vec![symbol("b")]))]),
            map(vec![]),
        ]),
        tag_call("t", vec![map(vec![(
            tag_call("t", vec![symbol("a")]),
            tag_call("t", vec![symbol("b"), symbol("c"), symbol("d")]),
        )])]),
        tag_call("t", vec![symbol("a"), call(symbol("b"), symbol("c")), symbol("d")]),
        tag_call("t", vec![infix_call(
            infix_call(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        )]),
        tag_call("t", vec![infix_reverse(symbol("a"), symbol("b"), symbol("c"))]),
    ]
}
