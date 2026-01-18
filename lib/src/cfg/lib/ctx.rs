use log::error;

use self::pattern::PatternAssign;
use self::pattern::PatternMatch;
use self::pattern::PatternParse;
use super::DynPrimFn;
use super::FreeImpl;
use super::MutImpl;
use super::const_impl;
use super::dyn_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::error::abort_bug_with_msg;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::Form;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::MutFn;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Call;
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
        extend_func(cfg, "_context.get", self.get);
        extend_func(cfg, "_context.set", self.set);
        extend_func(cfg, "_context.form", self.form);
        extend_func(cfg, "_context.represent", self.represent);
        extend_func(cfg, "_context.is_constant", self.is_constant);
        extend_func(cfg, "_context.self", self.self_);
        extend_func(cfg, "_context.which", self.which);
    }
}

pub fn get() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get) }.const_()
}

fn fn_get(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Some(val) = ctx.unwrap().ref_(input) else {
        error!("get failed");
        return abort_bug_with_msg(cfg, "_context.get failed");
    };
    val.clone()
}

pub fn set() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_set) }.mut_()
}

fn fn_set(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    if ctx.set(pair.left, pair.right).is_none() {
        error!("set failed");
        return abort_bug_with_msg(cfg, "_context.set failed");
    }
    Val::default()
}

pub fn form() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: true, f: Form }.const_()
}

pub fn represent() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_represent) }.mut_()
}

fn fn_represent(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Some(pattern) = pair.left.parse() else {
        error!("parse failed");
        return illegal_input(cfg);
    };
    let val = pair.right;
    if !pattern.match_(&val) {
        error!("match failed");
        return abort_bug_with_msg(cfg, "_context.represent not match");
    }
    if pattern.assign(ctx, val).is_none() {
        error!("set failed");
        return abort_bug_with_msg(cfg, "_context.represent assign failed");
    }
    Val::default()
}

pub fn is_constant() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: MutImpl::new(FreeImpl::abort, fn_const, fn_mut) }.mut_()
}

fn fn_const(cfg: &mut Cfg, _ctx: ConstRef<Val>, input: Val) -> Val {
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    Val::Bit(Bit::true_())
}

fn fn_mut(cfg: &mut Cfg, _ctx: &mut Val, input: Val) -> Val {
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    Val::Bit(Bit::false_())
}

pub fn self_() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_self) }.const_()
}

fn fn_self(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    ctx.unwrap().clone()
}

pub fn which() -> MutPrimFuncVal {
    DynPrimFn { raw_input: true, f: dyn_impl(fn_which) }.mut_()
}

fn fn_which(cfg: &mut Cfg, mut ctx: DynRef<Val>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input: {:?} should be a pair", input);
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Call(call) = pair.right else {
        error!("input.right {:?} should be a call", pair.right);
        return illegal_input(cfg);
    };
    let call = Call::from(call);
    let Val::Func(func) = Eval.dyn_call(cfg, ctx.reborrow(), call.func) else {
        error!("input.right.func should be a func");
        return illegal_input(cfg);
    };
    let input =
        if func.raw_input() { call.input } else { Eval.dyn_call(cfg, ctx.reborrow(), call.input) };
    let const_ = ctx.is_const();
    let Some(ctx) = ctx.reborrow().unwrap().ref_mut(pair.left) else {
        error!("input.left should be a valid reference");
        return abort_bug_with_msg(cfg, "_context.which reference is not valid");
    };
    func.dyn_call(cfg, DynRef::new(ctx, const_), input)
}

pub(in crate::cfg) mod pattern;
