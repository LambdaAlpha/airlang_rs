use crate::syntax::{
    repr::Repr,
    test::{
        abstract1,
        ask,
        call,
        change,
        infix,
        infix_abstract,
        infix_ask,
        infix_change,
        infix_pair,
        list,
        map,
        pair,
        raw,
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
        infix_change(
            symbol("a"),
            symbol("b"),
            infix_change(symbol("c"), symbol("d"), symbol("e")),
        ),
        change(symbol("a"), symbol("b")),
        call(symbol("a"), call(symbol("b"), symbol("c"))),
        call(
            symbol("a"),
            call(symbol("b"), call(symbol("c"), symbol("d"))),
        ),
        call(call(symbol("a"), symbol("b")), symbol("c")),
        infix(
            symbol("a"),
            symbol("b"),
            infix(symbol("c"), symbol("d"), symbol("e")),
        ),
        raw("r", vec![symbol("a")]),
        raw("r", vec![symbol("a")]),
        raw("r", vec![raw("r", vec![symbol("a")])]),
        raw("r", vec![raw("r", vec![symbol("a")])]),
        raw("a", vec![symbol("b")]),
        raw("1", vec![symbol("a")]),
        raw("r", vec![symbol("a"), symbol("b")]),
        raw("r", vec![
            symbol(":"),
            symbol(";"),
            symbol("!"),
            symbol("?"),
        ]),
        raw("r", vec![
            raw("r", vec![symbol("a"), symbol("b"), symbol("c")]),
            symbol("d"),
            symbol("e"),
        ]),
        raw("r", vec![
            list(vec![
                raw("r", vec![symbol("a")]),
                raw("r", vec![symbol("b")]),
            ]),
            list(vec![]),
        ]),
        raw("r", vec![
            map(vec![(
                raw("r", vec![symbol("a")]),
                raw("r", vec![symbol("b")]),
            )]),
            map(vec![]),
        ]),
        raw("r", vec![map(vec![(
            raw("r", vec![symbol("a")]),
            raw("r", vec![symbol("b"), symbol("c"), symbol("d")]),
        )])]),
        raw("r", vec![map(vec![(
            raw("r", vec![symbol("a")]),
            raw("r", vec![symbol(":")]),
        )])]),
        raw("r", vec![
            symbol("a"),
            call(symbol("b"), symbol("c")),
            symbol("d"),
        ]),
        raw("r", vec![infix(
            infix(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        )]),
        raw("r", vec![infix_ask(symbol("a"), symbol("b"), symbol("c"))]),
        infix(
            infix(symbol("a"), symbol("b"), symbol("c")),
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
