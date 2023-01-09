use crate::grammar::repr::{
    Int,
    Repr,
};

pub(crate) fn expected() -> Repr {
    Repr::list(vec![
        Repr::map(vec![]),
        Repr::map(vec![(int("1"), int("2"))]),
        Repr::map(vec![(int("1"), int("2"))]),
        Repr::map(vec![(int("1"), int("2")), (int("3"), int("4"))]),
        Repr::map(vec![(int("1"), int("2")), (int("3"), int("4"))]),
        Repr::map(vec![(Repr::map(vec![]), Repr::map(vec![]))]),
        Repr::map(vec![(
            Repr::map(vec![(int("1"), int("2"))]),
            Repr::map(vec![(int("3"), int("4"))]),
        )]),
    ])
}

fn int(s: &str) -> Repr {
    Repr::int(Int::new(true, 10, s.to_owned()))
}
