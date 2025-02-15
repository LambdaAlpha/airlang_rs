use crate::syntax::{
    repr::Repr,
    test::{
        abstract1,
        ask,
        call,
        change,
        infix_abstract,
        infix_ask,
        infix_call,
        infix_change,
        infix_pair,
        list,
        map,
        pair,
        symbol,
        tag_abstract,
        tag_ask,
        tag_call,
        tag_change,
        tag_pair,
    },
};

pub fn expected() -> Vec<Repr> {
    vec![
        symbol("a"),
        symbol("a"),
        infix_call(
            infix_call(
                symbol("a"),
                symbol("b"),
                infix_call(symbol("c"), symbol("d"), symbol("e")),
            ),
            symbol("f"),
            symbol("g"),
        ),
        list(vec![symbol("a")]),
        map(vec![(pair(symbol("a"), symbol("b")), symbol("c"))]),
        map(vec![(symbol("a"), pair(symbol("b"), symbol("c")))]),
        infix_call(
            infix_call(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        ),
        infix_call(
            infix_call(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        ),
        list(vec![infix_call(
            infix_call(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        )]),
        map(vec![(
            infix_call(
                infix_call(symbol("a"), symbol("b"), symbol("c")),
                symbol("d"),
                symbol("e"),
            ),
            infix_call(
                infix_call(symbol("f"), symbol("g"), symbol("h")),
                symbol("i"),
                symbol("j"),
            ),
        )]),
        infix_call(
            symbol("a"),
            symbol("b"),
            infix_call(symbol("c"), symbol("d"), symbol("e")),
        ),
        infix_call(
            symbol("a"),
            symbol("b"),
            infix_call(symbol("c"), symbol("d"), symbol("e")),
        ),
        list(vec![infix_call(
            symbol("a"),
            symbol("b"),
            infix_call(symbol("c"), symbol("d"), symbol("e")),
        )]),
        map(vec![(
            infix_call(
                symbol("a"),
                symbol("b"),
                infix_call(symbol("c"), symbol("d"), symbol("e")),
            ),
            infix_call(
                symbol("f"),
                symbol("g"),
                infix_call(symbol("h"), symbol("i"), symbol("j")),
            ),
        )]),
        infix_pair(
            symbol("a"),
            symbol("b"),
            infix_pair(symbol("c"), symbol("d"), symbol("e")),
        ),
        pair(symbol("a"), symbol("b")),
        infix_call(
            symbol("a"),
            symbol("b"),
            infix_call(symbol("c"), symbol("d"), symbol("e")),
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
        infix_call(
            symbol("a"),
            symbol("b"),
            infix_call(symbol("c"), symbol("d"), symbol("e")),
        ),
        tag_call("t", vec![symbol("a")]),
        tag_call("t", vec![symbol("a")]),
        tag_call("t", vec![tag_call("t", vec![symbol("a")])]),
        tag_call("t", vec![tag_call("t", vec![symbol("a")])]),
        tag_call("a", vec![symbol("b")]),
        tag_call("1", vec![symbol("a")]),
        tag_call("t", vec![symbol("a"), symbol("b")]),
        tag_call("t", vec![
            symbol(":"),
            symbol(";"),
            symbol("!"),
            symbol("?"),
        ]),
        tag_call("t", vec![
            tag_call("t", vec![symbol("a"), symbol("b"), symbol("c")]),
            symbol("d"),
            symbol("e"),
        ]),
        tag_pair("t", vec![symbol("a")]),
        tag_abstract("t", vec![symbol("a")]),
        tag_ask("t", vec![symbol("a")]),
        tag_change("t", vec![symbol("a")]),
        tag_call("t", vec![
            list(vec![
                tag_call("t", vec![symbol("a")]),
                tag_call("t", vec![symbol("b")]),
            ]),
            list(vec![]),
        ]),
        tag_call("t", vec![
            map(vec![(
                tag_call("t", vec![symbol("a")]),
                tag_call("t", vec![symbol("b")]),
            )]),
            map(vec![]),
        ]),
        tag_call("t", vec![map(vec![(
            tag_call("t", vec![symbol("a")]),
            tag_call("t", vec![symbol("b"), symbol("c"), symbol("d")]),
        )])]),
        tag_call("t", vec![map(vec![(
            tag_call("t", vec![symbol("a")]),
            tag_call("t", vec![symbol(":")]),
        )])]),
        tag_call("t", vec![
            symbol("a"),
            call(symbol("b"), symbol("c")),
            symbol("d"),
        ]),
        tag_call("t", vec![infix_call(
            infix_call(symbol("a"), symbol("b"), symbol("c")),
            symbol("d"),
            symbol("e"),
        )]),
        tag_call("t", vec![infix_ask(symbol("a"), symbol("b"), symbol("c"))]),
        infix_call(
            infix_call(symbol("a"), symbol("b"), symbol("c")),
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
        infix_call(
            infix_call(symbol("a"), symbol("b"), symbol("a")),
            symbol("c"),
            infix_call(symbol("a"), symbol("b"), symbol("a")),
        ),
        infix_call(
            infix_call(symbol("a"), symbol("b"), symbol("a")),
            symbol("c"),
            infix_call(symbol("a"), symbol("b"), symbol("a")),
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
