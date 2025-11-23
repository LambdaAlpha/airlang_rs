use crate::syntax::repr::Repr;
use crate::syntax::test::infix_call;
use crate::syntax::test::key;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;

pub fn expected() -> Vec<Repr> {
    vec![
        key("a"),
        key("a"),
        infix_call(
            infix_call(key("a"), key("b"), infix_call(key("c"), key("d"), key("e"))),
            key("f"),
            key("g"),
        ),
        list(vec![key("a")]),
        map(vec![(pair(key("a"), key("b")), key("c"))]),
        map(vec![(key("a"), pair(key("b"), key("c")))]),
        infix_call(infix_call(key("a"), key("b"), key("c")), key("d"), key("e")),
        infix_call(infix_call(key("a"), key("b"), key("c")), key("d"), key("e")),
        list(vec![infix_call(infix_call(key("a"), key("b"), key("c")), key("d"), key("e"))]),
        map(vec![(
            infix_call(infix_call(key("a"), key("b"), key("c")), key("d"), key("e")),
            infix_call(infix_call(key("f"), key("g"), key("h")), key("i"), key("j")),
        )]),
        infix_call(pair(key("a"), infix_call(key("b"), key("c"), key("d"))), key("e"), key("f")),
        infix_call(key("a"), key("b"), infix_call(key("c"), key("d"), key("e"))),
        infix_call(key("a"), key("b"), infix_call(key("c"), key("d"), key("e"))),
        list(vec![infix_call(key("a"), key("b"), infix_call(key("c"), key("d"), key("e")))]),
        map(vec![(
            infix_call(key("a"), key("b"), infix_call(key("c"), key("d"), key("e"))),
            infix_call(key("f"), key("g"), infix_call(key("h"), key("i"), key("j"))),
        )]),
    ]
}
