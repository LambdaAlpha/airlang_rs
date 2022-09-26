use std::str::FromStr;

use crate::num::Integer;
use crate::val::{List, Val};

pub(crate) fn expected() -> Val {
    Val::from(vec![
        Val::from(vec![] as List),
        Val::from(vec![int("1")]),
        Val::from(vec![int("1")]),
        Val::from(vec![int("1"), int("2")]),
        Val::from(vec![int("1"), int("2")]),
        Val::from(vec![Val::from(vec![] as List)]),
        Val::from(vec![Val::from(vec![] as List), Val::from(vec![] as List)]),
    ])
}

fn int(s: &str) -> Val {
    Val::from(Integer::from_str(s).unwrap())
}
