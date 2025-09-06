use log::error;
use pattern::PatternAssign;
use pattern::PatternMatch;
use pattern::PatternParse;

use super::DynPrimFn;
use super::FreeImpl;
use super::FuncMode;
use super::Library;
use super::MutImpl;
use super::const_impl;
use super::memo_put_func;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::mode::CallPrimMode;
use crate::cfg::mode::SymbolMode;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::MutFn;
use crate::semantics::func::Setup;
use crate::semantics::memo::Memo;
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
    pub read: ConstPrimFuncVal,
    pub assign: MutPrimFuncVal,
    pub is_const: MutPrimFuncVal,
    pub self_: ConstPrimFuncVal,
    pub which: MutPrimFuncVal,
}

impl Default for CtxLib {
    fn default() -> Self {
        CtxLib {
            read: read(),
            assign: assign(),
            is_const: is_const(),
            self_: self_(),
            which: which(),
        }
    }
}

impl CfgMod for CtxLib {
    fn extend(self, cfg: &Cfg) {
        self.read.extend(cfg);
        self.assign.extend(cfg);
        self.is_const.extend(cfg);
        self.self_.extend(cfg);
        self.which.extend(cfg);
    }
}

impl Library for CtxLib {
    fn prelude(&self, memo: &mut Memo) {
        memo_put_func(memo, "=", &self.assign);
        memo_put_func(memo, "which", &self.which);
    }
}

pub fn read() -> ConstPrimFuncVal {
    DynPrimFn { id: "context.read", f: const_impl(fn_read), mode: FuncMode::default_mode() }
        .const_()
}

fn fn_read(_cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Memo(memo) = &*ctx else {
        error!("ctx {ctx:?} should be a memo");
        return Val::default();
    };
    let Val::Symbol(s) = input else {
        error!("input {input:?} should be a symbol");
        return Val::default();
    };
    memo.get_ref(s).cloned().unwrap_or_default()
}

pub fn assign() -> MutPrimFuncVal {
    DynPrimFn {
        id: "context.assign",
        f: mut_impl(fn_assign),
        mode: FuncMode::pair_mode(
            Map::default(),
            FuncMode::prim_mode(SymbolMode::Literal, CallPrimMode::Form),
            FuncMode::default_mode(),
        ),
    }
    .mut_()
}

fn fn_assign(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Some(pattern) = pair.first.parse() else {
        error!("parse pattern failed");
        return Val::default();
    };
    let val = pair.second;
    if pattern.match_(&val) { pattern.assign(ctx, val) } else { Val::default() }
}

pub fn is_const() -> MutPrimFuncVal {
    DynPrimFn {
        id: "context.is_constant",
        f: MutImpl::new(FreeImpl::default, fn_const, fn_mut),
        mode: FuncMode::default_mode(),
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
    DynPrimFn { id: "context.self", f: const_impl(fn_self), mode: FuncMode::default_mode() }
        .const_()
}

fn fn_self(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    ctx.unwrap().clone()
}

pub fn which() -> MutPrimFuncVal {
    DynPrimFn { id: "context.which", f: mut_impl(fn_which), mode: FuncMode::id_mode() }.mut_()
}

fn fn_which(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input: {:?} should be a pair", input);
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Call(call) = pair.second else {
        error!("input.second {:?} should be a call", pair.second);
        return Val::default();
    };
    let call = Call::from(call);
    let Val::Func(func) = Eval.mut_call(cfg, ctx, call.func) else {
        error!("input.second.func should be a func");
        return Val::default();
    };
    let input = func.setup().mut_call(cfg, ctx, call.input);
    let Some(ctx) = ctx.ref_(pair.first) else {
        error!("input.first should be a valid reference");
        return Val::default();
    };
    func.dyn_call(cfg, ctx, input)
}

pub(in crate::cfg) mod pattern;
