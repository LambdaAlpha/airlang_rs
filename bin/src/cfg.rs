use airlang::cfg::CfgMod;
use airlang::cfg::CoreCfg;
use airlang::cfg::prelude::prelude_repr;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::val::LinkVal;
use airlang::semantics::val::Val;
use airlang::type_::Key;
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
    fn extend(self, cfg: &mut Cfg) {
        self.lib.extend(cfg);
        let prelude = prelude_repr(self.prelude);
        info!("bin prelude len {}", prelude.len());
        let prelude = Val::Link(LinkVal::new(Val::Map(prelude.into()), false));
        cfg.extend(Key::from_str_unchecked(CoreCfg::PRELUDE), prelude);
    }
}

pub mod lib;

pub mod prelude;
