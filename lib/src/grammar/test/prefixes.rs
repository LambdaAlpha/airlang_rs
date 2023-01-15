use crate::grammar::repr::{
    Int,
    Repr,
};

pub(crate) fn expected() -> Repr {
    Repr::list(vec![
        Repr::list(vec![]),
        Repr::list(vec![int("2"), int("5")]),
        Repr::map(vec![]),
        Repr::map(vec![(int("2"), int("5"))]),
        Repr::ltree(Repr::letter("b".to_owned()), vec![]),
        Repr::infix(
            Repr::letter("b".to_owned()),
            Repr::letter("e".to_owned()),
            Repr::letter("g".to_owned()),
        ),
    ])
}

fn int(s: &str) -> Repr {
    Repr::int(Int::new(true, 10, s.to_owned()))
}
