use crate::{
    repr::Repr,
    syntax::test::{
        infix,
        list,
        ltree,
        positive_decimal_int as int,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        infix(int("1"), symbol("&"), int("2")),
        infix(
            infix(int("1"), symbol("&"), int("2")),
            symbol("*"),
            int("3"),
        ),
        infix(int("1"), symbol("&"), list(vec![])),
        infix(list(vec![]), symbol("&"), int("1")),
        infix(
            ltree(int("1"), vec![]),
            symbol("&"),
            ltree(int("2"), vec![]),
        ),
        infix(int("1"), list(vec![]), int("2")),
        infix(
            infix(
                int("1"),
                symbol("&"),
                infix(int("2"), symbol("*"), int("3")),
            ),
            symbol("%"),
            int("4"),
        ),
        infix(int("1"), symbol("&"), int("2")),
    ]
}
