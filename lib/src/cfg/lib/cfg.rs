use std::ops::Deref;
use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;

use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxFreeInputEvalFunc;
use crate::semantics::func::CtxFreeInputFreeFunc;
use crate::semantics::func::CtxMutInputRawFunc;
use crate::semantics::func::DynFunc;
use crate::semantics::val::CFG;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::Map;
use crate::type_::Pair;

// todo design more
#[derive(Clone)]
pub struct CfgLib {
    pub make: PrimFuncVal,
    pub represent: PrimFuncVal,
    pub exist: PrimFuncVal,
    pub import: PrimFuncVal,
    pub export: PrimFuncVal,
    pub get_length: PrimFuncVal,
    pub with: PrimFuncVal,
    pub get_self: PrimFuncVal,
    // todo rename
    pub where_: PrimFuncVal,
}

pub const MAKE: &str = concatcp!(PREFIX_ID, CFG, ".make");
pub const REPRESENT: &str = concatcp!(PREFIX_ID, CFG, ".represent");
pub const EXIST: &str = concatcp!(PREFIX_ID, CFG, ".exist");
pub const IMPORT: &str = concatcp!(PREFIX_ID, CFG, ".import");
pub const EXPORT: &str = concatcp!(PREFIX_ID, CFG, ".export");
pub const GET_LENGTH: &str = concatcp!(PREFIX_ID, CFG, ".get_length");
pub const WITH: &str = concatcp!(PREFIX_ID, CFG, ".with");
pub const GET_SELF: &str = concatcp!(PREFIX_ID, CFG, ".get_self");
pub const WHERE: &str = concatcp!(PREFIX_ID, CFG, ".where");

impl Default for CfgLib {
    fn default() -> Self {
        CfgLib {
            make: CtxFreeInputEvalFunc { fn_: make }.build(),
            represent: CtxFreeInputEvalFunc { fn_: represent }.build(),
            exist: CtxFreeInputEvalFunc { fn_: exist }.build(),
            import: CtxFreeInputEvalFunc { fn_: import }.build(),
            export: CtxFreeInputEvalFunc { fn_: export }.build(),
            get_length: CtxConstInputFreeFunc { fn_: get_length }.build(),
            with: CtxMutInputRawFunc { fn_: with }.build(),
            get_self: CtxFreeInputFreeFunc { fn_: get_self }.build(),
            where_: CtxMutInputRawFunc { fn_: where_ }.build(),
        }
    }
}

impl CfgMod for CfgLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, MAKE, self.make);
        extend_func(cfg, REPRESENT, self.represent);
        extend_func(cfg, EXIST, self.exist);
        extend_func(cfg, IMPORT, self.import);
        extend_func(cfg, EXPORT, self.export);
        extend_func(cfg, GET_LENGTH, self.get_length);
        extend_func(cfg, WITH, self.with);
        extend_func(cfg, GET_SELF, self.get_self);
        extend_func(cfg, WHERE, self.where_);
    }
}

pub fn make(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Map(map) = input else {
        return bug!(cfg, "{MAKE}: expected input to be a map, but got {input}");
    };
    let new_cfg = Cfg::default();
    let map = Map::from(map);
    for (k, v) in map {
        new_cfg.extend_scope(k, v);
    }
    Val::Cfg(new_cfg.into())
}

pub fn represent(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Cfg(new_cfg) = input else {
        return bug!(cfg, "{REPRESENT}: expected input to be a config, but got {input}");
    };
    Val::Map(new_cfg.snapshot().into())
}

pub fn exist(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(name) = input else {
        return bug!(cfg, "{EXIST}: expected input to be a key, but got {input}");
    };
    let exist = cfg.exist(name);
    Val::Bit(Bit::from(exist))
}

pub fn import(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(name) = input else {
        return bug!(cfg, "{IMPORT}: expected input to be a key, but got {input}");
    };
    let Some(value) = cfg.import(name.clone()) else {
        return bug!(cfg, "{IMPORT}: value not found for key {name} in config");
    };
    value
}

pub fn export(cfg: &mut Cfg, input: Val) -> Val {
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

pub fn get_length(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Cfg(new_cfg) = ctx else {
        return bug!(cfg, "{GET_LENGTH}: expected context to be a config, but got {ctx}");
    };
    Val::Int(Int::from(new_cfg.len()).into())
}

pub fn with(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{WITH}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let map = Eval.call(cfg, ctx, pair.left);
    let Val::Map(map) = map else {
        return bug!(cfg, "{WITH}: expected input.left to be a map, but got {map}");
    };
    cfg.begin_scope();
    let map = Map::from(map);
    for (k, v) in map {
        cfg.extend_scope(k, v);
    }
    let output = Eval.call(cfg, ctx, pair.right);
    cfg.end_scope();
    output
}

pub fn get_self(cfg: &mut Cfg) -> Val {
    Val::Cfg(cfg.clone().into())
}

pub fn where_(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
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
        catch_unwind(AssertUnwindSafe(|| Eval.call(&mut **new_cfg, &mut new_ctx, pair.right)));
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
        },
    }
}
