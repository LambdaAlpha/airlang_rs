use crate::{
    repr::Repr,
    syntax::test::symbol,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        symbol("a"),
        symbol("Abc"),
        symbol("A_BB__CCC_"),
        symbol("A1B2C3"),
    ]
}
