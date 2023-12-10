use crate::{
    ctx::NameMap,
    prelude::Prelude,
};

#[derive(Clone)]
pub(crate) struct SymbolPrelude {}

#[allow(clippy::derivable_impls)]
impl Default for SymbolPrelude {
    fn default() -> Self {
        SymbolPrelude {}
    }
}

impl Prelude for SymbolPrelude {
    fn put(&self, _m: &mut NameMap) {}
}
