use std::mem::take;
use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;

use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::PRELUDE;
use crate::cfg::export_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::CtxFreeFunc;
use crate::semantics::func::DefaultFunc;
use crate::semantics::func::DynFunc;
use crate::semantics::func::FreeFunc;
use crate::semantics::func::MutFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::Map;
use crate::type_::Pair;

// todo design more
#[derive(Copy, Clone)]
pub struct CfgLib {
    pub exist: PrimFuncVal,
    pub import: PrimFuncVal,
    pub export: PrimFuncVal,
    pub get_length: PrimFuncVal,
    pub with: PrimFuncVal,
    pub get_self: PrimFuncVal,
    pub let_: PrimFuncVal,
}

const CFG: &str = "config";

pub const EXIST: &str = concatcp!(PREFIX_CELL, CFG, ".exist");
pub const IMPORT: &str = concatcp!(PREFIX_CELL, CFG, ".import");
pub const EXPORT: &str = concatcp!(PREFIX_CELL, CFG, ".export");
pub const GET_LENGTH: &str = concatcp!(PREFIX_CELL, CFG, ".get_length");
pub const WITH: &str = concatcp!(PREFIX_CELL, CFG, ".with");
pub const GET_SELF: &str = concatcp!(PREFIX_CELL, CFG, ".get_self");
pub const LET: &str = concatcp!(PREFIX_CELL, CFG, ".let");

impl Default for CfgLib {
    fn default() -> Self {
        Self {
            exist: CtxFreeFunc { fn_: exist }.build(),
            import: CtxFreeFunc { fn_: import }.build(),
            export: CtxFreeFunc { fn_: export }.build(),
            get_length: FreeFunc { fn_: get_length }.build(),
            with: DefaultFunc { fn_: with }.build(),
            get_self: FreeFunc { fn_: get_self }.build(),
            let_: MutFunc { fn_: let_ }.build(),
        }
    }
}

impl CfgMod for CfgLib {
    fn export(self, cfg: &mut Map<Key, Val>) {
        export_func(cfg, EXIST, self.exist);
        export_func(cfg, IMPORT, self.import);
        export_func(cfg, EXPORT, self.export);
        export_func(cfg, GET_LENGTH, self.get_length);
        export_func(cfg, WITH, self.with);
        export_func(cfg, GET_SELF, self.get_self);
        export_func(cfg, LET, self.let_);
    }
}

pub fn exist(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(name) = input else {
        return bug!(cfg, "{EXIST}: expected input to be a key, but got {input}");
    };
    let exist = cfg.map.contains_key(&name);
    Val::Bit(Bit::from(exist))
}

pub fn import(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(name) = input else {
        return bug!(cfg, "{IMPORT}: expected input to be a key, but got {input}");
    };
    let Some(value) = cfg.map.get(&name) else {
        return bug!(cfg, "{IMPORT}: value not found for key {name} in config");
    };
    value.clone()
}

pub fn export(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{EXPORT}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Key(name) = pair.left else {
        return bug!(cfg, "{EXPORT}: expected input.left to be a key, but got {}", pair.left);
    };
    if cfg.map.contains_key(&name) {
        return bug!(cfg, "{EXPORT}: already bound to value for key {name} in config");
    }
    cfg.map.insert(name, pair.right);
    Val::default()
}

pub fn get_length(cfg: &mut Cfg) -> Val {
    Val::Int(Int::from(cfg.map.len()).into())
}

pub fn with(cfg: &mut Cfg, ctx: Ctx<Val>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{WITH}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let map = pair.left;
    let Val::Map(map) = map else {
        return bug!(cfg, "{WITH}: expected input.left to be a map, but got {map}");
    };
    let mut backup = Map::with_capacity(map.len());
    for (k, v) in Map::from(map) {
        backup.insert(k.clone(), cfg.map.insert(k, v));
    }
    let output = Eval.call(cfg, ctx, pair.right);
    for (k, v) in backup {
        if let Some(v) = v {
            cfg.map.insert(k, v);
        } else {
            cfg.map.remove(&k);
        }
    }
    output
}

pub fn get_self(cfg: &mut Cfg) -> Val {
    Val::Map(cfg.map.clone().into())
}

pub fn let_(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{LET}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Some(ctx) = ctx.ref_mut(cfg, pair.left.clone()) else {
        return Val::default();
    };
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{LET}: expected context to be a map, but got {ctx}");
    };
    let mut new_cfg = Cfg::new(take(map));
    let output = eval_task(&mut new_cfg, LET, pair.right);
    *map = new_cfg.map.into();
    let aborted = Val::Bit(Bit::from(new_cfg.aborted));
    Val::Pair(Pair::new(output, aborted).into())
}

pub fn eval_task(cfg: &mut Cfg, tag: &str, input: Val) -> Val {
    let Some(mut ctx) = runtime_prelude(cfg, tag) else {
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

fn runtime_prelude(cfg: &mut Cfg, tag: &str) -> Option<Val> {
    let prelude = crate::cfg::import(&cfg.map, PRELUDE);
    let Some(prelude) = prelude else {
        bug!(cfg, "{tag}: value not found for key {PRELUDE} in config");
        return None;
    };
    let Val::Link(prelude) = prelude else {
        bug!(cfg, "{tag}: expected {PRELUDE} to be a link, but got {prelude}");
        return None;
    };
    let prelude = prelude.clone();
    let Ok(prelude) = prelude.try_borrow() else {
        bug!(cfg, "{tag}: link is not available");
        return None;
    };
    Some(prelude.clone())
}
