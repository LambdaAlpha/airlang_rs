use crate::{
    grammar::test::bool,
    repr::Repr,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![bool(false), bool(true)]
}
