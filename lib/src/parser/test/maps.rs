use std::str::FromStr;

use crate::num::Integer;
use crate::val::{Map, Val};

pub fn expected() -> Val {
    Val::list(vec![
        Val::map(Map::from([])),
        Val::map(Map::from([(int("1"), int("2"))])),
        Val::map(Map::from([(int("1"), int("2"))])),
        Val::map(Map::from([(int("1"), int("2")), (int("3"), int("4"))])),
        Val::map(Map::from([(int("1"), int("2")), (int("3"), int("4"))])),
        Val::map(Map::from([(
            Val::map(Map::from([])),
            Val::map(Map::from([])),
        )])),
        Val::map(Map::from([(
            Val::map(Map::from([(int("1"), int("2"))])),
            Val::map(Map::from([(int("3"), int("4"))])),
        )])),
    ])
}

fn int(s: &str) -> Val {
    Val::int(Integer::from_str(s).unwrap())
}
