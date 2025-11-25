use crate::syntax::repr::Repr;
use crate::syntax::test::infix_call;
use crate::syntax::test::key;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;
use crate::type_::Key;

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
        map(vec![(Key::from_str_unchecked("a"), pair(key("b"), key("c")))]),
        infix_call(infix_call(key("a"), key("b"), key("c")), key("d"), key("e")),
        infix_call(infix_call(key("a"), key("b"), key("c")), key("d"), key("e")),
        list(vec![infix_call(infix_call(key("a"), key("b"), key("c")), key("d"), key("e"))]),
        map(vec![(
            Key::from_str_unchecked("a"),
            infix_call(infix_call(key("b"), key("c"), key("d")), key("e"), key("f")),
        )]),
        infix_call(pair(key("a"), infix_call(key("b"), key("c"), key("d"))), key("e"), key("f")),
        infix_call(key("a"), key("b"), infix_call(key("c"), key("d"), key("e"))),
        infix_call(key("a"), key("b"), infix_call(key("c"), key("d"), key("e"))),
        list(vec![infix_call(key("a"), key("b"), infix_call(key("c"), key("d"), key("e")))]),
        map(vec![(
            Key::from_str_unchecked("a"),
            infix_call(key("b"), key("c"), infix_call(key("d"), key("e"), key("f"))),
        )]),
    ]
}
