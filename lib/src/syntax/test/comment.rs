use crate::syntax::{
    repr::Repr,
    test::{
        bool,
        byte,
        call,
        comment,
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
        comment(unit(), int("1")),
        comment(bool(true), int("2")),
        comment(int("1"), int("3")),
        comment(number(10, "10", 1, "0"), int("4")),
        comment(byte(vec![0x7f]), int("5")),
        comment(symbol("a"), int("6")),
        comment(symbol("%"), int("7")),
        comment(text("s"), int("8")),
        comment(list(vec![int("1")]), int("9")),
        comment(map(vec![(int("1"), int("2"))]), int("10")),
        comment(call(symbol("a"), list(vec![])), int("11")),
        comment(call(symbol("a"), map(vec![])), int("12")),
        comment(infix(symbol("a"), symbol("b"), symbol("c")), int("13")),
        comment(symbol("a"), comment(symbol("b"), int("14"))),
        comment(symbol("a"), list(vec![])),
        list(vec![
            comment(int("1"), int("2")),
            comment(int("3"), int("4")),
        ]),
        map(vec![(
            comment(int("1"), int("2")),
            comment(int("3"), int("4")),
        )]),
        comment(int("1"), int("2")),
        infix(
            comment(infix(symbol("a"), symbol("b"), symbol("c")), symbol("d")),
            symbol("e"),
            symbol("f"),
        ),
        comment(symbol("a"), symbol("a")),
    ]
}
