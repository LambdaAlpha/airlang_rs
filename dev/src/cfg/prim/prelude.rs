use airlang::cfg::prim::prelude::Prelude;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;
use airlang_ext::cfg::prim::prelude::ExtPrimPrelude;

use crate::cfg::prim::lib::DevPrimLib;

#[derive(Copy, Clone)]
pub struct DevPrimPrelude {
    pub ext: ExtPrimPrelude,
}

impl DevPrimPrelude {
    pub fn new(lib: &DevPrimLib) -> Self {
        Self { ext: ExtPrimPrelude::new(&lib.ext) }
    }
}

impl Prelude for DevPrimPrelude {
    fn extend(&self, map: &mut Map<Key, Val>) {
        self.ext.extend(map);
    }
}
