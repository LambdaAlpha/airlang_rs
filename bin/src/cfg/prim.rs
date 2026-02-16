use airlang::cfg::CfgMod;
use airlang::cfg::KEY_PRELUDE;
use airlang::cfg::prim::prelude::prelude_repr;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::val::LinkVal;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use log::info;

use crate::cfg::prim::lib::BinPrimLib;
use crate::cfg::prim::prelude::BinPrimPrelude;

#[derive(Clone)]
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
    fn extend(self, cfg: &mut Cfg) {
        self.lib.extend(cfg);
        let prelude = prelude_repr(self.prelude);
        info!("bin prelude len {}", prelude.len());
        let prelude = Val::Link(LinkVal::new(Val::Map(prelude.into()), false));
        cfg.extend(Key::from_str_unchecked(KEY_PRELUDE), prelude);
    }
}

pub mod lib;

pub mod prelude;
