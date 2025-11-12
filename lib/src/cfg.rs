use log::error;
use log::info;

use self::lib::CoreLib;
use crate::cfg::lib::adapter::CoreAdapter;
use crate::cfg::lib::adapter::adapter_func;
use crate::cfg::prelude::CorePrelude;
use crate::cfg::prelude::prelude_repr;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::CFG_ADAPTER;
use crate::semantics::memo::Memo;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Link;
use crate::type_::Symbol;

pub trait CfgMod {
    fn extend(self, cfg: &Cfg);
}

#[derive(Clone)]
pub struct CoreCfg {
    pub lib: CoreLib,
    pub prelude: CorePrelude,
}

impl Default for CoreCfg {
    fn default() -> Self {
        let lib = CoreLib::default();
        let prelude = CorePrelude::new(&lib);
        Self { lib, prelude }
    }
}

impl CfgMod for CoreCfg {
    fn extend(self, cfg: &Cfg) {
        self.lib.extend(cfg);
        let prelude = prelude_repr(self.prelude);
        info!("core prelude len {}", prelude.len());
        let prelude = Val::Link(Link::new(Val::Memo(prelude.into())));
        cfg.extend_scope(Symbol::from_str_unchecked(Self::PRELUDE), prelude);
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

    pub fn prelude(cfg: &Cfg) -> Option<Memo> {
        let prelude = cfg.import(Symbol::from_str_unchecked(Self::PRELUDE));
        let Some(prelude) = prelude else {
            error!("prelude should exist in cfg");
            return None;
        };
        let Val::Link(link) = prelude else {
            error!("prelude in cfg should be a link");
            return None;
        };
        let prelude = link.get_clone();
        let Val::Memo(prelude) = prelude else {
            error!("prelude in cfg should be a link of memo");
            return None;
        };
        Some(Memo::from(prelude))
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

pub mod lib;

pub mod prelude;

pub mod exception;

mod utils;
