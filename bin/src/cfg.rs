use airlang::cfg::CfgMod;
use airlang::cfg::CoreCfg;
use airlang::cfg::lib::Library;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::memo::Memo;
use airlang::semantics::val::Val;
use airlang::type_::Symbol;

use crate::cfg::lib::BinLib;

#[derive(Clone)]
pub struct BinCfg {
    pub lib: BinLib,
    pub prelude: Memo,
}

impl Default for BinCfg {
    fn default() -> Self {
        let lib = BinLib::default();
        let mut prelude = Memo::default();
        lib.prelude(&mut prelude);
        Self { lib, prelude }
    }
}

impl CfgMod for BinCfg {
    fn extend(self, cfg: &Cfg) {
        self.lib.extend(cfg);
        cfg.extend_scope(
            Symbol::from_str_unchecked(CoreCfg::PRELUDE),
            Val::Memo(self.prelude.into()),
        );
    }
}

pub mod lib;
