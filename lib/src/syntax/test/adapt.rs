use crate::syntax::{
    repr::Repr,
    test::{
        adapt,
        bool,
        byte,
        call,
        infix,
        list,
        map,
        number,
        positive_decimal_int as int,
        symbol,
        text,
        unit,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        adapt(unit(), int("1")),
        adapt(bool(true), int("2")),
        adapt(int("1"), int("3")),
        adapt(number(10, "10", 1, "0"), int("4")),
        adapt(byte(vec![0x7f]), int("5")),
        adapt(symbol("a"), int("6")),
        adapt(symbol("%"), int("7")),
        adapt(text("s"), int("8")),
        adapt(list(vec![int("1")]), int("9")),
        adapt(map(vec![(int("1"), int("2"))]), int("10")),
        adapt(call(symbol("a"), list(vec![])), int("11")),
        adapt(call(symbol("a"), map(vec![])), int("12")),
        adapt(infix(symbol("a"), symbol("b"), symbol("c")), int("13")),
        adapt(symbol("a"), adapt(symbol("b"), int("14"))),
        adapt(symbol("a"), list(vec![])),
        list(vec![adapt(int("1"), int("2")), adapt(int("3"), int("4"))]),
        map(vec![(adapt(int("1"), int("2")), adapt(int("3"), int("4")))]),
        adapt(int("1"), int("2")),
        infix(
            adapt(infix(symbol("a"), symbol("b"), symbol("c")), symbol("d")),
            symbol("e"),
            symbol("f"),
        ),
        adapt(symbol("a"), symbol("a")),
    ]
}
