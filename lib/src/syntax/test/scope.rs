use crate::syntax::{
    repr::Repr,
    test::{
        abstract1,
        ask,
        call,
        infix,
        infix_abstract,
        infix_ask,
        infix_pair,
        list,
        map,
        no_compose,
        pair,
        symbol,
    },
};

pub fn expected() -> Vec<Repr> {
    vec![
        symbol("a"),
        symbol("a"),
        infix(
            infix(
                symbol("a"),
                symbol("b"),
                infix(symbol("c"), symbol("d"), symbol("e")),
            ),
            symbol("f"),
            symbol("g"),
        ),
        list(vec![symbol("a")]),
        map(vec![(pair(symbol("a"), symbol("b")), symbol("c"))]),
        map(vec![(symbol("a"), pair(symbol("b"), symbol("c")))]),
        infix(
            infix(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        ),
        infix(
            infix(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        ),
        list(vec![infix(
            infix(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        )]),
        map(vec![(
            infix(
                infix(symbol("a"), symbol("b"), symbol("c")),
                symbol("d"),
                symbol("e"),
            ),
            infix(
                infix(symbol("f"), symbol("g"), symbol("h")),
                symbol("i"),
                symbol("j"),
            ),
        )]),
        infix(
            symbol("a"),
            symbol("b"),
            infix(symbol("c"), symbol("d"), symbol("e")),
        ),
        infix(
            symbol("a"),
            symbol("b"),
            infix(symbol("c"), symbol("d"), symbol("e")),
        ),
        list(vec![infix(
            symbol("a"),
            symbol("b"),
            infix(symbol("c"), symbol("d"), symbol("e")),
        )]),
        map(vec![(
            infix(
                symbol("a"),
                symbol("b"),
                infix(symbol("c"), symbol("d"), symbol("e")),
            ),
            infix(
                symbol("f"),
                symbol("g"),
                infix(symbol("h"), symbol("i"), symbol("j")),
            ),
        )]),
        infix_pair(
            symbol("a"),
            symbol("b"),
            infix_pair(symbol("c"), symbol("d"), symbol("e")),
        ),
        pair(symbol("a"), symbol("b")),
        infix(
            symbol("a"),
            symbol("b"),
            infix(symbol("c"), symbol("d"), symbol("e")),
        ),
        call(symbol("a"), symbol("b")),
        infix_abstract(
            symbol("a"),
            symbol("b"),
            infix_abstract(symbol("c"), symbol("d"), symbol("e")),
        ),
        abstract1(symbol("a"), symbol("b")),
        infix_ask(
            symbol("a"),
            symbol("b"),
            infix_ask(symbol("c"), symbol("d"), symbol("e")),
        ),
        ask(symbol("a"), symbol("b")),
        call(symbol("a"), call(symbol("b"), symbol("c"))),
        call(
            symbol("a"),
            call(symbol("b"), call(symbol("c"), symbol("d"))),
        ),
        call(call(symbol("a"), symbol("b")), symbol("c")),
        ask(symbol("a"), ask(symbol("b"), symbol("c"))),
        infix(
            symbol("a"),
            symbol("b"),
            infix(symbol("c"), symbol("d"), symbol("e")),
        ),
        list(vec![]),
        list(vec![symbol("a")]),
        list(vec![symbol("a"), symbol("b")]),
        list(vec![symbol(":"), symbol(";"), symbol("!"), symbol("?")]),
        symbol("a"),
        symbol("a"),
        no_compose(vec![symbol("a"), symbol("b")]),
        no_compose(vec![symbol(":"), symbol(";"), symbol("!"), symbol("?")]),
        no_compose(vec![
            no_compose(vec![symbol("a"), symbol("b"), symbol("c")]),
            symbol("d"),
            symbol("e"),
        ]),
        no_compose(vec![list(vec![symbol("a"), symbol("b")]), list(vec![])]),
        no_compose(vec![map(vec![(symbol("a"), symbol("b"))]), map(vec![])]),
        map(vec![(
            symbol("a"),
            no_compose(vec![symbol("b"), symbol("c"), symbol("d")]),
        )]),
        map(vec![(symbol("a"), symbol(":"))]),
        no_compose(vec![
            symbol("a"),
            call(symbol("b"), symbol("c")),
            symbol("d"),
        ]),
        infix(
            infix(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        ),
        infix_ask(symbol("a"), symbol("b"), symbol("c")),
        infix_abstract(
            infix_abstract(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        ),
        call(
            symbol("a"),
            call(
                symbol("b"),
                call(symbol("c"), call(symbol("d"), symbol("e"))),
            ),
        ),
        infix_ask(
            infix_ask(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        ),
        pair(symbol("a"), symbol("a")),
        call(symbol("a"), symbol("a")),
        abstract1(symbol("a"), symbol("a")),
        ask(symbol("a"), symbol("a")),
        call(symbol("a"), symbol("a")),
        infix(
            infix(symbol("a"), symbol("b"), symbol("a")),
            symbol("c"),
            infix(symbol("a"), symbol("b"), symbol("a")),
        ),
        infix(
            infix(symbol("a"), symbol("b"), symbol("a")),
            symbol("c"),
            infix(symbol("a"), symbol("b"), symbol("a")),
        ),
        call(symbol("a"), symbol("a")),
        pair(symbol("a"), symbol("a")),
        call(symbol("a"), call(symbol("b"), symbol("b"))),
        call(
            call(symbol("a"), symbol("a")),
            call(symbol("a"), symbol("a")),
        ),
        call(
            call(symbol("a"), symbol("a")),
            call(symbol("a"), symbol("a")),
        ),
    ]
}
