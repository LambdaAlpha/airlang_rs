use std::str::FromStr;

use crate::num::Integer;
use crate::val::{List, Val};

pub fn expected() -> Val {
    Val::from(vec![
        Val::infix(int("1"), Val::symbol("&".to_owned()), int("2")),
        Val::infix(
            Val::infix(int("1"), Val::symbol("&".to_owned()), int("2")),
            Val::symbol("*".to_owned()),
            int("3"),
        ),
        Val::infix(
            int("1"),
            Val::symbol("&".to_owned()),
            Val::from(vec![] as List),
        ),
        Val::infix(
            Val::from(vec![] as List),
            Val::symbol("&".to_owned()),
            int("1"),
        ),
        Val::infix(
            Val::ltree(int("1"), vec![]),
            Val::symbol("&".to_owned()),
            Val::ltree(int("2"), vec![]),
        ),
        Val::infix(int("1"), Val::letter("a".to_owned()), int("2")),
        Val::infix(
            Val::infix(int("1"), Val::letter("a".to_owned()), int("2")),
            Val::letter("b".to_owned()),
            int("3"),
        ),
        Val::infix(
            int("1"),
            Val::letter("a".to_owned()),
            Val::from(vec![] as List),
        ),
        Val::infix(
            Val::from(vec![] as List),
            Val::letter("a".to_owned()),
            int("1"),
        ),
        Val::infix(
            Val::ltree(int("1"), vec![]),
            Val::letter("a".to_owned()),
            Val::ltree(int("2"), vec![]),
        ),
        Val::infix(int("1"), Val::from(vec![] as List), int("2")),
        Val::infix(int("1"), int("2"), int("3")),
        Val::infix(int("1"), Val::infix(int("2"), int("3"), int("4")), int("5")),
        Val::infix(
            Val::infix(
                int("1"),
                Val::symbol("&".to_owned()),
                Val::infix(int("2"), Val::symbol("*".to_owned()), int("3")),
            ),
            Val::symbol("%".to_owned()),
            int("4"),
        ),
        Val::infix(
            Val::infix(
                Val::symbol("+".to_owned()),
                int("1"),
                Val::symbol("-".to_owned()),
            ),
            int("2"),
            Val::symbol("*".to_owned()),
        ),
        Val::letter("a".to_owned()),
        Val::symbol("&".to_owned()),
        Val::ltree(
            Val::infix(int("1"), Val::symbol("&".to_owned()), int("2")),
            vec![],
        ),
        Val::ltree(Val::letter("a".to_owned()), vec![]),
        Val::from(vec![] as List),
        Val::infix(int("1"), Val::symbol("&".to_owned()), int("2")),
    ])
}

fn int(s: &str) -> Val {
    Val::from(Integer::from_str(s).unwrap())
}
