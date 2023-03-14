use crate::{
    repr::Repr,
    syntax::test::{
        apply,
        infix,
        inverse,
        letter,
        list,
        ltree,
        pair,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        inverse(letter("a"), letter("b")),
        inverse(letter("a"), list(vec![])),
        inverse(letter("a"), pair(letter("b"), letter("c"))),
        inverse(letter("a"), apply(letter("b"), letter("c"))),
        inverse(letter("a"), ltree(letter("b"), vec![])),
        inverse(letter("a"), inverse(letter("b"), letter("c"))),
        inverse(letter("a"), infix(letter("b"), letter("c"), letter("d"))),
        inverse(list(vec![]), letter("a")),
        inverse(pair(letter("a"), letter("b")), letter("c")),
        inverse(apply(letter("a"), letter("b")), letter("c")),
        inverse(ltree(letter("a"), vec![]), letter("b")),
        inverse(inverse(letter("a"), letter("b")), letter("c")),
        inverse(infix(letter("a"), letter("b"), letter("c")), letter("d")),
    ]
}
