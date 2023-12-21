use crate::{
    ctx::NameMap,
    prelude::Prelude,
};

#[derive(Clone)]
pub(crate) struct ExtPrelude {}

#[allow(clippy::derivable_impls)]
impl Default for ExtPrelude {
    fn default() -> Self {
        ExtPrelude {}
    }
}

impl Prelude for ExtPrelude {
    fn put(&self, _m: &mut NameMap) {}
}
