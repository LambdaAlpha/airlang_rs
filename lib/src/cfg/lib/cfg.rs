use std::ops::Deref;
use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;

use const_format::concatcp;
use log::error;

use super::ConstImpl;
use super::DynImpl;
use super::FreeImpl;
use super::MutImpl;
use super::abort_const;
use super::abort_free;
use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::cfg::error::abort_bug_with_msg;
use crate::cfg::error::illegal_ctx;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::MutFn;
use crate::semantics::val::CFG;
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

pub const NEW: &str = concatcp!(PREFIX_ID, CFG, ".new");
pub const REPRESENT: &str = concatcp!(PREFIX_ID, CFG, ".represent");
pub const EXIST: &str = concatcp!(PREFIX_ID, CFG, ".exist");
pub const IMPORT: &str = concatcp!(PREFIX_ID, CFG, ".import");
pub const EXPORT: &str = concatcp!(PREFIX_ID, CFG, ".export");
pub const GET_LENGTH: &str = concatcp!(PREFIX_ID, CFG, ".get_length");
pub const WITH: &str = concatcp!(PREFIX_ID, CFG, ".with");
pub const SELF: &str = concatcp!(PREFIX_ID, CFG, ".self");
pub const WHERE: &str = concatcp!(PREFIX_ID, CFG, ".where");

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
        extend_func(cfg, NEW, self.new);
        extend_func(cfg, REPRESENT, self.represent);
        extend_func(cfg, EXIST, self.exist);
        extend_func(cfg, IMPORT, self.import);
        extend_func(cfg, EXPORT, self.export);
        extend_func(cfg, GET_LENGTH, self.get_length);
        extend_func(cfg, WITH, self.with);
        extend_func(cfg, SELF, self.self_);
        extend_func(cfg, WHERE, self.where_);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreeImpl { free: fn_new }.build()
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
    FreeImpl { free: fn_represent }.build()
}

fn fn_represent(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Cfg(new_cfg) = input else {
        error!("input {input:?} should be a cfg");
        return illegal_input(cfg);
    };
    Val::Map(new_cfg.snapshot().into())
}

pub fn exist() -> FreePrimFuncVal {
    FreeImpl { free: fn_exist }.build()
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
    FreeImpl { free: fn_import }.build()
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
    FreeImpl { free: fn_export }.build()
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
    ConstImpl { free: abort_free(GET_LENGTH), const_: fn_get_length }.build()
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
    DynImpl { free: abort_free(WITH), dyn_: fn_with }.build_with(true)
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
    FreeImpl { free: fn_self }.build()
}

fn fn_self(cfg: &mut Cfg, input: Val) -> Val {
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    Val::Cfg(cfg.clone().into())
}

pub fn where_() -> MutPrimFuncVal {
    MutImpl { free: abort_free(WHERE), const_: abort_const(WHERE), mut_: fn_where }.build_with(true)
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
