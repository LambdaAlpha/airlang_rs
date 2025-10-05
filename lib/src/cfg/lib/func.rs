use log::error;

use self::repr::generate_func;
use self::repr::parse_func;
use super::DynPrimFn;
use super::FreePrimFn;
use super::const_impl;
use super::free_impl;
use super::func::repr::generate_code;
use super::func::repr::generate_ctx_access;
use super::func::repr::parse_adapter;
use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MemoVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Clone)]
pub struct FuncLib {
    pub new: FreePrimFuncVal,
    pub repr: FreePrimFuncVal,
    pub ctx_access: ConstPrimFuncVal,
    pub is_primitive: ConstPrimFuncVal,
    pub id: ConstPrimFuncVal,
    pub code: ConstPrimFuncVal,
    pub memo: ConstPrimFuncVal,
}

impl Default for FuncLib {
    fn default() -> Self {
        FuncLib {
            new: new(),
            repr: repr(),
            ctx_access: ctx_access(),
            is_primitive: is_primitive(),
            id: id(),
            code: code(),
            memo: memo(),
        }
    }
}

impl CfgMod for FuncLib {
    fn extend(self, cfg: &Cfg) {
        CoreCfg::extend_adapter(cfg, &self.new.id, parse_adapter());
        self.new.extend(cfg);
        self.repr.extend(cfg);
        self.ctx_access.extend(cfg);
        self.is_primitive.extend(cfg);
        self.id.extend(cfg);
        self.code.extend(cfg);
        self.memo.extend(cfg);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "function.new", f: free_impl(fn_new) }.free()
}

fn fn_new(_cfg: &mut Cfg, input: Val) -> Val {
    let Some(func) = parse_func(input) else {
        error!("parse func failed");
        return Val::default();
    };
    Val::Func(func)
}

pub fn repr() -> FreePrimFuncVal {
    FreePrimFn { id: "function.represent", f: free_impl(fn_repr) }.free()
}

fn fn_repr(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(func) = input else {
        error!("input {input:?} should be a function");
        return Val::default();
    };
    generate_func(func)
}

pub fn ctx_access() -> ConstPrimFuncVal {
    DynPrimFn { id: "function.context_access", f: const_impl(fn_ctx_access) }.const_()
}

fn fn_ctx_access(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    let access = generate_ctx_access(func.ctx_access());
    Val::Symbol(Symbol::from_str_unchecked(access))
}

pub fn is_primitive() -> ConstPrimFuncVal {
    DynPrimFn { id: "function.is_primitive", f: const_impl(fn_is_primitive) }.const_()
}

fn fn_is_primitive(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    let is_primitive = func.is_primitive();
    Val::Bit(Bit::from(is_primitive))
}

pub fn id() -> ConstPrimFuncVal {
    DynPrimFn { id: "function.id", f: const_impl(fn_id) }.const_()
}

fn fn_id(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    Val::Symbol(func.id())
}

pub fn code() -> ConstPrimFuncVal {
    DynPrimFn { id: "function.code", f: const_impl(fn_code) }.const_()
}

fn fn_code(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    generate_code(func)
}

pub fn memo() -> ConstPrimFuncVal {
    DynPrimFn { id: "function.memory", f: const_impl(fn_memo) }.const_()
}

fn fn_memo(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    let Some(memo) = func.memo() else {
        error!("func {func:?} should have an inner memory");
        return Val::default();
    };
    Val::Memo(MemoVal::from(memo.clone()))
}

mod repr;
