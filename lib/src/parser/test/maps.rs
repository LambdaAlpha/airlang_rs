use std::str::FromStr;

use crate::num::Integer;
use crate::val::{Map, Val};

pub fn expected() -> Val {
    Val::from(vec![
        Val::from(Map::from([])),
        Val::from(Map::from([(int("1"), int("2"))])),
        Val::from(Map::from([(int("1"), int("2"))])),
        Val::from(Map::from([(int("1"), int("2")), (int("3"), int("4"))])),
        Val::from(Map::from([(int("1"), int("2")), (int("3"), int("4"))])),
        Val::from(Map::from([(
            Val::from(Map::from([])),
            Val::from(Map::from([])),
        )])),
        Val::from(Map::from([(
            Val::from(Map::from([(int("1"), int("2"))])),
            Val::from(Map::from([(int("3"), int("4"))])),
        )])),
    ])
}

fn int(s: &str) -> Val {
    Val::from(Integer::from_str(s).unwrap())
}
