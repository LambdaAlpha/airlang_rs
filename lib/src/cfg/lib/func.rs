use log::error;

use self::repr::generate_func;
use self::repr::parse_func;
use self::repr::parse_mode;
use super::DynPrimFn;
use super::FreePrimFn;
use super::Library;
use super::const_impl;
use super::ctx_put_func;
use super::free_impl;
use super::func::repr::generate_code;
use super::func::repr::generate_ctx_access;
use super::func::repr::generate_setup;
use super::setup::default_dyn_mode;
use super::setup::default_free_mode;
use super::setup::free_mode;
use crate::cfg::CfgMod;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::FuncSetup;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::CtxVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Clone)]
pub struct FuncLib {
    pub new: FreePrimFuncVal,
    pub repr: FreePrimFuncVal,
    pub ctx_access: ConstPrimFuncVal,
    pub call_setup: ConstPrimFuncVal,
    pub solve_setup: ConstPrimFuncVal,
    pub is_primitive: ConstPrimFuncVal,
    pub id: ConstPrimFuncVal,
    pub code: ConstPrimFuncVal,
    pub ctx: ConstPrimFuncVal,
}

impl Default for FuncLib {
    fn default() -> Self {
        FuncLib {
            new: new(),
            repr: repr(),
            ctx_access: ctx_access(),
            call_setup: call_setup(),
            solve_setup: solve_setup(),
            is_primitive: is_primitive(),
            id: id(),
            code: code(),
            ctx: ctx(),
        }
    }
}

impl CfgMod for FuncLib {
    fn extend(self, cfg: &Cfg) {
        self.new.extend(cfg);
        self.repr.extend(cfg);
        self.ctx_access.extend(cfg);
        self.call_setup.extend(cfg);
        self.solve_setup.extend(cfg);
        self.is_primitive.extend(cfg);
        self.id.extend(cfg);
        self.code.extend(cfg);
        self.ctx.extend(cfg);
    }
}

impl Library for FuncLib {
    fn prelude(&self, ctx: &mut Ctx) {
        ctx_put_func(ctx, "function", &self.new);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "function.new", f: free_impl(fn_new), mode: free_mode(parse_mode()) }.free()
}

fn fn_new(_cfg: &mut Cfg, input: Val) -> Val {
    let Some(func) = parse_func(input) else {
        error!("parse func failed");
        return Val::default();
    };
    Val::Func(func)
}

pub fn repr() -> FreePrimFuncVal {
    FreePrimFn { id: "function.represent", f: free_impl(fn_repr), mode: default_free_mode() }.free()
}

fn fn_repr(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Func(func) = input else {
        error!("input {input:?} should be a function");
        return Val::default();
    };
    generate_func(func)
}

pub fn ctx_access() -> ConstPrimFuncVal {
    DynPrimFn {
        id: "function.context_access",
        f: const_impl(fn_ctx_access),
        mode: default_dyn_mode(),
    }
    .const_()
}

fn fn_ctx_access(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    let access = generate_ctx_access(func.ctx_access());
    Val::Symbol(Symbol::from_str_unchecked(access))
}

pub fn call_setup() -> ConstPrimFuncVal {
    DynPrimFn { id: "function.call_setup", f: const_impl(fn_call_setup), mode: default_dyn_mode() }
        .const_()
}

fn fn_call_setup(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    generate_setup(func.call().cloned())
}

pub fn solve_setup() -> ConstPrimFuncVal {
    DynPrimFn {
        id: "function.solve_setup",
        f: const_impl(fn_solve_setup),
        mode: default_dyn_mode(),
    }
    .const_()
}

fn fn_solve_setup(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    generate_setup(func.solve().cloned())
}

pub fn is_primitive() -> ConstPrimFuncVal {
    DynPrimFn {
        id: "function.is_primitive",
        f: const_impl(fn_is_primitive),
        mode: default_dyn_mode(),
    }
    .const_()
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
    DynPrimFn { id: "function.id", f: const_impl(fn_id), mode: default_dyn_mode() }.const_()
}

fn fn_id(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    Val::Symbol(func.id())
}

pub fn code() -> ConstPrimFuncVal {
    DynPrimFn { id: "function.code", f: const_impl(fn_code), mode: default_dyn_mode() }.const_()
}

fn fn_code(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    generate_code(func)
}

pub fn ctx() -> ConstPrimFuncVal {
    DynPrimFn { id: "function.context", f: const_impl(fn_ctx), mode: default_dyn_mode() }.const_()
}

fn fn_ctx(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    let Some(ctx) = func.ctx() else {
        error!("func {func:?} should have an inner ctx");
        return Val::default();
    };
    Val::Ctx(CtxVal::from(ctx.clone()))
}

mod repr;
