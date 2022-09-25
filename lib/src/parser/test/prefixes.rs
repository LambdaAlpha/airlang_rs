use std::str::FromStr;

use crate::num::Integer;

use crate::val::{List, Map, Val};

pub fn expected() -> Val {
    Val::from(vec![
        Val::from(vec![] as List),
        Val::from(vec![int("2"), int("5")]),
        Val::from(Map::from([])),
        Val::from(Map::from([(int("2"), int("5")), (int("8"), int("9"))])),
        Val::ltree(Val::letter("b".to_owned()), vec![]),
        Val::infix(
            Val::letter("b".to_owned()),
            Val::letter("e".to_owned()),
            Val::letter("g".to_owned()),
        ),
    ])
}

fn int(s: &str) -> Val {
    Val::from(Integer::from_str(s).unwrap())
}
