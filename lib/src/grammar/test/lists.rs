use crate::grammar::repr::{Int, Repr};

pub(crate) fn expected() -> Repr {
    Repr::list(vec![
        Repr::list(vec![]),
        Repr::list(vec![int("1")]),
        Repr::list(vec![int("1")]),
        Repr::list(vec![int("1"), int("2")]),
        Repr::list(vec![int("1"), int("2")]),
        Repr::list(vec![Repr::list(vec![])]),
        Repr::list(vec![Repr::list(vec![]), Repr::list(vec![])]),
    ])
}

fn int(s: &str) -> Repr {
    Repr::int(Int::new(true, 10, s.to_owned()))
}
