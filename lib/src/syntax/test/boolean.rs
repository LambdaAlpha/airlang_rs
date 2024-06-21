use crate::syntax::{
    repr::Repr,
    test::bool,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![bool(false), bool(true)]
}
