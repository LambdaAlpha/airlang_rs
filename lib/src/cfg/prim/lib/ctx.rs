use const_format::concatcp;

use self::pattern::PatternAssign;
use self::pattern::PatternMatch;
use self::pattern::PatternParse;
use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::CtxConstInputAwareFunc;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxMutInputAwareFunc;
use crate::semantics::func::DynFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Pair;

#[derive(Clone)]
pub struct CtxLib {
    pub get: PrimFuncVal,
    pub set: PrimFuncVal,
    pub represent: PrimFuncVal,
    pub get_self: PrimFuncVal,
    // todo rename
    pub which: PrimFuncVal,
}

const CTX: &str = "context";

pub const GET: &str = concatcp!(PREFIX_CELL, CTX, ".get");
pub const SET: &str = concatcp!(PREFIX_CELL, CTX, ".set");
pub const REPRESENT: &str = concatcp!(PREFIX_CELL, CTX, ".represent");
pub const GET_SELF: &str = concatcp!(PREFIX_CELL, CTX, ".get_self");
pub const WHICH: &str = concatcp!(PREFIX_CELL, CTX, ".which");

impl Default for CtxLib {
    fn default() -> Self {
        CtxLib {
            get: CtxConstInputAwareFunc { fn_: get }.build(),
            set: CtxMutInputAwareFunc { fn_: set }.build(),
            represent: CtxMutInputAwareFunc { fn_: represent }.build(),
            get_self: CtxConstInputFreeFunc { fn_: get_self }.build(),
            which: CtxMutInputAwareFunc { fn_: which }.build(),
        }
    }
}

impl CfgMod for CtxLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, GET, self.get);
        extend_func(cfg, SET, self.set);
        extend_func(cfg, REPRESENT, self.represent);
        extend_func(cfg, GET_SELF, self.get_self);
        extend_func(cfg, WHICH, self.which);
    }
}

pub fn get(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Some(val) = ctx.ref_(cfg, input) else {
        return Val::default();
    };
    val.clone()
}

pub fn set(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{SET}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    ctx.set(cfg, pair.left, pair.right);
    Val::default()
}

pub fn represent(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{REPRESENT}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Some(pattern) = pair.left.parse(cfg, REPRESENT) else {
        return Val::default();
    };
    let val = pair.right;
    if !pattern.match_(cfg, true, REPRESENT, &val) {
        return Val::default();
    }
    pattern.assign(cfg, REPRESENT, ctx, val);
    Val::default()
}

pub fn get_self(_cfg: &mut Cfg, ctx: &Val) -> Val {
    ctx.clone()
}

pub fn which(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{WHICH}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Pair(func_input) = pair.right else {
        return bug!(cfg, "{WHICH}: expected input.right to be a pair, but got {}", pair.right);
    };
    let func_input = Pair::from(func_input);
    let Val::Func(func) = func_input.left else {
        return bug!(cfg, "{WHICH}: expected input.right.left to be a function, \
            but got {}", func_input.left);
    };
    let Some(ctx) = ctx.ref_mut(cfg, pair.left) else {
        return Val::default();
    };
    func.call(cfg, ctx, func_input.right)
}

pub(in crate::cfg) mod pattern;
