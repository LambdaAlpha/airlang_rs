use airlang::cfg::CfgMod;
use airlang::cfg::PRELUDE;
use airlang::cfg::export;
use airlang::cfg::prim::prelude::prelude_repr;
use airlang::semantics::val::LinkVal;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;

use crate::cfg::prim::lib::BinPrimLib;
use crate::cfg::prim::prelude::BinPrimPrelude;

#[derive(Copy, Clone)]
pub struct BinPrimCfg {
    pub lib: BinPrimLib,
    pub prelude: BinPrimPrelude,
}

impl Default for BinPrimCfg {
    fn default() -> Self {
        let lib = BinPrimLib::default();
        let prelude = BinPrimPrelude::new(&lib);
        Self { lib, prelude }
    }
}

impl CfgMod for BinPrimCfg {
    fn export(self, cfg: &mut Map<Key, Val>) {
        self.lib.export(cfg);
        let prelude = prelude_repr(self.prelude);
        let prelude = Val::Link(LinkVal::new(Val::Map(prelude.into()), false));
        export(cfg, PRELUDE, prelude);
    }
}

pub mod lib;

pub mod prelude;
