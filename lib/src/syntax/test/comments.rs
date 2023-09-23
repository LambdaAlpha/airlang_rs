use crate::syntax::{
    repr::Repr,
    test::{
        infix,
        list,
        map,
        positive_decimal_int as int,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        int("1"),
        int("2"),
        int("3"),
        int("4"),
        int("5"),
        int("6"),
        int("7"),
        int("8"),
        int("9"),
        int("10"),
        int("11"),
        int("12"),
        int("13"),
        int("14"),
        list(vec![]),
        list(vec![int("2"), int("4")]),
        map(vec![(int("2"), int("4"))]),
        int("2"),
        infix(symbol("d"), symbol("e"), symbol("f")),
        symbol("a"),
    ]
}
