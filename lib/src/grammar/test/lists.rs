use crate::{
    grammar::test::{
        list,
        positive_decimal_int as int,
    },
    repr::Repr,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        list(vec![]),
        list(vec![int("1")]),
        list(vec![int("1")]),
        list(vec![int("1"), int("2")]),
        list(vec![int("1"), int("2")]),
        list(vec![list(vec![])]),
        list(vec![list(vec![]), list(vec![])]),
    ]
}
