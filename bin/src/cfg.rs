use airlang::cfg::CfgMod;
use airlang::cfg::CoreCfg;
use airlang::cfg::prelude::prelude_repr;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::val::Val;
use airlang::type_::Symbol;
use log::info;

use crate::cfg::lib::BinLib;
use crate::cfg::prelude::BinPrelude;

#[derive(Clone)]
pub struct BinCfg {
    pub lib: BinLib,
    pub prelude: BinPrelude,
}

impl Default for BinCfg {
    fn default() -> Self {
        let lib = BinLib::default();
        let prelude = BinPrelude::new(&lib);
        Self { lib, prelude }
    }
}

impl CfgMod for BinCfg {
    fn extend(self, cfg: &Cfg) {
        self.lib.extend(cfg);
        let prelude = prelude_repr(self.prelude);
        info!("bin prelude len {}", prelude.len());
        cfg.extend_scope(Symbol::from_str_unchecked(CoreCfg::PRELUDE), Val::Memo(prelude.into()));
    }
}

pub mod lib;

pub mod prelude;
