use std::str::FromStr;

use crate::num::Integer;
use crate::val::Val;

pub fn expected() -> Val {
    Val::list(vec![
        Val::infix1(int("1"), Val::symbol("&".to_owned()), int("2")),
        Val::infix1(
            Val::infix1(int("1"), Val::symbol("&".to_owned()), int("2")),
            Val::symbol("*".to_owned()),
            int("3"),
        ),
        Val::infix1(int("1"), Val::symbol("&".to_owned()), Val::list(vec![])),
        Val::infix1(Val::list(vec![]), Val::symbol("&".to_owned()), int("1")),
        Val::infix1(
            Val::ltree1(int("1"), vec![]),
            Val::symbol("&".to_owned()),
            Val::ltree1(int("2"), vec![]),
        ),
        Val::infix1(int("1"), Val::letter("a".to_owned()), int("2")),
        Val::infix1(
            Val::infix1(int("1"), Val::letter("a".to_owned()), int("2")),
            Val::letter("b".to_owned()),
            int("3"),
        ),
        Val::infix1(int("1"), Val::letter("a".to_owned()), Val::list(vec![])),
        Val::infix1(Val::list(vec![]), Val::letter("a".to_owned()), int("1")),
        Val::infix1(
            Val::ltree1(int("1"), vec![]),
            Val::letter("a".to_owned()),
            Val::ltree1(int("2"), vec![]),
        ),
        Val::infix1(int("1"), Val::list(vec![]), int("2")),
        Val::infix1(int("1"), int("2"), int("3")),
        Val::infix1(
            int("1"),
            Val::infix1(int("2"), int("3"), int("4")),
            int("5"),
        ),
        Val::infix1(
            Val::infix1(
                int("1"),
                Val::symbol("&".to_owned()),
                Val::infix1(int("2"), Val::symbol("*".to_owned()), int("3")),
            ),
            Val::symbol("%".to_owned()),
            int("4"),
        ),
        Val::infix1(
            Val::infix1(
                Val::symbol("+".to_owned()),
                int("1"),
                Val::symbol("-".to_owned()),
            ),
            int("2"),
            Val::symbol("*".to_owned()),
        ),
        Val::letter("a".to_owned()),
        Val::symbol("&".to_owned()),
        Val::ltree1(
            Val::infix1(int("1"), Val::symbol("&".to_owned()), int("2")),
            vec![],
        ),
        Val::ltree1(Val::letter("a".to_owned()), vec![]),
        Val::list(vec![]),
        Val::infix1(int("1"), Val::symbol("&".to_owned()), int("2")),
    ])
}

fn int(s: &str) -> Val {
    Val::int(Integer::from_str(s).unwrap())
}
