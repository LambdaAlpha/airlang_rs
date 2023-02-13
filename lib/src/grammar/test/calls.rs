use crate::{
    grammar::test::{
        call,
        infix,
        letter,
        list,
        ltree,
        map,
        mtree,
        pair,
    },
    Repr,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        call(letter("a"), letter("b")),
        call(infix(letter("a"), letter("b"), letter("c")), letter("d")),
        call(letter("a"), ltree(letter("b"), vec![])),
        call(ltree(letter("a"), vec![]), letter("b")),
        call(letter("a"), pair(letter("b"), letter("c"))),
        call(pair(letter("a"), letter("b")), letter("c")),
        ltree(letter("a"), vec![]),
        ltree(letter("a"), vec![ltree(letter("a"), vec![])]),
        ltree(ltree(letter("a"), vec![]), vec![]),
        ltree(list(vec![]), vec![]),
        mtree(letter("a"), vec![]),
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
