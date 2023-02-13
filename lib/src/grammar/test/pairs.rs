use crate::{
    grammar::test::{
        infix,
        letter,
        list,
        ltree,
        map,
        pair,
    },
    repr::Repr,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        pair(letter("a"), letter("b")),
        pair(pair(letter("a"), letter("b")), letter("c")),
        pair(letter("a"), list(vec![])),
        pair(list(vec![]), letter("a")),
        pair(ltree(letter("a"), vec![]), letter("a")),
        ltree(pair(letter("a"), letter("a")), vec![]),
        infix(pair(letter("a"), letter("b")), letter("c"), letter("d")),
        infix(letter("a"), pair(letter("b"), letter("c")), letter("d")),
        infix(letter("a"), letter("b"), pair(letter("c"), letter("d"))),
        infix(
            pair(letter("a"), letter("b")),
            letter("c"),
            pair(letter("d"), letter("e")),
        ),
        infix(
            pair(letter("a"), letter("b")),
            pair(letter("c"), letter("d")),
            pair(letter("e"), letter("f")),
        ),
        list(vec![pair(letter("a"), letter("b"))]),
        map(vec![(letter("a"), letter("b"))]),
        map(vec![(pair(letter("a"), letter("b")), letter("c"))]),
    ]
}
