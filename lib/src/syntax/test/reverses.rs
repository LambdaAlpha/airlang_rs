use crate::{
    repr::Repr,
    syntax::test::{
        call,
        infix,
        letter,
        list,
        ltree,
        pair,
        reverse,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        reverse(letter("a"), letter("b")),
        reverse(letter("a"), list(vec![])),
        reverse(letter("a"), pair(letter("b"), letter("c"))),
        reverse(letter("a"), call(letter("b"), letter("c"))),
        reverse(letter("a"), ltree(letter("b"), vec![])),
        reverse(letter("a"), reverse(letter("b"), letter("c"))),
        reverse(letter("a"), infix(letter("b"), letter("c"), letter("d"))),
        reverse(list(vec![]), letter("a")),
        reverse(pair(letter("a"), letter("b")), letter("c")),
        reverse(call(letter("a"), letter("b")), letter("c")),
        reverse(ltree(letter("a"), vec![]), letter("b")),
        reverse(reverse(letter("a"), letter("b")), letter("c")),
        reverse(infix(letter("a"), letter("b"), letter("c")), letter("d")),
    ]
}
