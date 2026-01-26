use std::rc::Rc;

use const_format::concatcp;

use self::pattern::PatternAssign;
use self::pattern::PatternMatch;
use self::pattern::PatternParse;
use super::ConstImpl;
use super::DynImpl;
use super::MutImpl;
use super::abort_const;
use super::abort_free;
use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Form;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::ConstPrimFunc;
use crate::semantics::func::MutFn;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::DynRef;
use crate::type_::Pair;

#[derive(Clone)]
pub struct CtxLib {
    pub get: ConstPrimFuncVal,
    pub set: MutPrimFuncVal,
    pub form: ConstPrimFuncVal,
    pub represent: MutPrimFuncVal,
    pub is_constant: MutPrimFuncVal,
    pub self_: ConstPrimFuncVal,
    pub which: MutPrimFuncVal,
}

const CTX: &str = "context";

pub const GET: &str = concatcp!(PREFIX_ID, CTX, ".get");
pub const SET: &str = concatcp!(PREFIX_ID, CTX, ".set");
pub const FORM: &str = concatcp!(PREFIX_ID, CTX, ".form");
pub const REPRESENT: &str = concatcp!(PREFIX_ID, CTX, ".represent");
pub const IS_CONSTANT: &str = concatcp!(PREFIX_ID, CTX, ".is_constant");
pub const SELF: &str = concatcp!(PREFIX_ID, CTX, ".self");
pub const WHICH: &str = concatcp!(PREFIX_ID, CTX, ".which");

impl Default for CtxLib {
    fn default() -> Self {
        CtxLib {
            get: get(),
            set: set(),
            form: form(),
            represent: represent(),
            is_constant: is_constant(),
            self_: self_(),
            which: which(),
        }
    }
}

impl CfgMod for CtxLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, GET, self.get);
        extend_func(cfg, SET, self.set);
        extend_func(cfg, FORM, self.form);
        extend_func(cfg, REPRESENT, self.represent);
        extend_func(cfg, IS_CONSTANT, self.is_constant);
        extend_func(cfg, SELF, self.self_);
        extend_func(cfg, WHICH, self.which);
    }
}

pub fn get() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(GET), const_: fn_get }.build()
}

fn fn_get(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Some(val) = ctx.ref_(cfg, input) else {
        return Val::default();
    };
    val.clone()
}

pub fn set() -> MutPrimFuncVal {
    MutImpl { free: abort_free(SET), const_: abort_const(SET), mut_: fn_set }.build()
}

fn fn_set(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{SET}: expected input to be a pair, but got {input:?}");
    };
    let pair = Pair::from(pair);
    ctx.set(cfg, pair.left, pair.right);
    Val::default()
}

pub fn form() -> ConstPrimFuncVal {
    ConstPrimFunc { raw_input: true, fn_: Rc::new(Form) }.into()
}

pub fn represent() -> MutPrimFuncVal {
    MutImpl { free: abort_free(REPRESENT), const_: abort_const(REPRESENT), mut_: fn_represent }
        .build()
}

fn fn_represent(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{REPRESENT}: expected input to be a pair, but got {input:?}");
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

pub fn is_constant() -> MutPrimFuncVal {
    DynImpl { free: abort_free(IS_CONSTANT), dyn_: fn_is_constant }.build()
}

fn fn_is_constant(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    if !input.is_unit() {
        return bug!(cfg, "{IS_CONSTANT}: expected input to be a unit, but got {input:?}");
    }
    Val::Bit(Bit::from(ctx.is_const()))
}

pub fn self_() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(SELF), const_: fn_self }.build()
}

fn fn_self(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    if !input.is_unit() {
        return bug!(cfg, "{SELF}: expected input to be a unit, but got {input:?}");
    }
    ctx.unwrap().clone()
}

pub fn which() -> MutPrimFuncVal {
    DynImpl { free: abort_free(WHICH), dyn_: fn_which }.build()
}

fn fn_which(cfg: &mut Cfg, mut ctx: DynRef<Val>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{WHICH}: expected input to be a pair, but got {input:?}");
    };
    let pair = Pair::from(pair);
    let Val::Pair(func_input) = pair.right else {
        return bug!(cfg, "{WHICH}: expected input.right to be a pair, but got {:?}", pair.right);
    };
    let func_input = Pair::from(func_input);
    let Val::Func(func) = func_input.left else {
        return bug!(
            cfg,
            "{WHICH}: expected input.right.left to be a function, but got {:?}",
            func_input.left
        );
    };
    let const_ = ctx.is_const();
    let Some(ctx) = ctx.reborrow().unwrap().ref_mut(cfg, pair.left) else {
        return Val::default();
    };
    func.dyn_call(cfg, DynRef::new(ctx, const_), func_input.right)
}

pub(in crate::cfg) mod pattern;
