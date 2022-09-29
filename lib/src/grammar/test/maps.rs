use std::str::FromStr;

use crate::num::Integer;
use crate::val::{map, Val};

pub(crate) fn expected() -> Val {
    Val::from(vec![
        Val::from(map::from([])),
        Val::from(map::from([(int("1"), int("2"))])),
        Val::from(map::from([(int("1"), int("2"))])),
        Val::from(map::from([(int("1"), int("2")), (int("3"), int("4"))])),
        Val::from(map::from([(int("1"), int("2")), (int("3"), int("4"))])),
        Val::from(map::from([(
            Val::from(map::from([])),
            Val::from(map::from([])),
        )])),
        Val::from(map::from([(
            Val::from(map::from([(int("1"), int("2"))])),
            Val::from(map::from([(int("3"), int("4"))])),
        )])),
    ])
}

fn int(s: &str) -> Val {
    Val::from(Integer::from_str(s).unwrap())
}
