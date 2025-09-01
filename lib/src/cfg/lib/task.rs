use std::mem::swap;

use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::Library;
use super::MutImpl;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use super::setup::default_dyn_mode;
use super::setup::default_free_mode;
use crate::cfg::CfgMod;
use crate::cfg::lib::utils::map_remove;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::TaskApply;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Action;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Task;

#[derive(Clone)]
pub struct TaskLib {
    pub new_call: FreePrimFuncVal,
    pub new_solve: FreePrimFuncVal,
    pub apply: MutPrimFuncVal,
    pub is_solve: ConstPrimFuncVal,
    pub func: ConstPrimFuncVal,
    pub set_func: MutPrimFuncVal,
    pub ctx: ConstPrimFuncVal,
    pub set_ctx: MutPrimFuncVal,
    pub input: ConstPrimFuncVal,
    pub set_input: MutPrimFuncVal,
}

impl Default for TaskLib {
    fn default() -> Self {
        TaskLib {
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

impl CfgMod for TaskLib {
    fn extend(self, cfg: &Cfg) {
        self.new_call.extend(cfg);
        self.new_solve.extend(cfg);
        self.apply.extend(cfg);
        self.is_solve.extend(cfg);
        self.func.extend(cfg);
        self.set_func.extend(cfg);
        self.ctx.extend(cfg);
        self.set_ctx.extend(cfg);
        self.input.extend(cfg);
        self.set_input.extend(cfg);
    }
}

impl Library for TaskLib {
    fn prelude(&self, _ctx: &mut Ctx) {}
}

const FUNC: &str = "function";
const CTX: &str = "context";
const INPUT: &str = "input";

pub fn new_call() -> FreePrimFuncVal {
    FreePrimFn { id: "task.call", f: free_impl(fn_new_call), mode: default_free_mode() }.free()
}

fn fn_new_call(_cfg: &mut Cfg, input: Val) -> Val {
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

pub fn new_solve() -> FreePrimFuncVal {
    FreePrimFn { id: "task.solve", f: free_impl(fn_new_solve), mode: default_free_mode() }.free()
}

fn fn_new_solve(_cfg: &mut Cfg, input: Val) -> Val {
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

pub fn apply() -> MutPrimFuncVal {
    DynPrimFn {
        id: "task.apply",
        f: MutImpl::new(fn_apply_free, fn_apply_const, fn_apply_mut),
        mode: default_dyn_mode(),
    }
    .mut_()
}

fn fn_apply_free(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Task(task) = input else {
        return Val::default();
    };
    TaskApply.free_call(cfg, task)
}

fn fn_apply_const(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Task(task) = input else {
        return Val::default();
    };
    TaskApply.const_call(cfg, ctx, task)
}

fn fn_apply_mut(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Task(task) = input else {
        return Val::default();
    };
    TaskApply.mut_call(cfg, ctx, task)
}

pub fn is_solve() -> ConstPrimFuncVal {
    DynPrimFn { id: "task.is_solve", f: const_impl(fn_is_solve), mode: default_dyn_mode() }.const_()
}

fn fn_is_solve(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Task(task) = &*ctx else {
        return Val::default();
    };
    Val::Bit(Bit::from(matches!(task.action, Action::Solve)))
}

pub fn func() -> ConstPrimFuncVal {
    DynPrimFn { id: "task.function", f: const_impl(fn_func), mode: default_dyn_mode() }.const_()
}

fn fn_func(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Task(task) = &*ctx else {
        return Val::default();
    };
    task.func.clone()
}

pub fn set_func() -> MutPrimFuncVal {
    DynPrimFn { id: "task.set_function", f: mut_impl(fn_set_func), mode: default_dyn_mode() }.mut_()
}

fn fn_set_func(_cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Task(task) = ctx else {
        return Val::default();
    };
    swap(&mut task.func, &mut input);
    input
}

pub fn ctx() -> ConstPrimFuncVal {
    DynPrimFn { id: "task.context", f: const_impl(fn_ctx), mode: default_dyn_mode() }.const_()
}

fn fn_ctx(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Task(task) = &*ctx else {
        return Val::default();
    };
    task.ctx.clone()
}

pub fn set_ctx() -> MutPrimFuncVal {
    DynPrimFn { id: "task.set_context", f: mut_impl(fn_set_ctx), mode: default_dyn_mode() }.mut_()
}

fn fn_set_ctx(_cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Task(task) = ctx else {
        return Val::default();
    };
    swap(&mut task.ctx, &mut input);
    input
}

pub fn input() -> ConstPrimFuncVal {
    DynPrimFn { id: "task.input", f: const_impl(fn_input), mode: default_dyn_mode() }.const_()
}

fn fn_input(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Task(task) = &*ctx else {
        return Val::default();
    };
    task.input.clone()
}

pub fn set_input() -> MutPrimFuncVal {
    DynPrimFn { id: "task.set_input", f: mut_impl(fn_set_input), mode: default_dyn_mode() }.mut_()
}

fn fn_set_input(_cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Task(task) = ctx else {
        return Val::default();
    };
    swap(&mut task.input, &mut input);
    input
}
