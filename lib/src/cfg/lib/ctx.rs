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
use crate::cfg::exception::fail;
use crate::cfg::exception::illegal_input;
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
    pub repr: MutPrimFuncVal,
    pub is_const: MutPrimFuncVal,
    pub self_: ConstPrimFuncVal,
    pub which: MutPrimFuncVal,
}

impl Default for CtxLib {
    fn default() -> Self {
        CtxLib {
            get: get(),
            set: set(),
            form: form(),
            repr: repr(),
            is_const: is_const(),
            self_: self_(),
            which: which(),
        }
    }
}

impl CfgMod for CtxLib {
    fn extend(self, cfg: &Cfg) {
        self.get.extend(cfg);
        self.set.extend(cfg);
        self.form.extend(cfg);
        self.repr.extend(cfg);
        self.is_const.extend(cfg);
        self.self_.extend(cfg);
        self.which.extend(cfg);
    }
}

pub fn get() -> ConstPrimFuncVal {
    DynPrimFn { id: "_context.get", raw_input: false, f: const_impl(fn_get) }.const_()
}

fn fn_get(_cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    match ctx.unwrap().ref_(input) {
        Some(val) => val.clone(),
        None => Val::default(),
    }
}

pub fn set() -> MutPrimFuncVal {
    DynPrimFn { id: "_context.set", raw_input: false, f: mut_impl(fn_set) }.mut_()
}

fn fn_set(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let output = ctx.set(pair.first, pair.second);
    output.unwrap_or_default()
}

pub fn form() -> ConstPrimFuncVal {
    DynPrimFn { id: "_context.form", raw_input: true, f: Form }.const_()
}

pub fn repr() -> MutPrimFuncVal {
    DynPrimFn { id: "_context.represent", raw_input: false, f: mut_impl(fn_repr) }.mut_()
}

fn fn_repr(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Some(pattern) = pair.first.parse() else {
        error!("parse pattern failed");
        return illegal_input(cfg);
    };
    let val = pair.second;
    if pattern.match_(&val) { pattern.assign(ctx, val) } else { fail(cfg) }
}

pub fn is_const() -> MutPrimFuncVal {
    DynPrimFn {
        id: "_context.is_constant",
        raw_input: false,
        f: MutImpl::new(FreeImpl::default, fn_const, fn_mut),
    }
    .mut_()
}

fn fn_const(_cfg: &mut Cfg, _ctx: ConstRef<Val>, _input: Val) -> Val {
    Val::Bit(Bit::true_())
}

fn fn_mut(_cfg: &mut Cfg, _ctx: &mut Val, _input: Val) -> Val {
    Val::Bit(Bit::false_())
}

pub fn self_() -> ConstPrimFuncVal {
    DynPrimFn { id: "_context.self", raw_input: false, f: const_impl(fn_self) }.const_()
}

fn fn_self(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    ctx.unwrap().clone()
}

pub fn which() -> MutPrimFuncVal {
    DynPrimFn { id: "_context.which", raw_input: true, f: dyn_impl(fn_which) }.mut_()
}

fn fn_which(cfg: &mut Cfg, mut ctx: DynRef<Val>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input: {:?} should be a pair", input);
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Call(call) = pair.second else {
        error!("input.second {:?} should be a call", pair.second);
        return illegal_input(cfg);
    };
    let call = Call::from(call);
    let Val::Func(func) = Eval.dyn_call(cfg, ctx.reborrow(), call.func) else {
        error!("input.second.func should be a func");
        return fail(cfg);
    };
    let input =
        if func.raw_input() { call.input } else { Eval.dyn_call(cfg, ctx.reborrow(), call.input) };
    let const_ = ctx.is_const();
    let Some(ctx) = ctx.reborrow().unwrap().ref_mut(pair.first) else {
        error!("input.first should be a valid reference");
        return fail(cfg);
    };
    func.dyn_call(cfg, DynRef::new(ctx, const_), input)
}

pub(in crate::cfg) mod pattern;
