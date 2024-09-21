use crate::syntax::{
    repr::Repr,
    test::{
        ask,
        call_list,
        infix,
        infix_ask,
        list,
        map,
        no_compose,
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
        infix(infix(int("1"), int("2"), int("3")), int("4"), int("5")),
        list(vec![infix(
            infix(int("1"), int("2"), int("3")),
            int("4"),
            int("5"),
        )]),
        map(vec![(
            int("0"),
            infix(infix(int("1"), int("2"), int("3")), int("4"), int("5")),
        )]),
        infix(int("1"), int("2"), infix(int("3"), int("4"), int("5"))),
        infix(int("1"), int("2"), infix(int("3"), int("4"), int("5"))),
        list(vec![infix(
            int("1"),
            int("2"),
            infix(int("3"), int("4"), int("5")),
        )]),
        map(vec![(
            int("0"),
            infix(int("1"), int("2"), infix(int("3"), int("4"), int("5"))),
        )]),
        infix(int("1"), int("2"), infix(int("3"), int("4"), int("5"))),
        infix_ask(int("1"), int("2"), infix_ask(int("3"), int("4"), int("5"))),
        ask(int("1"), int("2")),
        infix(infix(int("1"), int("2"), int("3")), int("4"), int("5")),
        infix_ask(infix_ask(int("1"), int("2"), int("3")), int("4"), int("5")),
        infix(int("1"), int("2"), infix(int("3"), int("4"), int("5"))),
        infix_ask(int("1"), int("2"), infix_ask(int("3"), int("4"), int("5"))),
        int("1"),
        int("1"),
        no_compose(vec![int("1"), int("2")]),
        no_compose(vec![symbol("@"), symbol("!"), symbol(":"), symbol("?")]),
        symbol("^"),
        no_compose(vec![
            no_compose(vec![symbol("a"), symbol("b"), symbol("c")]),
            symbol("d"),
            symbol("e"),
        ]),
        no_compose(vec![
            no_compose(vec![symbol("a"), symbol("!"), symbol("b")]),
            symbol("c"),
            symbol("d"),
        ]),
        no_compose(vec![list(vec![int("1"), int("2")]), list(vec![])]),
        no_compose(vec![map(vec![(symbol("a"), symbol("b"))]), map(vec![])]),
        map(vec![(
            int("1"),
            no_compose(vec![int("2"), int("3"), int("4")]),
        )]),
        map(vec![(int("1"), symbol(":"))]),
        no_compose(vec![
            symbol("a"),
            no_compose(vec![symbol("b"), symbol("c")]),
            symbol("d"),
        ]),
        infix(infix(int("1"), int("2"), int("3")), int("4"), int("5")),
        no_compose(vec![int("1"), int("2"), int("3")]),
    ]
}
