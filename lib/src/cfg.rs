use self::prelude::CorePrelude;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::FuncVal;
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
    // todo design default solve
    pub solver: FuncVal,
}

impl CfgModule for CoreCfg {
    fn extend(self, cfg: &Cfg) {
        let prelude: Ctx = self.prelude.into();
        cfg.extend_scope(Symbol::from_str_unchecked(Self::PRELUDE), Val::Ctx(prelude.into()));
        cfg.extend_scope(Symbol::from_str_unchecked(Self::SOLVER), Val::Func(self.solver));
    }
}

impl CoreCfg {
    pub const PRELUDE: &'static str = "prelude";
    pub const SOLVER: &'static str = "solver";
    pub const REVERSE: &'static str = "reverse";
}

pub mod prelude;
