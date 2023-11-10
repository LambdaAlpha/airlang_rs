use crate::semantics::{
    ctx::NameMap,
    prelude::Prelude,
};
#[derive(Clone)]
pub(crate) struct FloatPrelude {}

#[allow(clippy::derivable_impls)]
impl Default for FloatPrelude {
    fn default() -> Self {
        FloatPrelude {}
    }
}

impl Prelude for FloatPrelude {
    fn put(&self, _m: &mut NameMap) {}
}
