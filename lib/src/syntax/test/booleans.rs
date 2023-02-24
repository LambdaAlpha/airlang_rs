use crate::{
    repr::Repr,
    syntax::test::bool,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![bool(false), bool(true)]
}
