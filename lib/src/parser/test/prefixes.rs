use std::str::FromStr;

use crate::num::Integer;

use crate::val::{Map, Val};

pub fn expected() -> Val {
    Val::list(vec![
        Val::list(vec![]),
        Val::list(vec![int("2"), int("5")]),
        Val::map(Map::from([])),
        Val::map(Map::from([(int("2"), int("5")), (int("8"), int("9"))])),
        Val::ltree1(Val::letter("b".to_owned()), vec![]),
        Val::infix1(
            Val::letter("b".to_owned()),
            Val::letter("e".to_owned()),
            Val::letter("g".to_owned()),
        ),
    ])
}

fn int(s: &str) -> Val {
    Val::int(Integer::from_str(s).unwrap())
}
