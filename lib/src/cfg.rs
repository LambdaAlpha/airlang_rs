use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;

use const_format::concatcp;

use crate::bug;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::DynFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Key;

pub trait CfgMod {
    fn extend(self, cfg: &mut Cfg);
}

pub fn extend(cfg: &mut Cfg, key: &str, val: impl Into<Val>) {
    cfg.extend(Key::from_str_unchecked(key), val.into());
}

pub fn extend_func(cfg: &mut Cfg, key: &str, val: PrimFuncVal) {
    cfg.extend(Key::from_str_unchecked(key), Val::Func(val.into()));
}

pub const KEY_PRELUDE: &str = concatcp!(PREFIX_CELL, "prelude");

pub fn prelude(cfg: &mut Cfg) -> Val {
    let prelude = cfg.import(Key::from_str_unchecked(KEY_PRELUDE));
    let Some(prelude) = prelude else {
        panic!("value not found for key {} in config", KEY_PRELUDE);
    };
    let Val::Link(prelude) = prelude else {
        panic!("expected {} to be a link, but got {prelude}", KEY_PRELUDE);
    };
    let prelude = prelude.clone();
    let Ok(prelude) = prelude.try_borrow() else {
        panic!("link is not available");
    };
    prelude.clone()
}

pub fn opt_prelude(cfg: &mut Cfg, tag: &str) -> Option<Val> {
    let prelude = cfg.import(Key::from_str_unchecked(KEY_PRELUDE));
    let Some(prelude) = prelude else {
        bug!(cfg, "{tag}: value not found for key {} in config", KEY_PRELUDE);
        return None;
    };
    let Val::Link(prelude) = prelude else {
        bug!(cfg, "{tag}: expected {} to be a link, but got {prelude}", KEY_PRELUDE);
        return None;
    };
    let prelude = prelude.clone();
    let Ok(prelude) = prelude.try_borrow() else {
        bug!(cfg, "{tag}: link is not available");
        return None;
    };
    Some(prelude.clone())
}

pub fn eval_with_prelude(cfg: &mut Cfg, tag: &str, input: Val) -> Val {
    let Some(mut ctx) = opt_prelude(cfg, tag) else {
        return Val::default();
    };
    let ctx = Ctx::new_mut(&mut ctx);
    // unwind safety:
    // ctx is local variable
    // cfg is aborted
    let result = catch_unwind(AssertUnwindSafe(|| Eval.call(cfg, ctx, input)));
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

pub mod prim;

pub mod comp;

pub mod error;

mod repr;

mod utils;
