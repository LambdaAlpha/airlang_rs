use crate::syntax::{
    repr::Repr,
    test::{
        infix,
        list,
        ltree,
        positive_decimal_int as int,
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
        ltree(infix(int("1"), int("2"), int("3")), vec![]),
        infix(infix(int("1"), int("2"), int("3")), int("4"), int("5")),
    ]
}
