use crate::semantics::{
    ctx::NameMap,
    prelude::Prelude,
};

#[derive(Clone)]
pub(crate) struct UnitPrelude {}

#[allow(clippy::derivable_impls)]
impl Default for UnitPrelude {
    fn default() -> Self {
        UnitPrelude {}
    }
}

impl Prelude for UnitPrelude {
    fn put(&self, _m: &mut NameMap) {}
}
