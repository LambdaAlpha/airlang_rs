use crate::{
    repr::Repr,
    syntax::test::letter,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        letter("a"),
        letter("Abc"),
        letter("A_BB__CCC_"),
        letter("A1B2C3"),
    ]
}
