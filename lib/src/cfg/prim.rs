use self::lib::BasePrimLib;
use crate::cfg::CfgMod;
use crate::cfg::PRELUDE;
use crate::cfg::export;
use crate::cfg::prim::prelude::BasePrimPrelude;
use crate::cfg::prim::prelude::prelude_repr;
use crate::semantics::val::LinkVal;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;

#[derive(Copy, Clone)]
pub struct BasePrimCfg {
    pub lib: BasePrimLib,
    pub prelude: BasePrimPrelude,
}

impl Default for BasePrimCfg {
    fn default() -> Self {
        let lib = BasePrimLib::default();
        let prelude = BasePrimPrelude::new(&lib);
        Self { lib, prelude }
    }
}

impl CfgMod for BasePrimCfg {
    fn export(self, cfg: &mut Map<Key, Val>) {
        self.lib.export(cfg);
        let prelude = prelude_repr(self.prelude);
        let prelude = Val::Link(LinkVal::new(Val::Map(prelude.into()), false));
        export(cfg, PRELUDE, prelude);
    }
}

pub mod lib;

pub mod prelude;
