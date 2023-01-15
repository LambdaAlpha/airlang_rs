use crate::grammar::repr::Repr;

pub(crate) fn expected() -> Repr {
    Repr::infix(
        Repr::infix(
            Repr::letter("a".to_owned()),
            Repr::symbol("+".to_owned()),
            Repr::ltree(Repr::letter("b".to_owned()), vec![]),
        ),
        Repr::symbol("+".to_owned()),
        Repr::letter("c".to_owned()),
    )
}
