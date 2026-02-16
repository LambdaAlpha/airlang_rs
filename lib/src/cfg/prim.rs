use self::lib::BasePrimLib;
use crate::cfg::CfgMod;
use crate::cfg::KEY_PRELUDE;
use crate::cfg::prim::prelude::BasePrimPrelude;
use crate::cfg::prim::prelude::prelude_repr;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::LinkVal;
use crate::semantics::val::Val;
use crate::type_::Key;

#[derive(Clone)]
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
    fn extend(self, cfg: &mut Cfg) {
        self.lib.extend(cfg);
        let prelude = prelude_repr(self.prelude);
        let prelude = Val::Link(LinkVal::new(Val::Map(prelude.into()), false));
        cfg.extend(Key::from_str_unchecked(KEY_PRELUDE), prelude);
    }
}

pub mod lib;

pub mod prelude;
