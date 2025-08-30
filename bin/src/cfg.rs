use airlang::cfg::CfgModule;
use airlang::cfg::CoreCfg;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::ctx::Ctx;
use airlang::semantics::val::Val;
use airlang::type_::Symbol;

use crate::cfg::prelude::BinPrelude;

#[derive(Default, Clone)]
pub struct BinCfg {
    pub prelude: BinPrelude,
}

impl CfgModule for BinCfg {
    fn extend(self, cfg: &Cfg) {
        let prelude: Ctx = self.prelude.into();
        cfg.extend_scope(Symbol::from_str_unchecked(CoreCfg::PRELUDE), Val::Ctx(prelude.into()));
    }
}

pub mod prelude;
