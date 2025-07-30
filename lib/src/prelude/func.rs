use log::error;

use self::repr::generate_func;
use self::repr::parse_func;
use self::repr::parse_mode;
use super::DynFn;
use super::FreeFn;
use super::MutStaticImpl;
use super::Prelude;
use super::PreludeCtx;
use super::const_impl;
use super::free_impl;
use super::func::repr::generate_code;
use super::func::repr::generate_ctx_access;
use super::func::repr::generate_setup;
use super::setup::default_dyn_mode;
use super::setup::default_free_mode;
use super::setup::dyn_mode;
use super::setup::free_mode;
use crate::prelude::mode::CodeMode;
use crate::prelude::mode::FuncMode;
use crate::prelude::mode::SymbolMode;
use crate::semantics::core::Eval;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::FuncSetup;
use crate::semantics::func::MutStaticFn;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::CtxVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Clone)]
pub struct FuncPrelude {
    pub new: FreeStaticPrimFuncVal,
    pub repr: FreeStaticPrimFuncVal,
    pub eval: MutStaticPrimFuncVal,
    pub ctx_access: ConstStaticPrimFuncVal,
    pub call_setup: ConstStaticPrimFuncVal,
    pub solve_setup: ConstStaticPrimFuncVal,
    pub is_primitive: ConstStaticPrimFuncVal,
    pub is_cell: ConstStaticPrimFuncVal,
    pub id: ConstStaticPrimFuncVal,
    pub code: ConstStaticPrimFuncVal,
    pub ctx: ConstStaticPrimFuncVal,
}

impl Default for FuncPrelude {
    fn default() -> Self {
        FuncPrelude {
            new: new(),
            repr: repr(),
            eval: eval(),
            ctx_access: ctx_access(),
            call_setup: call_setup(),
            solve_setup: solve_setup(),
            is_primitive: is_primitive(),
            is_cell: is_cell(),
            id: id(),
            code: code(),
            ctx: ctx(),
        }
    }
}

impl Prelude for FuncPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.new.put(ctx);
        self.repr.put(ctx);
        self.eval.put(ctx);
        self.ctx_access.put(ctx);
        self.call_setup.put(ctx);
        self.solve_setup.put(ctx);
        self.is_primitive.put(ctx);
        self.is_cell.put(ctx);
        self.id.put(ctx);
        self.code.put(ctx);
        self.ctx.put(ctx);
    }
}

pub fn new() -> FreeStaticPrimFuncVal {
    FreeFn { id: "function", f: free_impl(fn_new), mode: free_mode(parse_mode()) }.free_static()
}

fn fn_new(input: Val) -> Val {
    let Some(func) = parse_func(input) else {
        error!("parse func failed");
        return Val::default();
    };
    Val::Func(func)
}

pub fn repr() -> FreeStaticPrimFuncVal {
    FreeFn { id: "function.represent", f: free_impl(fn_repr), mode: default_free_mode() }
        .free_static()
}

fn fn_repr(input: Val) -> Val {
    let Val::Func(func) = input else {
        error!("input {input:?} should be a function");
        return Val::default();
    };
    generate_func(func)
}

pub fn eval() -> MutStaticPrimFuncVal {
    DynFn {
        id: "eval",
        f: MutStaticImpl::new(fn_eval_free, fn_eval_const, fn_eval_mut),
        mode: dyn_mode(FuncMode::prim_mode(SymbolMode::Ref, CodeMode::Form)),
    }
    .mut_static()
}

fn fn_eval_free(input: Val) -> Val {
    Eval.free_static_call(input)
}

fn fn_eval_const(ctx: ConstRef<Val>, input: Val) -> Val {
    Eval.const_static_call(ctx, input)
}

fn fn_eval_mut(ctx: &mut Val, input: Val) -> Val {
    Eval.mut_static_call(ctx, input)
}

pub fn ctx_access() -> ConstStaticPrimFuncVal {
    DynFn { id: "function.context_access", f: const_impl(fn_ctx_access), mode: default_dyn_mode() }
        .const_static()
}

fn fn_ctx_access(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    let access = generate_ctx_access(func.ctx_access());
    Val::Symbol(Symbol::from_str_unchecked(access))
}

pub fn call_setup() -> ConstStaticPrimFuncVal {
    DynFn { id: "function.call_setup", f: const_impl(fn_call_setup), mode: default_dyn_mode() }
        .const_static()
}

fn fn_call_setup(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    generate_setup(func.call().cloned())
}

pub fn solve_setup() -> ConstStaticPrimFuncVal {
    DynFn { id: "function.solve_setup", f: const_impl(fn_solve_setup), mode: default_dyn_mode() }
        .const_static()
}

fn fn_solve_setup(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    generate_setup(func.solve().cloned())
}

pub fn is_primitive() -> ConstStaticPrimFuncVal {
    DynFn { id: "function.is_primitive", f: const_impl(fn_is_primitive), mode: default_dyn_mode() }
        .const_static()
}

fn fn_is_primitive(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    let is_primitive = func.is_primitive();
    Val::Bit(Bit::from(is_primitive))
}

pub fn is_cell() -> ConstStaticPrimFuncVal {
    DynFn { id: "function.is_cell", f: const_impl(fn_is_cell), mode: default_dyn_mode() }
        .const_static()
}

fn fn_is_cell(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    Val::Bit(Bit::from(func.is_cell()))
}

pub fn id() -> ConstStaticPrimFuncVal {
    DynFn { id: "function.id", f: const_impl(fn_id), mode: default_dyn_mode() }.const_static()
}

fn fn_id(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    Val::Symbol(func.id())
}

pub fn code() -> ConstStaticPrimFuncVal {
    DynFn { id: "function.code", f: const_impl(fn_code), mode: default_dyn_mode() }.const_static()
}

fn fn_code(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    generate_code(func)
}

pub fn ctx() -> ConstStaticPrimFuncVal {
    DynFn { id: "function.context", f: const_impl(fn_ctx), mode: default_dyn_mode() }.const_static()
}

fn fn_ctx(ctx: ConstRef<Val>, _input: Val) -> Val {
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
