use crate::{
    Map,
    Symbol,
    ctx::CtxValue,
    prelude::Prelude,
};

#[derive(Clone)]
pub(crate) struct NumberPrelude {}

#[allow(clippy::derivable_impls)]
impl Default for NumberPrelude {
    fn default() -> Self {
        NumberPrelude {}
    }
}

impl Prelude for NumberPrelude {
    fn put(&self, _m: &mut Map<Symbol, CtxValue>) {}
}
