use crate::syntax::{
    repr::Repr,
    test::{
        abstract1,
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
        abstract1(unit(), int("1")),
        abstract1(bool(true), int("2")),
        abstract1(int("1"), int("3")),
        abstract1(number(10, "10", 1, "0"), int("4")),
        abstract1(byte(vec![0x7f]), int("5")),
        abstract1(symbol("a"), int("6")),
        abstract1(symbol("%"), int("7")),
        abstract1(text("s"), int("8")),
        abstract1(list(vec![int("1")]), int("9")),
        abstract1(map(vec![(int("1"), int("2"))]), int("10")),
        abstract1(call(symbol("a"), list(vec![])), int("11")),
        abstract1(call(symbol("a"), map(vec![])), int("12")),
        abstract1(infix(symbol("a"), symbol("b"), symbol("c")), int("13")),
        abstract1(symbol("a"), abstract1(symbol("b"), int("14"))),
        abstract1(symbol("a"), list(vec![])),
        list(vec![
            abstract1(int("1"), int("2")),
            abstract1(int("3"), int("4")),
        ]),
        map(vec![(
            abstract1(int("1"), int("2")),
            abstract1(int("3"), int("4")),
        )]),
        abstract1(int("1"), int("2")),
        infix(
            abstract1(infix(symbol("a"), symbol("b"), symbol("c")), symbol("d")),
            symbol("e"),
            symbol("f"),
        ),
        abstract1(symbol("a"), symbol("a")),
    ]
}
