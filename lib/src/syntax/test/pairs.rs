use crate::{
    repr::Repr,
    syntax::test::{
        apply,
        infix,
        inverse,
        letter,
        list,
        ltree,
        map,
        pair,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        pair(letter("a"), letter("b")),
        pair(letter("a"), list(vec![])),
        pair(letter("a"), pair(letter("b"), letter("c"))),
        pair(letter("a"), apply(letter("b"), letter("c"))),
        pair(letter("a"), ltree(letter("b"), vec![])),
        pair(letter("a"), inverse(letter("b"), letter("c"))),
        pair(letter("a"), infix(letter("b"), letter("c"), letter("d"))),
        pair(list(vec![]), letter("a")),
        pair(pair(letter("a"), letter("b")), letter("c")),
        pair(apply(letter("a"), letter("b")), letter("c")),
        pair(ltree(letter("a"), vec![]), letter("b")),
        pair(inverse(letter("a"), letter("b")), letter("c")),
        pair(infix(letter("a"), letter("b"), letter("c")), letter("d")),
        list(vec![pair(letter("a"), letter("b"))]),
        map(vec![(letter("a"), letter("b"))]),
        map(vec![(pair(letter("a"), letter("b")), letter("c"))]),
    ]
}
