use std::mem::swap;

use log::error;

use super::DynFn;
use super::FreeFn;
use super::MutStaticImpl;
use super::Prelude;
use super::PreludeCtx;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use super::setup::default_dyn_mode;
use super::setup::free_mode;
use crate::prelude::mode::FuncMode;
use crate::prelude::mode::Mode;
use crate::prelude::mode::SymbolMode;
use crate::prelude::utils::map_remove;
use crate::semantics::core::TaskApply;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::syntax::CALL;
use crate::syntax::SOLVE;
use crate::type_::Action;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Map;
use crate::type_::Task;

#[derive(Clone)]
pub struct TaskPrelude {
    pub new_call: FreeStaticPrimFuncVal,
    pub new_solve: FreeStaticPrimFuncVal,
    pub apply: MutStaticPrimFuncVal,
    pub is_solve: ConstStaticPrimFuncVal,
    pub func: ConstStaticPrimFuncVal,
    pub set_func: MutStaticPrimFuncVal,
    pub ctx: ConstStaticPrimFuncVal,
    pub set_ctx: MutStaticPrimFuncVal,
    pub input: ConstStaticPrimFuncVal,
    pub set_input: MutStaticPrimFuncVal,
}

impl Default for TaskPrelude {
    fn default() -> Self {
        TaskPrelude {
            new_call: new_call(),
            new_solve: new_solve(),
            apply: apply(),
            is_solve: is_solve(),
            func: func(),
            set_func: set_func(),
            ctx: ctx(),
            set_ctx: set_ctx(),
            input: input(),
            set_input: set_input(),
        }
    }
}

impl Prelude for TaskPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.new_call.put(ctx);
        self.new_solve.put(ctx);
        self.apply.put(ctx);
        self.is_solve.put(ctx);
        self.func.put(ctx);
        self.set_func.put(ctx);
        self.ctx.put(ctx);
        self.set_ctx.put(ctx);
        self.input.put(ctx);
        self.set_input.put(ctx);
    }
}

const FUNC: &str = "function";
const CTX: &str = "context";
const INPUT: &str = "input";

fn parse_mode() -> Option<Mode> {
    FuncMode::map_mode(
        Map::default(),
        FuncMode::symbol_mode(SymbolMode::Literal),
        FuncMode::default_mode(),
    )
}

pub fn new_call() -> FreeStaticPrimFuncVal {
    FreeFn { id: CALL, f: free_impl(fn_new_call), mode: free_mode(parse_mode()) }.free_static()
}

fn fn_new_call(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        error!("input {input:?} should be a map");
        return Val::default();
    };
    let func = map_remove(&mut map, FUNC);
    let ctx = map_remove(&mut map, CTX);
    let input = map_remove(&mut map, INPUT);
    let task = Task { action: Action::Call, func, ctx, input };
    Val::Task(task.into())
}

pub fn new_solve() -> FreeStaticPrimFuncVal {
    FreeFn { id: SOLVE, f: free_impl(fn_new_solve), mode: free_mode(parse_mode()) }.free_static()
}

fn fn_new_solve(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        error!("input {input:?} should be a map");
        return Val::default();
    };
    let func = map_remove(&mut map, FUNC);
    let ctx = map_remove(&mut map, CTX);
    let input = map_remove(&mut map, INPUT);
    let task = Task { action: Action::Solve, func, ctx, input };
    Val::Task(task.into())
}

pub fn apply() -> MutStaticPrimFuncVal {
    DynFn {
        id: "task.apply",
        f: MutStaticImpl::new(fn_apply_free, fn_apply_const, fn_apply_mut),
        mode: default_dyn_mode(),
    }
    .mut_static()
}

fn fn_apply_free(input: Val) -> Val {
    let Val::Task(task) = input else {
        return Val::default();
    };
    TaskApply.free_static_call(task)
}

fn fn_apply_const(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Task(task) = input else {
        return Val::default();
    };
    TaskApply.const_static_call(ctx, task)
}

fn fn_apply_mut(ctx: &mut Val, input: Val) -> Val {
    let Val::Task(task) = input else {
        return Val::default();
    };
    TaskApply.mut_static_call(ctx, task)
}

pub fn is_solve() -> ConstStaticPrimFuncVal {
    DynFn { id: "task.is_solve", f: const_impl(fn_is_solve), mode: default_dyn_mode() }
        .const_static()
}

fn fn_is_solve(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Task(task) = &*ctx else {
        return Val::default();
    };
    Val::Bit(Bit::from(matches!(task.action, Action::Solve)))
}

pub fn func() -> ConstStaticPrimFuncVal {
    DynFn { id: "task.function", f: const_impl(fn_func), mode: default_dyn_mode() }.const_static()
}

fn fn_func(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Task(task) = &*ctx else {
        return Val::default();
    };
    task.func.clone()
}

pub fn set_func() -> MutStaticPrimFuncVal {
    DynFn { id: "task.set_function", f: mut_impl(fn_set_func), mode: default_dyn_mode() }
        .mut_static()
}

fn fn_set_func(ctx: &mut Val, mut input: Val) -> Val {
    let Val::Task(task) = ctx else {
        return Val::default();
    };
    swap(&mut task.func, &mut input);
    input
}

pub fn ctx() -> ConstStaticPrimFuncVal {
    DynFn { id: "task.context", f: const_impl(fn_ctx), mode: default_dyn_mode() }.const_static()
}

fn fn_ctx(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Task(task) = &*ctx else {
        return Val::default();
    };
    task.ctx.clone()
}

pub fn set_ctx() -> MutStaticPrimFuncVal {
    DynFn { id: "task.set_context", f: mut_impl(fn_set_ctx), mode: default_dyn_mode() }.mut_static()
}

fn fn_set_ctx(ctx: &mut Val, mut input: Val) -> Val {
    let Val::Task(task) = ctx else {
        return Val::default();
    };
    swap(&mut task.ctx, &mut input);
    input
}

pub fn input() -> ConstStaticPrimFuncVal {
    DynFn { id: "task.input", f: const_impl(fn_input), mode: default_dyn_mode() }.const_static()
}

fn fn_input(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Task(task) = &*ctx else {
        return Val::default();
    };
    task.input.clone()
}

pub fn set_input() -> MutStaticPrimFuncVal {
    DynFn { id: "task.set_input", f: mut_impl(fn_set_input), mode: default_dyn_mode() }.mut_static()
}

fn fn_set_input(ctx: &mut Val, mut input: Val) -> Val {
    let Val::Task(task) = ctx else {
        return Val::default();
    };
    swap(&mut task.input, &mut input);
    input
}
