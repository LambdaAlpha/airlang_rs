use std::ops::Deref;

use log::error;
use log::info;

use self::lib::CoreLib;
use crate::cfg::prelude::CorePrelude;
use crate::cfg::prelude::prelude_repr;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::FuncVal;
use crate::semantics::val::LinkVal;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;

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
        let prelude = Val::Link(LinkVal::new(Val::Map(prelude.into()), false));
        cfg.extend_scope(Key::from_str_unchecked(Self::PRELUDE), prelude);
    }
}

impl CoreCfg {
    pub const PRELUDE: &'static str = "_prelude";

    pub fn prelude(cfg: &Cfg) -> Option<Map<Key, Val>> {
        let prelude = cfg.import(Key::from_str_unchecked(Self::PRELUDE));
        let Some(prelude) = prelude else {
            error!("prelude should exist in cfg");
            return None;
        };
        let Val::Link(prelude) = prelude else {
            error!("prelude in cfg should be a link");
            return None;
        };
        let Ok(prelude) = prelude.try_borrow() else {
            error!("prelude should not be borrowed");
            return None;
        };
        let Val::Map(prelude) = prelude.deref().clone() else {
            error!("prelude in cfg should be a link of map");
            return None;
        };
        Some(Map::from(prelude))
    }
}

pub fn extend(cfg: &Cfg, key: &str, val: impl Into<Val>) {
    cfg.extend_scope(Key::from_str_unchecked(key), val.into());
}

pub fn extend_func(cfg: &Cfg, key: &str, val: impl Into<FuncVal>) {
    cfg.extend_scope(Key::from_str_unchecked(key), Val::Func(val.into()));
}

pub mod lib;

pub mod prelude;

pub mod exception;

mod utils;
