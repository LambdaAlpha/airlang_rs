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
use crate::cfg::error::illegal_ctx;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Key;
use crate::type_::Pair;

#[derive(Clone)]
pub struct FuncLib {
    pub new: FreePrimFuncVal,
    pub represent: FreePrimFuncVal,
    pub apply: MutPrimFuncVal,
    pub get_context_access: ConstPrimFuncVal,
    pub get_raw_input: ConstPrimFuncVal,
    pub is_primitive: ConstPrimFuncVal,
    pub get_code: ConstPrimFuncVal,
    pub get_context: ConstPrimFuncVal,
}

impl Default for FuncLib {
    fn default() -> Self {
        FuncLib {
            new: new(),
            represent: represent(),
            apply: apply(),
            get_context_access: get_context_access(),
            get_raw_input: get_raw_input(),
            is_primitive: is_primitive(),
            get_code: get_code(),
            get_context: get_context(),
        }
    }
}

impl CfgMod for FuncLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_function.new", self.new);
        extend_func(cfg, "_function.represent", self.represent);
        extend_func(cfg, "_function.apply", self.apply);
        extend_func(cfg, "_function.get_context_access", self.get_context_access);
        extend_func(cfg, "_function.get_raw_input", self.get_raw_input);
        extend_func(cfg, "_function.is_primitive", self.is_primitive);
        extend_func(cfg, "_function.get_code", self.get_code);
        extend_func(cfg, "_function.get_context", self.get_context);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_new) }.free()
}

fn fn_new(cfg: &mut Cfg, input: Val) -> Val {
    let Some(func) = parse_func(input) else {
        error!("parse func failed");
        return illegal_input(cfg);
    };
    Val::Func(func)
}

pub fn represent() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_represent) }.free()
}

fn fn_represent(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(func) = input else {
        error!("input {input:?} should be a function");
        return illegal_input(cfg);
    };
    generate_func(func)
}

pub fn apply() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: MutImpl::new(fn_apply_free, fn_apply_const, fn_apply_mut) }
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

pub fn get_context_access() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_context_access) }.const_()
}

fn fn_get_context_access(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let access = generate_ctx_access(func.ctx_access());
    Val::Key(Key::from_str_unchecked(access))
}

pub fn get_raw_input() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_raw_input) }.const_()
}

fn fn_get_raw_input(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    Val::Bit(Bit::from(func.raw_input()))
}

pub fn is_primitive() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_is_primitive) }.const_()
}

fn fn_is_primitive(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let is_primitive = func.is_primitive();
    Val::Bit(Bit::from(is_primitive))
}

pub fn get_code() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_code) }.const_()
}

fn fn_get_code(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    generate_code(func)
}

pub fn get_context() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_context) }.const_()
}

fn fn_get_context(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let Some(ctx) = func.ctx() else {
        error!("func {func:?} should have an inner context");
        return illegal_ctx(cfg);
    };
    ctx.clone()
}

mod repr;
