use crate::syntax::{
    repr::Repr,
    test::{
        ask,
        call,
        call_list,
        infix,
        infix_ask,
        list,
        map,
        positive_decimal_int as int,
        symbol,
    },
};

pub fn expected() -> Vec<Repr> {
    vec![
        int("1"),
        int("1"),
        list(vec![int("1")]),
        infix(int("1"), int("2"), list(vec![])),
        infix(int("1"), list(vec![]), int("2")),
        infix(
            infix(int("1"), int("2"), infix(int("3"), int("4"), int("5"))),
            int("6"),
            int("7"),
        ),
        call_list(infix(int("1"), int("2"), int("3")), vec![]),
        infix(infix(int("1"), int("2"), int("3")), int("4"), int("5")),
        infix(int("1"), int("2"), infix(int("3"), int("4"), int("5"))),
        infix(int("1"), int("2"), infix(int("3"), int("4"), int("5"))),
        infix_ask(int("1"), int("2"), infix_ask(int("3"), int("4"), int("5"))),
        ask(int("1"), int("2")),
        infix(infix(int("1"), int("2"), int("3")), int("4"), int("5")),
        infix_ask(infix_ask(int("1"), int("2"), int("3")), int("4"), int("5")),
        infix(int("1"), int("2"), infix(int("3"), int("4"), int("5"))),
        infix_ask(int("1"), int("2"), infix_ask(int("3"), int("4"), int("5"))),
        list(vec![]),
        list(vec![]),
        list(vec![int("1")]),
        list(vec![int("1")]),
        list(vec![int("1"), int("2")]),
        list(vec![symbol("@"), symbol("!"), symbol(":"), symbol("?")]),
        list(vec![symbol("^")]),
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
        list(vec![
            symbol("a"),
            list(vec![symbol("b"), symbol("c")]),
            symbol("d"),
        ]),
    ]
}
