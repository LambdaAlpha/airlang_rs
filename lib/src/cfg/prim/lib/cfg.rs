use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::eval_with_prelude;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::ConstInputFreeFunc;
use crate::semantics::func::CtxFreeFunc;
use crate::semantics::func::DefaultFunc;
use crate::semantics::func::DynFunc;
use crate::semantics::func::FreeFunc;
use crate::semantics::func::MutFunc;
use crate::semantics::val::CFG;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Int;
use crate::type_::Map;
use crate::type_::Pair;

// todo design more
#[derive(Copy, Clone)]
pub struct CfgLib {
    pub make: PrimFuncVal,
    pub represent: PrimFuncVal,
    pub exist: PrimFuncVal,
    pub import: PrimFuncVal,
    pub export: PrimFuncVal,
    pub get_length: PrimFuncVal,
    pub with: PrimFuncVal,
    pub get_self: PrimFuncVal,
    pub let_: PrimFuncVal,
}

pub const MAKE: &str = concatcp!(PREFIX_CELL, CFG, ".make");
pub const REPRESENT: &str = concatcp!(PREFIX_CELL, CFG, ".represent");
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
            make: CtxFreeFunc { fn_: make }.build(),
            represent: CtxFreeFunc { fn_: represent }.build(),
            exist: CtxFreeFunc { fn_: exist }.build(),
            import: CtxFreeFunc { fn_: import }.build(),
            export: CtxFreeFunc { fn_: export }.build(),
            get_length: ConstInputFreeFunc { fn_: get_length }.build(),
            with: DefaultFunc { fn_: with }.build(),
            get_self: FreeFunc { fn_: get_self }.build(),
            let_: MutFunc { fn_: let_ }.build(),
        }
    }
}

impl CfgMod for CfgLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, MAKE, self.make);
        extend_func(cfg, REPRESENT, self.represent);
        extend_func(cfg, EXIST, self.exist);
        extend_func(cfg, IMPORT, self.import);
        extend_func(cfg, EXPORT, self.export);
        extend_func(cfg, GET_LENGTH, self.get_length);
        extend_func(cfg, WITH, self.with);
        extend_func(cfg, GET_SELF, self.get_self);
        extend_func(cfg, LET, self.let_);
    }
}

pub fn make(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Map(map) = input else {
        return bug!(cfg, "{MAKE}: expected input to be a map, but got {input}");
    };
    Val::Cfg(Cfg::from(Map::from(map)).into())
}

pub fn represent(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Cfg(new_cfg) = input else {
        return bug!(cfg, "{REPRESENT}: expected input to be a config, but got {input}");
    };
    Val::Map(Map::from(Cfg::from(new_cfg)).into())
}

pub fn exist(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(name) = input else {
        return bug!(cfg, "{EXIST}: expected input to be a key, but got {input}");
    };
    let exist = cfg.contains_key(&name);
    Val::Bit(Bit::from(exist))
}

pub fn import(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(name) = input else {
        return bug!(cfg, "{IMPORT}: expected input to be a key, but got {input}");
    };
    let Some(value) = cfg.get(&name) else {
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
    if cfg.contains_key(&name) {
        return bug!(cfg, "{EXPORT}: already bound to value for key {name} in config");
    }
    cfg.insert(name, pair.right);
    Val::default()
}

pub fn get_length(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Cfg(new_cfg) = ctx else {
        return bug!(cfg, "{GET_LENGTH}: expected context to be a config, but got {ctx}");
    };
    Val::Int(Int::from(new_cfg.len()).into())
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
        backup.insert(k.clone(), cfg.insert(k, v));
    }
    let output = Eval.call(cfg, ctx, pair.right);
    for (k, v) in backup {
        if let Some(v) = v {
            cfg.insert(k, v);
        } else {
            cfg.remove(&k);
        }
    }
    output
}

pub fn get_self(cfg: &mut Cfg) -> Val {
    Val::Cfg(cfg.clone().into())
}

pub fn let_(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{LET}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Some(ctx) = ctx.ref_mut(cfg, pair.left.clone()) else {
        return Val::default();
    };
    let Val::Cfg(new_cfg) = ctx else {
        return bug!(cfg, "{LET}: expected context to be a config, but got {ctx}");
    };
    eval_with_prelude(new_cfg, LET, pair.right)
}
