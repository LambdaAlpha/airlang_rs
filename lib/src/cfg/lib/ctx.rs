use log::error;

use self::repr::generate_contract;
use self::repr::generate_ctx;
use self::repr::parse_contract;
use self::repr::parse_ctx;
use self::repr::parse_mode;
use super::DynPrimFn;
use super::FreeImpl;
use super::FreePrimFn;
use super::FuncMode;
use super::Library;
use super::MutImpl;
use super::const_impl;
use super::free_impl;
use super::mode::SymbolMode;
use super::mode::TaskPrimMode;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::lib::ctx::pattern::PatternAssign;
use crate::cfg::lib::ctx::pattern::PatternMatch;
use crate::cfg::lib::ctx::pattern::PatternParse;
use crate::cfg::lib::setup::default_dyn_mode;
use crate::cfg::lib::setup::default_free_mode;
use crate::cfg::lib::setup::dyn_mode;
use crate::cfg::lib::setup::free_mode;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Map;
use crate::type_::Pair;

#[derive(Clone)]
pub struct CtxLib {
    pub read: ConstPrimFuncVal,
    pub move_: MutPrimFuncVal,
    pub assign: MutPrimFuncVal,
    pub contract: ConstPrimFuncVal,
    pub set_contract: MutPrimFuncVal,
    pub is_null: ConstPrimFuncVal,
    pub is_const: MutPrimFuncVal,
    pub ctx_new: FreePrimFuncVal,
    pub ctx_repr: FreePrimFuncVal,
    pub ctx_reverse: FreePrimFuncVal,
    pub ctx_self: ConstPrimFuncVal,
}

impl Default for CtxLib {
    fn default() -> Self {
        CtxLib {
            read: read(),
            move_: move_(),
            assign: assign(),
            contract: contract(),
            set_contract: set_contract(),
            is_null: is_null(),
            is_const: is_const(),
            ctx_new: ctx_new(),
            ctx_repr: ctx_repr(),
            ctx_reverse: ctx_reverse(),
            ctx_self: ctx_self(),
        }
    }
}

impl CfgMod for CtxLib {
    fn extend(self, cfg: &Cfg) {
        self.read.extend(cfg);
        self.move_.extend(cfg);
        self.assign.extend(cfg);
        self.contract.extend(cfg);
        self.set_contract.extend(cfg);
        self.is_null.extend(cfg);
        self.is_const.extend(cfg);
        self.ctx_new.extend(cfg);
        self.ctx_repr.extend(cfg);
        self.ctx_reverse.extend(cfg);
        self.ctx_self.extend(cfg);
    }
}

impl Library for CtxLib {
    fn prelude(&self, ctx: &mut Ctx) {
        self.read.prelude(ctx);
        self.move_.prelude(ctx);
        self.assign.prelude(ctx);
        self.contract.prelude(ctx);
        self.set_contract.prelude(ctx);
        self.is_null.prelude(ctx);
        self.is_const.prelude(ctx);
        self.ctx_new.prelude(ctx);
        self.ctx_repr.prelude(ctx);
        self.ctx_reverse.prelude(ctx);
        self.ctx_self.prelude(ctx);
    }
}

pub fn read() -> ConstPrimFuncVal {
    DynPrimFn { id: "read", f: const_impl(fn_read), mode: default_dyn_mode() }.const_()
}

fn fn_read(_cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Ctx(ctx) = &*ctx else {
        error!("ctx {ctx:?} should be a ctx");
        return Val::default();
    };
    let Val::Symbol(s) = input else {
        error!("input {input:?} should be a symbol");
        return Val::default();
    };
    ctx.get_ref(s).cloned().unwrap_or_default()
}

pub fn move_() -> MutPrimFuncVal {
    DynPrimFn { id: "move", f: mut_impl(fn_move), mode: default_dyn_mode() }.mut_()
}

fn fn_move(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Ctx(ctx) = ctx else {
        error!("ctx {ctx:?} should be a ctx");
        return Val::default();
    };
    let Val::Symbol(s) = input else {
        error!("input {input:?} should be a symbol");
        return Val::default();
    };
    ctx.remove(s).unwrap_or_default()
}

