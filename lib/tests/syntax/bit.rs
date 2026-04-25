use airlang::syntax::repr::Repr;

use crate::bit;

pub(super) fn expected() -> Vec<Repr> {
    vec![bit(false), bit(true)]
}
