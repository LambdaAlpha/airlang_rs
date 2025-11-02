use log::error;

use self::pattern::PatternAssign;
use self::pattern::PatternMatch;
use self::pattern::PatternParse;
use super::DynPrimFn;
use super::FreeImpl;
use super::MutImpl;
use super::const_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::cfg::adapter::CallPrimAdapter;
use crate::cfg::adapter::SymbolAdapter;
use crate::cfg::adapter::default_adapter;
use crate::cfg::adapter::id_adapter;
use crate::cfg::adapter::pair_adapter;
use crate::cfg::adapter::prim_adapter;
use crate::cfg::exception::fail;
use crate::cfg::exception::illegal_input;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::import_adapter;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::MutFn;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Call;
use crate::type_::ConstRef;
use crate::type_::Map;
use crate::type_::Pair;

#[derive(Clone)]
pub struct CtxLib {
    pub get: ConstPrimFuncVal,
    pub set: MutPrimFuncVal,
    pub assign: MutPrimFuncVal,
    pub is_const: MutPrimFuncVal,
    pub self_: ConstPrimFuncVal,
    pub which: MutPrimFuncVal,
}

impl Default for CtxLib {
    fn default() -> Self {
        CtxLib {
            get: get(),
            set: set(),
            assign: assign(),
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
        let assign_adapter = pair_adapter(
            Map::default(),
            prim_adapter(SymbolAdapter::Literal, CallPrimAdapter::Form),
            default_adapter(),
        );
        CoreCfg::extend_adapter(cfg, &self.assign.id, assign_adapter);
        self.assign.extend(cfg);
        self.is_const.extend(cfg);
        self.self_.extend(cfg);
        CoreCfg::extend_adapter(cfg, &self.which.id, id_adapter());
        self.which.extend(cfg);
    }
}

pub fn get() -> ConstPrimFuncVal {
    DynPrimFn { id: "context.get", f: const_impl(fn_get) }.const_()
}

fn fn_get(_cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    match ctx.unwrap().ref_(input) {
        Some(val) => val.unwrap().clone(),
        None => Val::default(),
    }
}

pub fn set() -> MutPrimFuncVal {
    DynPrimFn { id: "context.set", f: mut_impl(fn_set) }.mut_()
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

pub fn assign() -> MutPrimFuncVal {
    DynPrimFn { id: "context.assign", f: mut_impl(fn_assign) }.mut_()
}

fn fn_assign(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
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
    DynPrimFn { id: "context.is_constant", f: MutImpl::new(FreeImpl::default, fn_const, fn_mut) }
        .mut_()
}

fn fn_const(_cfg: &mut Cfg, _ctx: ConstRef<Val>, _input: Val) -> Val {
    Val::Bit(Bit::true_())
}

fn fn_mut(_cfg: &mut Cfg, _ctx: &mut Val, _input: Val) -> Val {
    Val::Bit(Bit::false_())
}

pub fn self_() -> ConstPrimFuncVal {
    DynPrimFn { id: "context.self", f: const_impl(fn_self) }.const_()
}

fn fn_self(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    ctx.unwrap().clone()
}

pub fn which() -> MutPrimFuncVal {
    DynPrimFn { id: "context.which", f: mut_impl(fn_which) }.mut_()
}

fn fn_which(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
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
    let Val::Func(func) = Eval.mut_call(cfg, ctx, call.func) else {
        error!("input.second.func should be a func");
        return fail(cfg);
    };
    let Some(adapter) = import_adapter(cfg, func.id()) else {
        return fail(cfg);
    };
    let input = adapter.mut_call(cfg, ctx, call.input);
    let Some(ctx) = ctx.ref_(pair.first) else {
        error!("input.first should be a valid reference");
        return fail(cfg);
    };
    func.dyn_call(cfg, ctx, input)
}

pub(in crate::cfg) mod pattern;
