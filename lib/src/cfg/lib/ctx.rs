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
use super::ctx_put_func;
use super::free_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::lib::ctx::pattern::PatternAssign;
use crate::cfg::lib::ctx::pattern::PatternMatch;
use crate::cfg::lib::ctx::pattern::PatternParse;
use crate::cfg::mode::CallPrimMode;
use crate::cfg::mode::SymbolMode;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::MutFn;
use crate::semantics::func::Setup;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
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
    pub move_: MutPrimFuncVal,
    pub assign: MutPrimFuncVal,
    pub contract: ConstPrimFuncVal,
    pub set_contract: MutPrimFuncVal,
    pub is_null: ConstPrimFuncVal,
    pub is_const: MutPrimFuncVal,
    pub new: FreePrimFuncVal,
    pub repr: FreePrimFuncVal,
    pub reverse: FreePrimFuncVal,
    pub self_: ConstPrimFuncVal,
    pub which: MutPrimFuncVal,
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
            new: new(),
            repr: repr(),
            reverse: reverse(),
            self_: self_(),
            which: which(),
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
        self.new.extend(cfg);
        self.repr.extend(cfg);
        self.reverse.extend(cfg);
        self.self_.extend(cfg);
        self.which.extend(cfg);
    }
}

impl Library for CtxLib {
    fn prelude(&self, ctx: &mut Ctx) {
        ctx_put_func(ctx, "=", &self.assign);
        ctx_put_func(ctx, "which", &self.which);
    }
}

pub fn read() -> ConstPrimFuncVal {
    DynPrimFn { id: "context.read", f: const_impl(fn_read), mode: FuncMode::default_mode() }
        .const_()
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
    DynPrimFn { id: "context.move", f: mut_impl(fn_move), mode: FuncMode::default_mode() }.mut_()
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

pub fn contract() -> ConstPrimFuncVal {
    DynPrimFn { id: "context.contract", f: const_impl(fn_contract), mode: FuncMode::default_mode() }
        .const_()
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
    DynPrimFn {
        id: "context.set_contract",
        f: mut_impl(fn_set_contract),
        mode: FuncMode::default_mode(),
    }
    .mut_()
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
    DynPrimFn { id: "context.is_null", f: const_impl(fn_is_null), mode: FuncMode::default_mode() }
        .const_()
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

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "context.new", f: free_impl(fn_new), mode: parse_mode() }.free()
}

fn fn_new(_cfg: &mut Cfg, input: Val) -> Val {
    let Some(ctx) = parse_ctx(input) else {
        error!("parse_ctx failed");
        return Val::default();
    };
    Val::Ctx(ctx)
}

pub fn repr() -> FreePrimFuncVal {
    FreePrimFn { id: "context.represent", f: free_impl(fn_repr), mode: FuncMode::default_mode() }
        .free()
}

fn fn_repr(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Ctx(ctx) = input else {
        error!("input {input:?} should be a ctx");
        return Val::default();
    };
    generate_ctx(ctx)
}

pub fn reverse() -> FreePrimFuncVal {
    FreePrimFn { id: "context.reverse", f: free_impl(fn_reverse), mode: FuncMode::default_mode() }
        .free()
}

fn fn_reverse(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Ctx(ctx) = input else {
        error!("input {input:?} should be a ctx");
        return Val::default();
    };
    let ctx = Ctx::from(ctx);
    let reverse = ctx.reverse();
    Val::Ctx(reverse.into())
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

pub(in crate::cfg) mod repr;
