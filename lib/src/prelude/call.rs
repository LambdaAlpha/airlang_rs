use std::mem::swap;

use super::DynFn;
use super::FreeFn;
use super::FuncMode;
use super::MutStaticImpl;
use super::Prelude;
use super::PreludeCtx;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use super::setup::ctx_default_mode;
use crate::semantics::core::CallApply;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::syntax::CALL_FORWARD;
use crate::syntax::CALL_REVERSE;
use crate::type_::Bit;
use crate::type_::Call;
use crate::type_::ConstRef;
use crate::type_::Pair;

#[derive(Clone)]
pub struct CallPrelude {
    pub new_forward: FreeStaticPrimFuncVal,
    pub new_reverse: FreeStaticPrimFuncVal,
    pub apply: MutStaticPrimFuncVal,
    pub is_reverse: ConstStaticPrimFuncVal,
    pub func: ConstStaticPrimFuncVal,
    pub set_func: MutStaticPrimFuncVal,
    pub input: ConstStaticPrimFuncVal,
    pub set_input: MutStaticPrimFuncVal,
}

impl Default for CallPrelude {
    fn default() -> Self {
        CallPrelude {
            new_forward: new_forward(),
            new_reverse: new_reverse(),
            apply: apply(),
            is_reverse: is_reverse(),
            func: func(),
            set_func: set_func(),
            input: input(),
            set_input: set_input(),
        }
    }
}

impl Prelude for CallPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.new_forward.put(ctx);
        self.new_reverse.put(ctx);
        self.apply.put(ctx);
        self.is_reverse.put(ctx);
        self.func.put(ctx);
        self.set_func.put(ctx);
        self.input.put(ctx);
        self.set_input.put(ctx);
    }
}

pub fn new_forward() -> FreeStaticPrimFuncVal {
    FreeFn { id: CALL_FORWARD, f: free_impl(fn_new_forward), mode: FuncMode::default() }
        .free_static()
}

fn fn_new_forward(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    Val::Call(Call::new(false, pair.first, pair.second).into())
}

pub fn new_reverse() -> FreeStaticPrimFuncVal {
    FreeFn { id: CALL_REVERSE, f: free_impl(fn_new_reverse), mode: FuncMode::default() }
        .free_static()
}

fn fn_new_reverse(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    Val::Call(Call::new(true, pair.first, pair.second).into())
}

pub fn apply() -> MutStaticPrimFuncVal {
    DynFn {
        id: "call.apply",
        f: MutStaticImpl::new(fn_apply_free, fn_apply_const, fn_apply_mut),
        mode: FuncMode::default(),
        ctx_explicit: false,
    }
    .mut_static()
}

fn fn_apply_free(input: Val) -> Val {
    let Val::Call(call) = input else {
        return Val::default();
    };
    CallApply.free_static_call(call)
}

fn fn_apply_const(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Call(call) = input else {
        return Val::default();
    };
    CallApply.const_static_call(ctx, call)
}

fn fn_apply_mut(ctx: &mut Val, input: Val) -> Val {
    let Val::Call(call) = input else {
        return Val::default();
    };
    CallApply.mut_static_call(ctx, call)
}

pub fn is_reverse() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "call.is_reverse",
        f: const_impl(fn_is_reverse),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_is_reverse(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Call(call) = &*ctx else {
        return Val::default();
    };
    Val::Bit(Bit::new(call.reverse))
}

pub fn func() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "call.function",
        f: const_impl(fn_func),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_func(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Call(call) = &*ctx else {
        return Val::default();
    };
    call.func.clone()
}

pub fn set_func() -> MutStaticPrimFuncVal {
    DynFn {
        id: "call.set_function",
        f: mut_impl(fn_set_func),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
}

fn fn_set_func(ctx: &mut Val, mut input: Val) -> Val {
    let Val::Call(call) = ctx else {
        return Val::default();
    };
    swap(&mut call.func, &mut input);
    input
}

pub fn input() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "call.input",
        f: const_impl(fn_input),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_input(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Call(call) = &*ctx else {
        return Val::default();
    };
    call.input.clone()
}

pub fn set_input() -> MutStaticPrimFuncVal {
    DynFn {
        id: "call.set_input",
        f: mut_impl(fn_set_input),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
}

fn fn_set_input(ctx: &mut Val, mut input: Val) -> Val {
    let Val::Call(call) = ctx else {
        return Val::default();
    };
    swap(&mut call.input, &mut input);
    input
}
