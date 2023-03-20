use crate::{
    repr::Repr,
    syntax::test::{
        call,
        infix,
        letter,
        list,
        ltree,
        map,
        mtree,
        pair,
        reverse,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        call(letter("a"), letter("b")),
        ltree(letter("a"), vec![]),
        mtree(letter("a"), vec![]),
        call(letter("a"), pair(letter("b"), letter("c"))),
        call(letter("a"), call(letter("b"), letter("c"))),
        call(letter("a"), ltree(letter("b"), vec![])),
        call(letter("a"), reverse(letter("b"), letter("c"))),
        call(letter("a"), infix(letter("b"), letter("c"), letter("d"))),
        call(list(vec![]), letter("a")),
        call(map(vec![]), letter("a")),
        call(pair(letter("a"), letter("b")), letter("c")),
        call(call(letter("a"), letter("b")), letter("c")),
        call(ltree(letter("a"), vec![]), letter("b")),
        call(reverse(letter("a"), letter("b")), letter("c")),
        call(infix(letter("a"), letter("b"), letter("c")), letter("d")),
        ltree(letter("a"), vec![ltree(letter("a"), vec![])]),
        ltree(ltree(letter("a"), vec![]), vec![]),
        ltree(list(vec![]), vec![]),
        mtree(
            letter("a"),
            vec![(mtree(letter("b"), vec![]), mtree(letter("c"), vec![]))],
        ),
        mtree(mtree(letter("a"), vec![]), vec![]),
        mtree(map(vec![]), vec![]),
        mtree(ltree(letter("a"), vec![]), vec![]),
        ltree(mtree(letter("a"), vec![]), vec![]),
    ]
}
