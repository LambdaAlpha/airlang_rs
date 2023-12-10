use crate::{
    ctx::NameMap,
    prelude::Prelude,
};

#[derive(Clone)]
pub(crate) struct BytesPrelude {}

#[allow(clippy::derivable_impls)]
impl Default for BytesPrelude {
    fn default() -> Self {
        BytesPrelude {}
    }
}

impl Prelude for BytesPrelude {
    fn put(&self, _m: &mut NameMap) {}
}
