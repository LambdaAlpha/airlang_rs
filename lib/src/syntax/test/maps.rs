use crate::syntax::{
    repr::Repr,
    test::{
        map,
        positive_decimal_int as int,
        unit,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        map(vec![]),
        map(vec![(int("1"), int("2"))]),
        map(vec![(int("1"), int("2"))]),
        map(vec![(int("1"), int("2")), (int("3"), int("4"))]),
        map(vec![(int("1"), int("2")), (int("3"), int("4"))]),
        map(vec![(map(vec![]), map(vec![]))]),
        map(vec![(
            map(vec![(int("1"), int("2"))]),
            map(vec![(int("3"), int("4"))]),
        )]),
        map(vec![(int("1"), unit())]),
        map(vec![(int("1"), unit()), (int("2"), int("3"))]),
        map(vec![(int("1"), int("1"))]),
        map(vec![(int("1"), int("1")), (int("2"), int("3"))]),
        map(vec![(int("1"), int("2"))]),
        map(vec![(int("1"), int("2")), (int("1"), int("3"))]),
    ]
}
