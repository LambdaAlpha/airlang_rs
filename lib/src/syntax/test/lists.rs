use crate::syntax::{
    repr::Repr,
    test::{
        call,
        infix,
        list,
        map,
        positive_decimal_int as int,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        list(vec![]),
        list(vec![int("1")]),
        list(vec![int("1")]),
        list(vec![int("1"), int("2")]),
        list(vec![int("1"), int("2")]),
        list(vec![list(vec![])]),
        list(vec![list(vec![]), list(vec![])]),
        list(vec![]),
        list(vec![]),
        list(vec![int("1")]),
        list(vec![int("1")]),
        list(vec![int("1"), int("2")]),
        list(vec![symbol("@"), symbol("!"), symbol(":"), symbol("?")]),
        list(vec![symbol("`")]),
        list(vec![
            infix(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        ]),
        list(vec![
            call(symbol("a"), symbol("b")),
            symbol("c"),
            symbol("d"),
        ]),
        list(vec![list(vec![int("1"), int("2")]), list(vec![])]),
        list(vec![map(vec![(symbol("a"), symbol("b"))]), map(vec![])]),
    ]
}
