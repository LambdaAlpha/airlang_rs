use airlang::cfg::CfgModule;
use airlang::cfg::CoreCfg;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::ctx::Ctx;
use airlang::semantics::val::Val;
use airlang::type_::Symbol;

use self::prelude::StdPrelude;

#[derive(Default, Clone)]
pub struct StdCfg {
    pub prelude: StdPrelude,
}

impl CfgModule for StdCfg {
    fn extend(self, cfg: &Cfg) {
        let prelude: Ctx = self.prelude.into();
        cfg.extend_scope(Symbol::from_str_unchecked(CoreCfg::PRELUDE), Val::Ctx(prelude.into()));
    }
}

pub mod prelude;
