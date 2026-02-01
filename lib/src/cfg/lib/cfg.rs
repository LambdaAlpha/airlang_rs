use std::ops::Deref;
use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;

use const_format::concatcp;

use super::ConstImpl;
use super::FreeImpl;
use super::ImplExtra;
use super::MutImpl;
use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::CtxFn;
use crate::semantics::val::CFG;
use crate::semantics::val::CtxPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
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
    pub get_length: CtxPrimFuncVal,
    pub with: CtxPrimFuncVal,
    pub self_: FreePrimFuncVal,
    pub where_: CtxPrimFuncVal,
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
    FreeImpl { fn_: fn_new }.build(ImplExtra { raw_input: false })
}

fn fn_new(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Map(map) = input else {
        return bug!(cfg, "{NEW}: expected input to be a map, but got {input}");
    };
    let new_cfg = Cfg::default();
    let map = Map::from(map);
    for (k, v) in map {
        new_cfg.extend_scope(k, v);
    }
    Val::Cfg(new_cfg.into())
}

pub fn represent() -> FreePrimFuncVal {
    FreeImpl { fn_: fn_represent }.build(ImplExtra { raw_input: false })
}

fn fn_represent(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Cfg(new_cfg) = input else {
        return bug!(cfg, "{REPRESENT}: expected input to be a config, but got {input}");
    };
    Val::Map(new_cfg.snapshot().into())
}

pub fn exist() -> FreePrimFuncVal {
    FreeImpl { fn_: fn_exist }.build(ImplExtra { raw_input: false })
}

fn fn_exist(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(name) = input else {
        return bug!(cfg, "{EXIST}: expected input to be a key, but got {input}");
    };
    let exist = cfg.exist(name);
    Val::Bit(Bit::from(exist))
}

pub fn import() -> FreePrimFuncVal {
    FreeImpl { fn_: fn_import }.build(ImplExtra { raw_input: false })
}

fn fn_import(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(name) = input else {
        return bug!(cfg, "{IMPORT}: expected input to be a key, but got {input}");
    };
    let Some(value) = cfg.import(name.clone()) else {
        return bug!(cfg, "{IMPORT}: value not found for key {name} in config");
    };
    value
}

pub fn export() -> FreePrimFuncVal {
    FreeImpl { fn_: fn_export }.build(ImplExtra { raw_input: false })
}

fn fn_export(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{EXPORT}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Key(name) = pair.left else {
        return bug!(cfg, "{EXPORT}: expected input.left to be a key, but got {}", pair.left);
    };
    if cfg.export(name.clone(), pair.right).is_none() {
        return bug!(cfg, "{EXPORT}: already bound to value for key {name} in config");
    }
    Val::default()
}

pub fn get_length() -> CtxPrimFuncVal {
    ConstImpl { fn_: fn_get_length }.build(ImplExtra { raw_input: false })
}

fn fn_get_length(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Cfg(new_cfg) = ctx else {
        return bug!(cfg, "{GET_LENGTH}: expected context to be a config, but got {ctx}");
    };
    if !input.is_unit() {
        return bug!(cfg, "{GET_LENGTH}: expected input to be a unit, but got {input}");
    }
    Val::Int(Int::from(new_cfg.len()).into())
}

pub fn with() -> CtxPrimFuncVal {
    MutImpl { fn_: fn_with }.build(ImplExtra { raw_input: true })
}

fn fn_with(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{WITH}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let map = Eval.ctx_call(cfg, ctx, pair.left);
    let Val::Map(map) = map else {
        return bug!(cfg, "{WITH}: expected input.left to be a map, but got {map}");
    };
    cfg.begin_scope();
    let map = Map::from(map);
    for (k, v) in map {
        cfg.extend_scope(k, v);
    }
    let output = Eval.ctx_call(cfg, ctx, pair.right);
    cfg.end_scope();
    output
}

pub fn self_() -> FreePrimFuncVal {
    FreeImpl { fn_: fn_self }.build(ImplExtra { raw_input: false })
}

fn fn_self(cfg: &mut Cfg, input: Val) -> Val {
    if !input.is_unit() {
        return bug!(cfg, "{SELF}: expected input to be a unit, but got {input}");
    }
    Val::Cfg(cfg.clone().into())
}

pub fn where_() -> CtxPrimFuncVal {
    MutImpl { fn_: fn_where }.build(ImplExtra { raw_input: true })
}

fn fn_where(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{WHERE}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Some(ctx) = ctx.ref_mut(cfg, pair.left.clone()) else {
        return Val::default();
    };
    let Val::Cfg(new_cfg) = ctx else {
        return bug!(cfg, "{WHERE}: expected context to be a config, but got {ctx}");
    };
    let prelude = new_cfg.import(Key::from_str_unchecked(CoreCfg::PRELUDE));
    let Some(prelude) = prelude else {
        return bug!(cfg, "{WHERE}: value not found for key {} in config", CoreCfg::PRELUDE);
    };
    let Val::Link(prelude) = prelude else {
        return bug!(cfg, "{WHERE}: expected {} to be a link, but got {prelude}", CoreCfg::PRELUDE);
    };
    let Ok(prelude) = prelude.try_borrow() else {
        return bug!(cfg, "{WHERE}: link is in use");
    };
    let mut new_ctx = prelude.deref().clone();
    // unwind safety:
    // new_ctx is local variable
    // new_cfg is aborted
    let result =
        catch_unwind(AssertUnwindSafe(|| Eval.ctx_call(&mut **new_cfg, &mut new_ctx, pair.right)));
    match result {
        Ok(output) => output,
        Err(err) => {
            if let Some(err) = err.downcast_ref::<String>() {
                bug!(new_cfg, "{WHERE}: panic by {err}")
            } else if let Some(err) = err.downcast_ref::<&str>() {
                bug!(new_cfg, "{WHERE}: panic by {err}")
            } else {
                bug!(new_cfg, "{WHERE}: panic")
            }
        }
    }
}
