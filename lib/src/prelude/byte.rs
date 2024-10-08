use crate::{
    Map,
    Symbol,
    ctx::CtxValue,
    prelude::Prelude,
};

#[derive(Clone)]
pub(crate) struct BytePrelude {}

#[allow(clippy::derivable_impls)]
impl Default for BytePrelude {
    fn default() -> Self {
        BytePrelude {}
    }
}

impl Prelude for BytePrelude {
    fn put(&self, _m: &mut Map<Symbol, CtxValue>) {}
}
