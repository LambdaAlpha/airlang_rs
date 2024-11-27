use crate::syntax::{
    repr::Repr,
    test::bit,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![bit(false), bit(true)]
}
