use crate::{
    repr::Repr,
    syntax::test::{
        call,
        infix,
        letter,
        list,
        ltree,
        map,
        pair,
        reverse,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        pair(letter("a"), letter("b")),
        pair(letter("a"), list(vec![])),
        pair(letter("a"), pair(letter("b"), letter("c"))),
        pair(letter("a"), call(letter("b"), letter("c"))),
        pair(letter("a"), ltree(letter("b"), vec![])),
        pair(letter("a"), reverse(letter("b"), letter("c"))),
        pair(letter("a"), infix(letter("b"), letter("c"), letter("d"))),
        pair(list(vec![]), letter("a")),
        pair(pair(letter("a"), letter("b")), letter("c")),
        pair(call(letter("a"), letter("b")), letter("c")),
        pair(ltree(letter("a"), vec![]), letter("b")),
        pair(reverse(letter("a"), letter("b")), letter("c")),
        pair(infix(letter("a"), letter("b"), letter("c")), letter("d")),
        list(vec![pair(letter("a"), letter("b"))]),
        map(vec![(letter("a"), letter("b"))]),
        map(vec![(pair(letter("a"), letter("b")), letter("c"))]),
    ]
}
