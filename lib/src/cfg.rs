use self::adapter::CoreAdapter;
use self::lib::CoreLib;
use crate::cfg::adapter::adapter_func;
use crate::cfg::lib::Library;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::CFG_ADAPTER;
use crate::semantics::memo::Memo;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Symbol;

pub trait CfgMod {
    fn extend(self, cfg: &Cfg);
}

impl<T: CfgMod> From<T> for Cfg {
    fn from(t: T) -> Cfg {
        let cfg = Cfg::default();
        t.extend(&cfg);
        cfg
    }
}

#[derive(Clone)]
pub struct CoreCfg {
    pub lib: CoreLib,
    pub prelude: Memo,
}

impl Default for CoreCfg {
    fn default() -> Self {
        let lib = CoreLib::default();
        let mut prelude = Memo::default();
        lib.prelude(&mut prelude);
        Self { lib, prelude }
    }
}

impl CfgMod for CoreCfg {
    fn extend(self, cfg: &Cfg) {
        self.lib.extend(cfg);
        cfg.extend_scope(Symbol::from_str_unchecked(Self::PRELUDE), Val::Memo(self.prelude.into()));
    }
}

impl CoreCfg {
    pub const PRELUDE: &'static str = "prelude";
    pub const ADAPTER: &'static str = CFG_ADAPTER;
    pub const REVERSE: &'static str = "reverse";

    pub fn extend_adapter(cfg: &Cfg, id: &str, adapter: CoreAdapter) -> Option<()> {
        let id = format!("{}@{id}", Self::ADAPTER);
        let adapter = Val::Func(adapter_func(adapter));
        cfg.extend_scope(Symbol::from_string_unchecked(id), adapter)
    }
}

pub(crate) trait Named {
    fn name(&self) -> Symbol;
}

impl Named for FreePrimFuncVal {
    fn name(&self) -> Symbol {
        self.id.clone()
    }
}

impl Named for ConstPrimFuncVal {
    fn name(&self) -> Symbol {
        self.id.clone()
    }
}

impl Named for MutPrimFuncVal {
    fn name(&self) -> Symbol {
        self.id.clone()
    }
}

impl<F: Named + Into<FuncVal>> CfgMod for F {
    fn extend(self, cfg: &Cfg) {
        cfg.extend_scope(self.name(), Val::Func(self.into()));
    }
}

pub mod adapter;

pub mod lib;

mod utils;
