use crate::syntax::{
    repr::Repr,
    test::{
        annotate,
        bool,
        bytes,
        call,
        infix,
        list,
        map,
        number,
        positive_decimal_int as int,
        string,
        symbol,
        unit,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        annotate(unit(), int("1")),
        annotate(bool(true), int("2")),
        annotate(int("1"), int("3")),
        annotate(number(10, "10", 1, "0"), int("4")),
        annotate(bytes(vec![0x7f]), int("5")),
        annotate(symbol("a"), int("6")),
        annotate(symbol("%"), int("7")),
        annotate(string("s"), int("8")),
        annotate(list(vec![int("1")]), int("9")),
        annotate(map(vec![(int("1"), int("2"))]), int("10")),
        annotate(call(symbol("a"), list(vec![])), int("11")),
        annotate(call(symbol("a"), map(vec![])), int("12")),
        annotate(infix(symbol("a"), symbol("b"), symbol("c")), int("13")),
        annotate(symbol("a"), annotate(symbol("b"), int("14"))),
        annotate(symbol("a"), list(vec![])),
        list(vec![
            annotate(int("1"), int("2")),
            annotate(int("3"), int("4")),
        ]),
        map(vec![(
            annotate(int("1"), int("2")),
            annotate(int("3"), int("4")),
        )]),
        annotate(int("1"), int("2")),
        infix(
            annotate(infix(symbol("a"), symbol("b"), symbol("c")), symbol("d")),
            symbol("e"),
            symbol("f"),
        ),
        annotate(symbol("a"), symbol("a")),
    ]
}
