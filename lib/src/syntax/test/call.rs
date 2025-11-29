use crate::syntax::repr::Repr;
use crate::syntax::test::call;
use crate::syntax::test::infix_call;
use crate::syntax::test::key;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;
use crate::type_::Key;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        infix_call(key("a"), key("b"), key("c")),
        call(key("a"), key("b")),
        call(key("b"), key("a")),
        infix_call(infix_call(key("a"), key("b"), key("c")), key("d"), key("e")),
        infix_call(pair(key("a"), key("c")), key("d"), key("e")),
        pair(infix_call(key("a"), key("b"), key("c")), key("e")),
        infix_call(call(key("a"), key("b")), key("c"), key("d")),
        pair(call(key("a"), key("b")), key("d")),
        infix_call(call(key("b"), key("a")), key("c"), key("d")),
        pair(call(key("b"), key("a")), key("d")),
        call(key("d"), infix_call(key("a"), key("b"), key("c"))),
        call(key("d"), pair(key("a"), key("c"))),
        call(key("c"), call(key("a"), key("b"))),
        call(key("c"), call(key("b"), key("a"))),
        infix_call(key("a"), key("b"), infix_call(key("c"), key("d"), key("e"))),
        pair(key("a"), infix_call(key("c"), key("d"), key("e"))),
        infix_call(key("a"), key("b"), pair(key("c"), key("e"))),
        call(key("a"), infix_call(key("b"), key("c"), key("d"))),
        call(key("a"), pair(key("b"), key("d"))),
        infix_call(key("a"), key("b"), call(key("c"), key("d"))),
        pair(key("a"), call(key("c"), key("d"))),
        infix_call(key("a"), key("b"), call(key("d"), key("c"))),
        pair(key("a"), call(key("d"), key("c"))),
        call(key("a"), call(key("c"), key("b"))),
        call(key("a"), call(key("b"), key("c"))),
        call(list(vec![key("a"), key("b")]), key("c")),
        call(key("a"), list(vec![key("b"), key("c")])),
        list(vec![call(key("a"), key("b"))]),
        map(vec![(Key::from_str_unchecked("a"), call(key("b"), key("c")))]),
        infix_call(key("_"), key("_"), key("_")),
    ]
}
