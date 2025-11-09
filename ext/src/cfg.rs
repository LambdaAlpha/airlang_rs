use airlang::cfg::CfgMod;
use airlang::cfg::CoreCfg;
use airlang::cfg::prelude::prelude_repr;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::val::Val;
use airlang::type_::Link;
use airlang::type_::Symbol;
use log::info;

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
    fn extend(self, cfg: &Cfg) {
        self.lib.extend(cfg);
        let prelude = prelude_repr(self.prelude);
        info!("std prelude len {}", prelude.len());
        let prelude = Val::Link(Link::new(Val::Memo(prelude.into())));
        cfg.extend_scope(Symbol::from_str_unchecked(CoreCfg::PRELUDE), prelude);
    }
}

pub mod lib;

pub mod prelude;
