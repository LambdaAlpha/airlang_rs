use airlang::cfg::CfgMod;
use airlang::cfg::PRELUDE;
use airlang::cfg::export;
use airlang::cfg::prim::prelude::prelude_repr;
use airlang::semantics::val::LinkVal;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;

use crate::cfg::prim::lib::DevPrimLib;
use crate::cfg::prim::prelude::DevPrimPrelude;

#[derive(Copy, Clone)]
pub struct DevPrimCfg {
    pub lib: DevPrimLib,
    pub prelude: DevPrimPrelude,
}

impl Default for DevPrimCfg {
    fn default() -> Self {
        let lib = DevPrimLib::default();
        let prelude = DevPrimPrelude::new(&lib);
        Self { lib, prelude }
    }
}

impl CfgMod for DevPrimCfg {
    fn export(self, cfg: &mut Map<Key, Val>) {
        self.lib.export(cfg);
        let prelude = prelude_repr(self.prelude);
        let prelude = Val::Link(LinkVal::new(Val::Map(prelude.into()), false));
        export(cfg, PRELUDE, prelude);
    }
}

pub mod lib;

pub mod prelude;
