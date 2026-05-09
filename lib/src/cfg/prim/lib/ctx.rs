use const_format::concatcp;

use self::pattern::PatternAssign;
use self::pattern::PatternMatch;
use self::pattern::PatternParse;
use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::ConstFunc;
use crate::semantics::func::ConstInputFreeFunc;
use crate::semantics::func::DefaultFunc;
use crate::semantics::func::DynFunc;
use crate::semantics::func::InputFreeFunc;
use crate::semantics::func::MutFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Pair;

#[derive(Copy, Clone)]
pub struct CtxLib {
    pub get: PrimFuncVal,
    pub set: PrimFuncVal,
    pub is_constant: PrimFuncVal,
    pub is: PrimFuncVal,
    pub get_self: PrimFuncVal,
    pub let_: PrimFuncVal,
}

const CTX: &str = "context";

pub const GET: &str = concatcp!(PREFIX_CELL, CTX, ".get");
pub const SET: &str = concatcp!(PREFIX_CELL, CTX, ".set");
pub const IS_CONSTANT: &str = concatcp!(PREFIX_CELL, CTX, ".is_constant");
pub const IS: &str = concatcp!(PREFIX_CELL, CTX, ".is");
pub const GET_SELF: &str = concatcp!(PREFIX_CELL, CTX, ".get_self");
pub const LET: &str = concatcp!(PREFIX_CELL, CTX, ".let");

impl Default for CtxLib {
    fn default() -> Self {
        Self {
            get: ConstFunc { fn_: get }.build(),
            set: MutFunc { fn_: set }.build(),
            is_constant: InputFreeFunc { fn_: is_constant }.build(),
            is: MutFunc { fn_: is }.build(),
            get_self: ConstInputFreeFunc { fn_: get_self }.build(),
            let_: DefaultFunc { fn_: let_ }.build(),
        }
    }
}

impl CfgMod for CtxLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, GET, self.get);
        extend_func(cfg, SET, self.set);
        extend_func(cfg, IS_CONSTANT, self.is_constant);
        extend_func(cfg, IS, self.is);
        extend_func(cfg, GET_SELF, self.get_self);
        extend_func(cfg, LET, self.let_);
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

pub fn is_constant(_cfg: &mut Cfg, ctx: Ctx<Val>) -> Val {
    Val::Bit(Bit::from(ctx.const_))
}

pub fn is(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{IS}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Some(pattern) = pair.left.parse(cfg, IS) else {
        return Val::default();
    };
    let val = pair.right;
    if !pattern.match_(cfg, true, IS, &val) {
        return Val::default();
    }
    pattern.assign(cfg, IS, Ctx::new_mut(ctx), val);
    Val::default()
}

pub fn get_self(_cfg: &mut Cfg, ctx: &Val) -> Val {
    ctx.clone()
}

pub fn let_(cfg: &mut Cfg, ctx: Ctx<Val>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{LET}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Pair(func_input) = pair.right else {
        return bug!(cfg, "{LET}: expected input.right to be a pair, but got {}", pair.right);
    };
    let func_input = Pair::from(func_input);
    let Val::Func(func) = func_input.left else {
        return bug!(cfg, "{LET}: expected input.right.left to be a function, \
            but got {}", func_input.left);
    };
    let Some(val) = ctx.val.ref_mut(cfg, pair.left) else {
        return Val::default();
    };
    let ctx = Ctx { val, const_: ctx.const_ };
    func.call(cfg, ctx, func_input.right)
}

pub(in crate::cfg) mod pattern;
