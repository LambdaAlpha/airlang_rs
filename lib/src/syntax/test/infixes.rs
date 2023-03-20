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
        infix(letter("a"), letter("b"), letter("c")),
        infix(list(vec![]), letter("b"), letter("c")),
        infix(pair(letter("a"), letter("b")), letter("c"), letter("d")),
        infix(call(letter("a"), letter("b")), letter("c"), letter("d")),
        infix(ltree(letter("a"), vec![]), letter("b"), letter("c")),
        infix(reverse(letter("a"), letter("b")), letter("c"), letter("d")),
        infix(
            infix(letter("a"), letter("b"), letter("c")),
            letter("d"),
            letter("e"),
        ),
        infix(letter("a"), list(vec![]), letter("b")),
        infix(letter("a"), pair(letter("b"), letter("c")), letter("d")),
        infix(letter("a"), call(letter("b"), letter("c")), letter("d")),
        infix(letter("a"), ltree(letter("b"), vec![]), letter("c")),
        infix(letter("a"), reverse(letter("b"), letter("c")), letter("d")),
        infix(
            letter("a"),
            infix(letter("b"), letter("c"), letter("d")),
            letter("e"),
        ),
        infix(letter("a"), letter("b"), list(vec![])),
        infix(letter("a"), letter("b"), pair(letter("c"), letter("d"))),
        infix(letter("a"), letter("b"), call(letter("c"), letter("d"))),
        infix(letter("a"), letter("b"), ltree(letter("c"), vec![])),
        infix(letter("a"), letter("b"), reverse(letter("c"), letter("d"))),
        infix(
            letter("a"),
            letter("b"),
            infix(letter("c"), letter("d"), letter("e")),
        ),
    ]
}
