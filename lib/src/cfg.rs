use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;

use self::lib::CoreLib;
use crate::bug;
use crate::cfg::prelude::CorePrelude;
use crate::cfg::prelude::prelude_repr;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::func::DynFunc;
use crate::semantics::val::LinkVal;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Key;

pub trait CfgMod {
    fn extend(self, cfg: &mut Cfg);
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
    fn extend(self, cfg: &mut Cfg) {
        self.lib.extend(cfg);
        let prelude = prelude_repr(self.prelude);
        let prelude = Val::Link(LinkVal::new(Val::Map(prelude.into()), false));
        cfg.extend(Key::from_str_unchecked(Self::PRELUDE), prelude);
    }
}

impl CoreCfg {
    pub const PRELUDE: &str = "_prelude";

    pub fn prelude(cfg: &mut Cfg, tag: &str) -> Option<Val> {
        let prelude = cfg.import(Key::from_str_unchecked(Self::PRELUDE));
        let Some(prelude) = prelude else {
            bug!(cfg, "{tag}: value not found for key {} in config", Self::PRELUDE);
            return None;
        };
        let Val::Link(prelude) = prelude else {
            bug!(cfg, "{tag}: expected {} to be a link, but got {prelude}", Self::PRELUDE);
            return None;
        };
        let prelude = prelude.clone();
        let Ok(prelude) = prelude.try_borrow() else {
            bug!(cfg, "{tag}: link is in use");
            return None;
        };
        Some(prelude.clone())
    }

    pub fn eval_with_prelude(cfg: &mut Cfg, tag: &str, input: Val) -> Val {
        let Some(mut ctx) = Self::prelude(cfg, tag) else {
            return Val::default();
        };
        // unwind safety:
        // ctx is local variable
        // cfg is aborted
        let result = catch_unwind(AssertUnwindSafe(|| Eval.call(cfg, &mut ctx, input)));
        match result {
            Ok(output) => output,
            Err(err) => {
                if let Some(err) = err.downcast_ref::<String>() {
                    bug!(cfg, "{tag}: panic by {err}")
                } else if let Some(err) = err.downcast_ref::<&str>() {
                    bug!(cfg, "{tag}: panic by {err}")
                } else {
                    bug!(cfg, "{tag}: panic")
                }
            },
        }
    }
}

pub fn extend(cfg: &mut Cfg, key: &str, val: impl Into<Val>) {
    cfg.extend(Key::from_str_unchecked(key), val.into());
}

pub fn extend_func(cfg: &mut Cfg, key: &str, val: PrimFuncVal) {
    cfg.extend(Key::from_str_unchecked(key), Val::Func(val.into()));
}

pub mod lib;

pub mod prelude;

pub mod error;

mod repr;

mod utils;
