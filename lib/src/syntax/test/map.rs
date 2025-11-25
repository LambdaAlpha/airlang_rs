use crate::syntax::repr::Repr;
use crate::syntax::test::key;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;
use crate::syntax::test::unit;
use crate::type_::Key;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        map(vec![]),
        map(vec![]),
        map(vec![(Key::from_str_unchecked("a"), key("b"))]),
        map(vec![(Key::from_str_unchecked("a"), key("b"))]),
        map(vec![(Key::from_str_unchecked("a"), key("b"))]),
        map(vec![
            (Key::from_str_unchecked("a"), key("b")),
            (Key::from_str_unchecked("c"), key("d")),
        ]),
        map(vec![
            (Key::from_str_unchecked("a"), key("b")),
            (Key::from_str_unchecked("c"), key("d")),
        ]),
        map(vec![
            (Key::from_str_unchecked("a"), key("b")),
            (Key::from_str_unchecked("c"), pair(key("d"), key("e"))),
        ]),
        map(vec![
            (Key::from_str_unchecked("a"), key("b")),
            (Key::from_str_unchecked("c"), key("d")),
        ]),
        map(vec![(Key::from_str_unchecked("a"), map(vec![]))]),
        list(vec![map(vec![]), map(vec![])]),
        map(vec![(Key::from_str_unchecked("a"), unit())]),
        map(vec![(Key::from_str_unchecked("a"), unit()), (Key::from_str_unchecked("b"), key("c"))]),
        map(vec![(Key::from_str_unchecked("a"), pair(key("b"), key("c")))]),
    ]
}
