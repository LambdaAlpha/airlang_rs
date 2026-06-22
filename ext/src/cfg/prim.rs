use airlang::cfg::CfgMod;
use airlang::cfg::PRELUDE;
use airlang::cfg::export;
use airlang::cfg::prim::prelude::prelude_repr;
use airlang::semantics::val::LinkVal;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;

use self::lib::ExtPrimLib;
use crate::cfg::prim::prelude::ExtPrimPrelude;

#[derive(Copy, Clone)]
pub struct ExtPrimCfg {
    pub lib: ExtPrimLib,
    pub prelude: ExtPrimPrelude,
}

impl Default for ExtPrimCfg {
    fn default() -> Self {
        let lib = ExtPrimLib::default();
        let prelude = ExtPrimPrelude::new(&lib);
        Self { lib, prelude }
    }
}

impl CfgMod for ExtPrimCfg {
    fn export(self, cfg: &mut Map<Key, Val>) {
        self.lib.export(cfg);
        let prelude = prelude_repr(self.prelude);
        let prelude = Val::Link(LinkVal::new(Val::Map(prelude.into()), false));
        export(cfg, PRELUDE, prelude);
    }
}

pub mod lib;

pub mod prelude;
