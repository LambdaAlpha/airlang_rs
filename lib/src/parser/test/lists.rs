use std::str::FromStr;

use crate::num::Integer;
use crate::val::Val;

pub fn expected() -> Val {
    Val::list(vec![
        Val::list(vec![]),
        Val::list(vec![int("1")]),
        Val::list(vec![int("1")]),
        Val::list(vec![int("1"), int("2")]),
        Val::list(vec![int("1"), int("2")]),
        Val::list(vec![Val::list(vec![])]),
        Val::list(vec![Val::list(vec![]), Val::list(vec![])]),
    ])
}

fn int(s: &str) -> Val {
    Val::int(Integer::from_str(s).unwrap())
}
