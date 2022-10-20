use std::str::FromStr;

use crate::grammar::repr::{map, Repr};
use crate::num::Integer;

pub(crate) fn expected() -> Repr {
    Repr::from(vec![
        Repr::from(map::from([])),
        Repr::from(map::from([(int("1"), int("2"))])),
        Repr::from(map::from([(int("1"), int("2"))])),
        Repr::from(map::from([(int("1"), int("2")), (int("3"), int("4"))])),
        Repr::from(map::from([(int("1"), int("2")), (int("3"), int("4"))])),
        Repr::from(map::from([(
            Repr::from(map::from([])),
            Repr::from(map::from([])),
        )])),
        Repr::from(map::from([(
            Repr::from(map::from([(int("1"), int("2"))])),
            Repr::from(map::from([(int("3"), int("4"))])),
        )])),
    ])
}

fn int(s: &str) -> Repr {
    Repr::from(Integer::from_str(s).unwrap())
}
