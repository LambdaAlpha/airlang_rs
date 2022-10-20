use std::str::FromStr;

use crate::grammar::repr::{List, Repr};
use crate::num::Integer;

pub(crate) fn expected() -> Repr {
    Repr::from(vec![
        Repr::from(vec![] as List),
        Repr::from(vec![int("1")]),
        Repr::from(vec![int("1")]),
        Repr::from(vec![int("1"), int("2")]),
        Repr::from(vec![int("1"), int("2")]),
        Repr::from(vec![Repr::from(vec![] as List)]),
        Repr::from(vec![Repr::from(vec![] as List), Repr::from(vec![] as List)]),
    ])
}

fn int(s: &str) -> Repr {
    Repr::from(Integer::from_str(s).unwrap())
}
