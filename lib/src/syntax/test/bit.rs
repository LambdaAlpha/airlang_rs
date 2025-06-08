use crate::syntax::repr::Repr;
use crate::syntax::test::bit;

pub(super) fn expected() -> Vec<Repr> {
    vec![bit(false), bit(true)]
}