pub fn assign() -> MutPrimFuncVal {
    DynPrimFn {
        id: "=",
        f: mut_impl(fn_assign),
        mode: dyn_mode(FuncMode::pair_mode(
            Map::default(),
            FuncMode::prim_mode(SymbolMode::Literal, TaskPrimMode::Form),
            FuncMode::default_mode(),
        )),
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

pub fn contract() -> ConstPrimFuncVal {
    DynPrimFn { id: "contract", f: const_impl(fn_contract), mode: default_dyn_mode() }.const_()
}

fn fn_contract(_cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Ctx(ctx) = &*ctx else {
        error!("ctx {ctx:?} should be a ctx");
        return Val::default();
    };
    let Val::Symbol(s) = input else {
        error!("input {input:?} should be a symbol");
        return Val::default();
    };
    let Some(contract) = ctx.get_contract(s.clone()) else {
        error!("variable {s:?} should exist");
        return Val::default();
    };
    generate_contract(contract)
}

pub fn set_contract() -> MutPrimFuncVal {
    DynPrimFn { id: "set_contract", f: mut_impl(fn_set_contract), mode: default_dyn_mode() }.mut_()
}

fn fn_set_contract(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Ctx(ctx) = ctx else {
        error!("ctx {ctx:?} should be a ctx");
        return Val::default();
    };
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Symbol(s) = pair.first else {
        error!("input.first {:?} should be a symbol", pair.first);
        return Val::default();
    };
    let Some(contract) = parse_contract(&pair.second) else {
        error!("parse contract failed");
        return Val::default();
    };
    let _ = ctx.set_contract(s, contract);
    Val::default()
}

pub fn is_null() -> ConstPrimFuncVal {
    DynPrimFn { id: "is_null", f: const_impl(fn_is_null), mode: default_dyn_mode() }.const_()
}

fn fn_is_null(_cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Ctx(ctx) = &*ctx else {
        error!("ctx {ctx:?} should be a ctx");
        return Val::default();
    };
    let Val::Symbol(s) = input else {
        error!("input {input:?} should be a symbol");
        return Val::default();
    };
    Val::Bit(Bit::from(ctx.is_null(s)))
}

pub fn is_const() -> MutPrimFuncVal {
    DynPrimFn {
        id: "is_constant",
        f: MutImpl::new(FreeImpl::default, fn_const, fn_mut),
        mode: default_dyn_mode(),
    }
    .mut_()
}

fn fn_const(_cfg: &mut Cfg, _ctx: ConstRef<Val>, _input: Val) -> Val {
    Val::Bit(Bit::true_())
}

fn fn_mut(_cfg: &mut Cfg, _ctx: &mut Val, _input: Val) -> Val {
    Val::Bit(Bit::false_())
}

pub fn ctx_new() -> FreePrimFuncVal {
    FreePrimFn { id: "context", f: free_impl(fn_ctx_new), mode: free_mode(parse_mode()) }.free()
}

fn fn_ctx_new(_cfg: &mut Cfg, input: Val) -> Val {
    let Some(ctx) = parse_ctx(input) else {
        error!("parse_ctx failed");
        return Val::default();
    };
    Val::Ctx(ctx)
}

pub fn ctx_repr() -> FreePrimFuncVal {
    FreePrimFn { id: "context.represent", f: free_impl(fn_ctx_repr), mode: default_free_mode() }
        .free()
}

fn fn_ctx_repr(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Ctx(ctx) = input else {
        error!("input {input:?} should be a ctx");
        return Val::default();
    };
    generate_ctx(ctx)
}

pub fn ctx_reverse() -> FreePrimFuncVal {
    FreePrimFn { id: "context.reverse", f: free_impl(fn_ctx_reverse), mode: default_free_mode() }
        .free()
}

fn fn_ctx_reverse(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Ctx(ctx) = input else {
        error!("input {input:?} should be a ctx");
        return Val::default();
    };
    let ctx = Ctx::from(ctx);
    let reverse = ctx.reverse();
    Val::Ctx(reverse.into())
}

pub fn ctx_self() -> ConstPrimFuncVal {
    DynPrimFn { id: "self", f: const_impl(fn_ctx_self), mode: default_dyn_mode() }.const_()
}

fn fn_ctx_self(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    ctx.unwrap().clone()
}

pub(in crate::cfg) mod pattern;

pub(in crate::cfg) mod repr;
