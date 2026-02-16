use airlang::cfg::CfgMod;
use airlang::cfg::KEY_PRELUDE;
use airlang::cfg::prim::prelude::prelude_repr;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::val::LinkVal;
use airlang::semantics::val::Val;
use airlang::type_::Key;

use self::lib::ExtPrimLib;
use crate::cfg::prim::prelude::ExtPrimPrelude;

#[derive(Clone)]
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
    fn extend(self, cfg: &mut Cfg) {
        self.lib.extend(cfg);
        let prelude = prelude_repr(self.prelude);
        let prelude = Val::Link(LinkVal::new(Val::Map(prelude.into()), false));
        cfg.extend(Key::from_str_unchecked(KEY_PRELUDE), prelude);
    }
}

pub mod lib;

pub mod prelude;
