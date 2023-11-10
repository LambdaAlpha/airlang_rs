use crate::semantics::{
    ctx::NameMap,
    prelude::Prelude,
};

#[derive(Clone)]
pub(crate) struct ReversePrelude {}

#[allow(clippy::derivable_impls)]
impl Default for ReversePrelude {
    fn default() -> Self {
        ReversePrelude {}
    }
}

impl Prelude for ReversePrelude {
    fn put(&self, _m: &mut NameMap) {}
}
