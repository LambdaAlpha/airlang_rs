use std::str::FromStr;

use crate::grammar::repr::{List, Repr};
use crate::num::Integer;

pub(crate) fn expected() -> Repr {
    Repr::from(vec![
        Repr::infix(int("1"), Repr::symbol("&".to_owned()), int("2")),
        Repr::infix(
            Repr::infix(int("1"), Repr::symbol("&".to_owned()), int("2")),
            Repr::symbol("*".to_owned()),
            int("3"),
        ),
        Repr::infix(
            int("1"),
            Repr::symbol("&".to_owned()),
            Repr::from(vec![] as List),
        ),
        Repr::infix(
            Repr::from(vec![] as List),
            Repr::symbol("&".to_owned()),
            int("1"),
        ),
        Repr::infix(
            Repr::ltree(int("1"), vec![]),
            Repr::symbol("&".to_owned()),
            Repr::ltree(int("2"), vec![]),
        ),
        Repr::infix(int("1"), Repr::letter("a".to_owned()), int("2")),
        Repr::infix(
            Repr::infix(int("1"), Repr::letter("a".to_owned()), int("2")),
            Repr::letter("b".to_owned()),
            int("3"),
        ),
        Repr::infix(
            int("1"),
            Repr::letter("a".to_owned()),
            Repr::from(vec![] as List),
        ),
        Repr::infix(
            Repr::from(vec![] as List),
            Repr::letter("a".to_owned()),
            int("1"),
        ),
        Repr::infix(
            Repr::ltree(int("1"), vec![]),
            Repr::letter("a".to_owned()),
            Repr::ltree(int("2"), vec![]),
        ),
        Repr::infix(int("1"), Repr::from(vec![] as List), int("2")),
        Repr::infix(int("1"), int("2"), int("3")),
        Repr::infix(
            int("1"),
            Repr::infix(int("2"), int("3"), int("4")),
            int("5"),
        ),
        Repr::infix(
            Repr::infix(
                int("1"),
                Repr::symbol("&".to_owned()),
                Repr::infix(int("2"), Repr::symbol("*".to_owned()), int("3")),
            ),
            Repr::symbol("%".to_owned()),
            int("4"),
        ),
        Repr::infix(
            Repr::infix(
                Repr::symbol("+".to_owned()),
                int("1"),
                Repr::symbol("-".to_owned()),
            ),
            int("2"),
            Repr::symbol("*".to_owned()),
        ),
        Repr::letter("a".to_owned()),
        Repr::symbol("&".to_owned()),
        Repr::ltree(
            Repr::infix(int("1"), Repr::symbol("&".to_owned()), int("2")),
            vec![],
        ),
        Repr::ltree(Repr::letter("a".to_owned()), vec![]),
        Repr::from(vec![] as List),
        Repr::infix(int("1"), Repr::symbol("&".to_owned()), int("2")),
    ])
}

fn int(s: &str) -> Repr {
    Repr::from(Integer::from_str(s).unwrap())
}
