use airlang::syntax::repr::Repr;

use crate::infix_call;
use crate::key;
use crate::list;
use crate::map;
use crate::pair;

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
        map(vec![("a", pair(key("b"), key("c")))]),
        infix_call(infix_call(key("a"), key("b"), key("c")), key("d"), key("e")),
        infix_call(infix_call(key("a"), key("b"), key("c")), key("d"), key("e")),
        list(vec![infix_call(infix_call(key("a"), key("b"), key("c")), key("d"), key("e"))]),
        map(vec![("a", infix_call(infix_call(key("b"), key("c"), key("d")), key("e"), key("f")))]),
        infix_call(pair(key("a"), infix_call(key("b"), key("c"), key("d"))), key("e"), key("f")),
        infix_call(key("a"), key("b"), infix_call(key("c"), key("d"), key("e"))),
        infix_call(key("a"), key("b"), infix_call(key("c"), key("d"), key("e"))),
        list(vec![infix_call(key("a"), key("b"), infix_call(key("c"), key("d"), key("e")))]),
        map(vec![("a", infix_call(key("b"), key("c"), infix_call(key("d"), key("e"), key("f"))))]),
        key("a"),
        key("a"),
    ]
}
