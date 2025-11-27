use log::error;

use self::repr::generate_func;
use self::repr::parse_func;
use super::DynPrimFn;
use super::FreePrimFn;
use super::MutImpl;
use super::const_impl;
use super::free_impl;
use super::func::repr::generate_code;
use super::func::repr::generate_ctx_access;
use crate::cfg::CfgMod;
use crate::cfg::exception::illegal_ctx;
use crate::cfg::exception::illegal_input;
use crate::semantics::cfg::Cfg;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MemoVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Key;
use crate::type_::Pair;

#[derive(Clone)]
pub struct FuncLib {
    pub new: FreePrimFuncVal,
    pub repr: FreePrimFuncVal,
    pub apply: MutPrimFuncVal,
    pub ctx_access: ConstPrimFuncVal,
    pub raw_input: ConstPrimFuncVal,
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
            apply: apply(),
            ctx_access: ctx_access(),
            raw_input: raw_input(),
            is_primitive: is_primitive(),
            id: id(),
            code: code(),
            memo: memo(),
        }
    }
}

impl CfgMod for FuncLib {
    fn extend(self, cfg: &Cfg) {
        self.new.extend(cfg);
        self.repr.extend(cfg);
        self.apply.extend(cfg);
        self.ctx_access.extend(cfg);
        self.raw_input.extend(cfg);
        self.is_primitive.extend(cfg);
        self.id.extend(cfg);
        self.code.extend(cfg);
        self.memo.extend(cfg);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "_function.new", raw_input: false, f: free_impl(fn_new) }.free()
}

fn fn_new(cfg: &mut Cfg, input: Val) -> Val {
    let Some(func) = parse_func(input) else {
        error!("parse func failed");
        return illegal_input(cfg);
    };
    Val::Func(func)
}

pub fn repr() -> FreePrimFuncVal {
    FreePrimFn { id: "_function.represent", raw_input: false, f: free_impl(fn_repr) }.free()
}

fn fn_repr(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(func) = input else {
        error!("input {input:?} should be a function");
        return illegal_input(cfg);
    };
    generate_func(func)
}

pub fn apply() -> MutPrimFuncVal {
    DynPrimFn {
        id: "_function.apply",
        raw_input: false,
        f: MutImpl::new(fn_apply_free, fn_apply_const, fn_apply_mut),
    }
    .mut_()
}

fn fn_apply_free(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Func(func) = pair.first else {
        error!("input.first {:?} should be a func", pair.first);
        return illegal_input(cfg);
    };
    func.free_call(cfg, pair.second)
}

fn fn_apply_const(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Func(func) = pair.first else {
        error!("input.first {:?} should be a func", pair.first);
        return illegal_input(cfg);
    };
    func.const_call(cfg, ctx, pair.second)
}

fn fn_apply_mut(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Func(func) = pair.first else {
        error!("input.first {:?} should be a func", pair.first);
        return illegal_input(cfg);
    };
    func.mut_call(cfg, ctx, pair.second)
}

pub fn ctx_access() -> ConstPrimFuncVal {
    DynPrimFn { id: "_function.context_access", raw_input: false, f: const_impl(fn_ctx_access) }
        .const_()
}

fn fn_ctx_access(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return illegal_ctx(cfg);
    };
    let access = generate_ctx_access(func.ctx_access());
    Val::Key(Key::from_str_unchecked(access))
}

pub fn raw_input() -> ConstPrimFuncVal {
    DynPrimFn { id: "_function.raw_input", raw_input: false, f: const_impl(fn_raw_input) }.const_()
}

fn fn_raw_input(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return illegal_ctx(cfg);
    };
    Val::Bit(Bit::from(func.raw_input()))
}

pub fn is_primitive() -> ConstPrimFuncVal {
    DynPrimFn { id: "_function.is_primitive", raw_input: false, f: const_impl(fn_is_primitive) }
        .const_()
}

fn fn_is_primitive(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return illegal_ctx(cfg);
    };
    let is_primitive = func.is_primitive();
    Val::Bit(Bit::from(is_primitive))
}

pub fn id() -> ConstPrimFuncVal {
    DynPrimFn { id: "_function.id", raw_input: false, f: const_impl(fn_id) }.const_()
}

fn fn_id(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return illegal_ctx(cfg);
    };
    Val::Key(func.id())
}

pub fn code() -> ConstPrimFuncVal {
    DynPrimFn { id: "_function.code", raw_input: false, f: const_impl(fn_code) }.const_()
}

fn fn_code(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return illegal_ctx(cfg);
    };
    let Some(code) = generate_code(func) else {
        error!("func {func:?} should have code");
        return illegal_ctx(cfg);
    };
    code
}

pub fn memo() -> ConstPrimFuncVal {
    DynPrimFn { id: "_function.memory", raw_input: false, f: const_impl(fn_memo) }.const_()
}

fn fn_memo(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return illegal_ctx(cfg);
    };
    let Some(memo) = func.memo() else {
        error!("func {func:?} should have an inner memory");
        return illegal_ctx(cfg);
    };
    Val::Memo(MemoVal::from(memo.clone()))
}

mod repr;
