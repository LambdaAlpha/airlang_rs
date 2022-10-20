use std::str::FromStr;

use crate::grammar::repr::{List, map, Repr};
use crate::num::Integer;

pub(crate) fn expected() -> Repr {
    Repr::from(vec![
        Repr::from(vec![] as List),
        Repr::from(vec![int("2"), int("5"), int("7")]),
        Repr::from(map::from([])),
        Repr::from(map::from([(int("2"), int("5")), (int("12"), int("13"))])),
        Repr::ltree(Repr::letter("b".to_owned()), vec![]),
        Repr::infix(
            Repr::letter("b".to_owned()),
            Repr::letter("e".to_owned()),
            Repr::letter("g".to_owned()),
        ),
    ])
}

fn int(s: &str) -> Repr {
    Repr::from(Integer::from_str(s).unwrap())
}
