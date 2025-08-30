use log::error;

use self::repr::generate_func;
use self::repr::parse_func;
use self::repr::parse_mode;
use super::DynPrimFn;
use super::FreePrimFn;
use super::MutImpl;
use super::Prelude;
use super::const_impl;
use super::free_impl;
use super::func::repr::generate_code;
use super::func::repr::generate_ctx_access;
use super::func::repr::generate_setup;
use super::setup::default_dyn_mode;
use super::setup::default_free_mode;
use super::setup::dyn_mode;
use super::setup::free_mode;
use crate::cfg::prelude::mode::FuncMode;
use crate::cfg::prelude::mode::SymbolMode;
use crate::cfg::prelude::mode::TaskPrimMode;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::FuncSetup;
use crate::semantics::func::MutFn;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::CtxVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Clone)]
pub struct FuncPrelude {
    pub new: FreePrimFuncVal,
    pub repr: FreePrimFuncVal,
    pub apply: MutPrimFuncVal,
    pub ctx_access: ConstPrimFuncVal,
    pub call_setup: ConstPrimFuncVal,
    pub solve_setup: ConstPrimFuncVal,
    pub is_primitive: ConstPrimFuncVal,
    pub id: ConstPrimFuncVal,
    pub code: ConstPrimFuncVal,
    pub ctx: ConstPrimFuncVal,
}

impl Default for FuncPrelude {
    fn default() -> Self {
        FuncPrelude {
            new: new(),
            repr: repr(),
            apply: apply(),
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

impl Prelude for FuncPrelude {
    fn put(self, ctx: &mut Ctx) {
        self.new.put(ctx);
        self.repr.put(ctx);
        self.apply.put(ctx);
        self.ctx_access.put(ctx);
        self.call_setup.put(ctx);
        self.solve_setup.put(ctx);
        self.is_primitive.put(ctx);
        self.id.put(ctx);
        self.code.put(ctx);
        self.ctx.put(ctx);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "function", f: free_impl(fn_new), mode: free_mode(parse_mode()) }.free()
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

pub fn apply() -> MutPrimFuncVal {
    DynPrimFn {
        id: "apply",
        f: MutImpl::new(fn_eval_free, fn_eval_const, fn_eval_mut),
        mode: dyn_mode(FuncMode::prim_mode(SymbolMode::Ref, TaskPrimMode::Form)),
    }
    .mut_()
}

fn fn_eval_free(cfg: &mut Cfg, input: Val) -> Val {
    Eval.free_call(cfg, input)
}

fn fn_eval_const(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    Eval.const_call(cfg, ctx, input)
}

fn fn_eval_mut(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    Eval.mut_call(cfg, ctx, input)
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
