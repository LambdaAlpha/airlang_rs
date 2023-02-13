use crate::{
    grammar::test::{
        infix,
        letter,
        list,
        ltree,
        map,
        positive_decimal_int as int,
    },
    repr::Repr,
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
        ltree(letter("b"), vec![]),
        list(vec![]),
        list(vec![int("2"), int("5")]),
        map(vec![]),
        map(vec![(int("2"), int("5"))]),
        int("2"),
        ltree(letter("b"), vec![]),
        infix(letter("b"), letter("d"), letter("f")),
        list(vec![]),
    ]
}
