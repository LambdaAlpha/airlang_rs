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
        mtree,
        pair,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        apply(letter("a"), letter("b")),
        ltree(letter("a"), vec![]),
        mtree(letter("a"), vec![]),
        apply(letter("a"), pair(letter("b"), letter("c"))),
        apply(letter("a"), apply(letter("b"), letter("c"))),
        apply(letter("a"), ltree(letter("b"), vec![])),
        apply(letter("a"), inverse(letter("b"), letter("c"))),
        apply(letter("a"), infix(letter("b"), letter("c"), letter("d"))),
        apply(list(vec![]), letter("a")),
        apply(map(vec![]), letter("a")),
        apply(pair(letter("a"), letter("b")), letter("c")),
        apply(apply(letter("a"), letter("b")), letter("c")),
        apply(ltree(letter("a"), vec![]), letter("b")),
        apply(inverse(letter("a"), letter("b")), letter("c")),
        apply(infix(letter("a"), letter("b"), letter("c")), letter("d")),
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
