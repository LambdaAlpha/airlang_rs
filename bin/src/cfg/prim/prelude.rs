use airlang::cfg::prim::prelude::Prelude;
use airlang::cfg::prim::prelude::map_put_func;
use airlang::semantics::val::PrimFuncVal;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;
use airlang_ext::cfg::prim::prelude::ExtPrimPrelude;

use crate::cfg::prim::lib::BinPrimLib;

#[derive(Copy, Clone)]
pub struct BinPrimPrelude {
    pub ext: ExtPrimPrelude,
    pub run: PrimFuncVal,
}

impl BinPrimPrelude {
    pub fn new(lib: &BinPrimLib) -> Self {
        Self { ext: ExtPrimPrelude::new(&lib.ext), run: lib.cmd.run }
    }
}

impl Prelude for BinPrimPrelude {
    fn extend(&self, map: &mut Map<Key, Val>) {
        self.ext.extend(map);
        map_put_func(map, "run", self.run);
    }
}
