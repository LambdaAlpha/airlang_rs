use self::prelude::CorePrelude;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::Val;
use crate::type_::Symbol;

pub trait CfgModule {
    fn extend(self, cfg: &Cfg);
}

impl<T: CfgModule> From<T> for Cfg {
    fn from(t: T) -> Cfg {
        let cfg = Cfg::default();
        t.extend(&cfg);
        cfg
    }
}

#[derive(Default, Clone)]
pub struct CoreCfg {
    pub prelude: CorePrelude,
}

impl CfgModule for CoreCfg {
    fn extend(self, cfg: &Cfg) {
        let prelude: Ctx = self.prelude.into();
        cfg.extend_scope(Symbol::from_str_unchecked(Self::PRELUDE), Val::Ctx(prelude.into()));
    }
}

impl CoreCfg {
    pub const PRELUDE: &'static str = "prelude";
}

pub mod prelude;
