use crate::{
    Map,
    Symbol,
    ctx::map::CtxValue,
    prelude::Prelude,
};

#[derive(Clone)]
pub(crate) struct NumberPrelude {}

#[expect(clippy::derivable_impls)]
impl Default for NumberPrelude {
    fn default() -> Self {
        NumberPrelude {}
    }
}

impl Prelude for NumberPrelude {
    fn put(&self, _m: &mut Map<Symbol, CtxValue>) {}
}
