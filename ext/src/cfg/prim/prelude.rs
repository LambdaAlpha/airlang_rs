use airlang::cfg::prim::prelude::BasePrimPrelude;
use airlang::cfg::prim::prelude::Prelude;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;

use crate::cfg::prim::lib::ExtPrimLib;

#[derive(Clone)]
pub struct ExtPrimPrelude {
    base: BasePrimPrelude,
}

impl ExtPrimPrelude {
    pub fn new(lib: &ExtPrimLib) -> Self {
        Self { base: BasePrimPrelude::new(&lib.base) }
    }
}

impl Prelude for ExtPrimPrelude {
    fn extend(&self, map: &mut Map<Key, Val>) {
        self.base.extend(map);
    }
}
