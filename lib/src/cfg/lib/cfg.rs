use std::ops::Deref;
use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;

use log::error;

use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::cfg::error::abort_bug_with_msg;
use crate::cfg::error::illegal_ctx;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::cfg::lib::DynPrimFn;
use crate::cfg::lib::FreePrimFn;
use crate::cfg::lib::const_impl;
use crate::cfg::lib::dyn_impl;
use crate::cfg::lib::free_impl;
use crate::cfg::lib::mut_impl;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::MutFn;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::DynRef;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::Map;
use crate::type_::Pair;

// todo design more
#[derive(Clone)]
pub struct CfgLib {
    pub new: FreePrimFuncVal,
    pub represent: FreePrimFuncVal,
    pub exist: FreePrimFuncVal,
    pub import: FreePrimFuncVal,
    pub export: FreePrimFuncVal,
    pub get_length: ConstPrimFuncVal,
    pub with: MutPrimFuncVal,
    pub self_: FreePrimFuncVal,
    pub where_: MutPrimFuncVal,
}

impl Default for CfgLib {
    fn default() -> Self {
        CfgLib {
            new: new(),
            represent: represent(),
            exist: exist(),
            import: import(),
            export: export(),
            get_length: get_length(),
            with: with(),
            self_: self_(),
            where_: where_(),
        }
    }
}

impl CfgMod for CfgLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_config.new", self.new);
        extend_func(cfg, "_config.represent", self.represent);
        extend_func(cfg, "_config.exist", self.exist);
        extend_func(cfg, "_config.import", self.import);
        extend_func(cfg, "_config.export", self.export);
        extend_func(cfg, "_config.get_length", self.get_length);
        extend_func(cfg, "_config.with", self.with);
        extend_func(cfg, "_config.self", self.self_);
        extend_func(cfg, "_config.where", self.where_);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_new) }.free()
}

fn fn_new(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Map(map) = input else {
        error!("input {input:?} should be a map");
        return illegal_input(cfg);
    };
    let new_cfg = Cfg::default();
    let map = Map::from(map);
    for (k, v) in map {
        new_cfg.extend_scope(k, v);
    }
    Val::Cfg(new_cfg.into())
}

pub fn represent() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_represent) }.free()
}

fn fn_represent(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Cfg(new_cfg) = input else {
        error!("input {input:?} should be a cfg");
        return illegal_input(cfg);
    };
    Val::Map(new_cfg.snapshot().into())
}

pub fn exist() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_exist) }.free()
}

fn fn_exist(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(name) = input else {
        error!("input {input:?} should be a key");
        return illegal_input(cfg);
    };
    let exist = cfg.exist(name);
    Val::Bit(Bit::from(exist))
}

pub fn import() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_import) }.free()
}

fn fn_import(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(name) = input else {
        error!("input {input:?} should be a key");
        return illegal_input(cfg);
    };
    let Some(value) = cfg.import(name) else {
        error!("key should exist");
        return illegal_input(cfg);
    };
    value
}

pub fn export() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_export) }.free()
}

fn fn_export(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Key(name) = pair.left else {
        error!("input.left {:?} should be a key", pair.left);
        return illegal_input(cfg);
    };
    if cfg.export(name, pair.right).is_none() {
        error!("key should not exist");
        return illegal_input(cfg);
    }
    Val::default()
}

pub fn get_length() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_length) }.const_()
}

fn fn_get_length(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Cfg(new_cfg) = &*ctx else {
        error!("ctx {ctx:?} should be a cfg");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    Val::Int(Int::from(new_cfg.len()).into())
}

pub fn with() -> MutPrimFuncVal {
    DynPrimFn { raw_input: true, f: dyn_impl(fn_with) }.mut_()
}

fn fn_with(cfg: &mut Cfg, mut ctx: DynRef<Val>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let map = Eval.dyn_call(cfg, ctx.reborrow(), pair.left);
    let Val::Map(map) = map else {
        error!("input.left {map:?} should be a map");
        return illegal_input(cfg);
    };
    cfg.begin_scope();
    let map = Map::from(map);
    for (k, v) in map {
        cfg.extend_scope(k, v);
    }
    let output = Eval.dyn_call(cfg, ctx, pair.right);
    cfg.end_scope();
    output
}

pub fn self_() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_self) }.free()
}

fn fn_self(cfg: &mut Cfg, input: Val) -> Val {
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    Val::Cfg(cfg.clone().into())
}

pub fn where_() -> MutPrimFuncVal {
    DynPrimFn { raw_input: true, f: mut_impl(fn_where) }.mut_()
}

fn fn_where(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Some(ctx) = ctx.ref_mut(pair.left) else {
        error!("input.left should be a valid reference");
        return abort_bug_with_msg(cfg, "_config.where reference is not valid");
    };
    let Val::Cfg(new_cfg) = ctx else {
        error!("ctx reference {ctx:?} should be a cfg");
        return illegal_ctx(cfg);
    };
    let prelude = new_cfg.import(Key::from_str_unchecked(CoreCfg::PRELUDE));
    let Some(prelude) = prelude else {
        error!("prelude should exist in cfg");
        return illegal_ctx(cfg);
    };
    let Val::Link(prelude) = prelude else {
        error!("prelude in cfg should be a link");
        return illegal_ctx(cfg);
    };
    let Ok(prelude) = prelude.try_borrow() else {
        error!("prelude should not be borrowed");
        return illegal_ctx(cfg);
    };
    let mut new_ctx = prelude.deref().clone();
    // unwind safety:
    // new_ctx is local variable
    // new_cfg is aborted
    let result =
        catch_unwind(AssertUnwindSafe(|| Eval.mut_call(&mut **new_cfg, &mut new_ctx, pair.right)));
    match result {
        Ok(output) => output,
        Err(err) => {
            if let Some(err) = err.downcast_ref::<String>() {
                error!("panic by {err}");
                abort_bug_with_msg(new_cfg, &format!("panic by {err}"))
            } else if let Some(err) = err.downcast_ref::<&str>() {
                error!("panic by {err}");
                abort_bug_with_msg(new_cfg, &format!("panic by {err}"))
            } else {
                error!("panic");
                abort_bug_with_msg(new_cfg, "panic")
            }
        }
    }
}
