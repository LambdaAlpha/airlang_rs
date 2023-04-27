use crate::{
    repr::Repr,
    syntax::test::{
        call,
        infix,
        list,
        ltree,
        map,
        mtree,
        pair,
        reverse,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        call(symbol("a"), symbol("b")),
        ltree(symbol("a"), vec![]),
        mtree(symbol("a"), vec![]),
        call(symbol("a"), pair(symbol("b"), symbol("c"))),
        call(symbol("a"), call(symbol("b"), symbol("c"))),
        call(symbol("a"), ltree(symbol("b"), vec![])),
        call(symbol("a"), reverse(symbol("b"), symbol("c"))),
        call(symbol("a"), infix(symbol("b"), symbol("c"), symbol("d"))),
        call(list(vec![]), symbol("a")),
        call(map(vec![]), symbol("a")),
        call(pair(symbol("a"), symbol("b")), symbol("c")),
        call(call(symbol("a"), symbol("b")), symbol("c")),
        call(ltree(symbol("a"), vec![]), symbol("b")),
        call(reverse(symbol("a"), symbol("b")), symbol("c")),
        call(infix(symbol("a"), symbol("b"), symbol("c")), symbol("d")),
        ltree(symbol("a"), vec![ltree(symbol("a"), vec![])]),
        ltree(ltree(symbol("a"), vec![]), vec![]),
        ltree(list(vec![]), vec![]),
        mtree(
            symbol("a"),
            vec![(mtree(symbol("b"), vec![]), mtree(symbol("c"), vec![]))],
        ),
        mtree(mtree(symbol("a"), vec![]), vec![]),
        mtree(map(vec![]), vec![]),
        mtree(ltree(symbol("a"), vec![]), vec![]),
        ltree(mtree(symbol("a"), vec![]), vec![]),
    ]
}
