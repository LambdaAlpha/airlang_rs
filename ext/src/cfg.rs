use airlang::cfg::CfgMod;
use airlang::cfg::CoreCfg;
use airlang::cfg::prelude::prelude_repr;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::val::LinkVal;
use airlang::semantics::val::Val;
use airlang::type_::Key;

use self::lib::StdLib;
use crate::cfg::prelude::StdPrelude;

#[derive(Clone)]
pub struct StdCfg {
    pub lib: StdLib,
    pub prelude: StdPrelude,
}

impl Default for StdCfg {
    fn default() -> Self {
        let lib = StdLib::default();
        let prelude = StdPrelude::new(&lib);
        Self { lib, prelude }
    }
}

impl CfgMod for StdCfg {
    fn extend(self, cfg: &mut Cfg) {
        self.lib.extend(cfg);
        let prelude = prelude_repr(self.prelude);
        let prelude = Val::Link(LinkVal::new(Val::Map(prelude.into()), false));
        cfg.extend(Key::from_str_unchecked(CoreCfg::PRELUDE), prelude);
    }
}

pub mod lib;

pub mod prelude;
