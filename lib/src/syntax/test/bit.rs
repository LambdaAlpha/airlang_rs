use crate::syntax::repr::Repr;
use crate::syntax::test::bit;

pub(crate) fn expected() -> Vec<Repr> {
    vec![bit(false), bit(true)]
}
