use std::mem::swap;

use crate::Bit;
use crate::Call;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::ConstStaticPrimFuncVal;
use crate::FreeStaticFn;
use crate::FreeStaticPrimFuncVal;
use crate::FuncMode;
use crate::MutStaticFn;
use crate::MutStaticImpl;
use crate::MutStaticPrimFuncVal;
use crate::Pair;
use crate::Val;
use crate::core::CallApply;
use crate::prelude::DynFn;
use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;
use crate::prelude::free_impl;
use crate::prelude::mut_impl;
use crate::syntax::CALL_FORWARD;
use crate::syntax::CALL_REVERSE;

#[derive(Clone)]
pub(crate) struct CallPrelude {
    pub(crate) new_forward: FreeStaticPrimFuncVal,
    pub(crate) new_reverse: FreeStaticPrimFuncVal,
    pub(crate) apply: MutStaticPrimFuncVal,
    pub(crate) is_reverse: ConstStaticPrimFuncVal,
    pub(crate) get_func: ConstStaticPrimFuncVal,
    pub(crate) set_func: MutStaticPrimFuncVal,
    pub(crate) get_input: ConstStaticPrimFuncVal,
    pub(crate) set_input: MutStaticPrimFuncVal,
}

impl Default for CallPrelude {
    fn default() -> Self {
        CallPrelude {
            new_forward: new_forward(),
            new_reverse: new_reverse(),
            apply: apply(),
            is_reverse: is_reverse(),
            get_func: get_func(),
            set_func: set_func(),
            get_input: get_input(),
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
        self.get_func.put(ctx);
        self.set_func.put(ctx);
        self.get_input.put(ctx);
        self.set_input.put(ctx);
    }
}

fn new_forward() -> FreeStaticPrimFuncVal {
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

fn new_reverse() -> FreeStaticPrimFuncVal {
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

fn apply() -> MutStaticPrimFuncVal {
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

fn is_reverse() -> ConstStaticPrimFuncVal {
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

fn get_func() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "call.function",
        f: const_impl(fn_get_func),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_get_func(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Call(call) = &*ctx else {
        return Val::default();
    };
    call.func.clone()
}

fn set_func() -> MutStaticPrimFuncVal {
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

fn get_input() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "call.input",
        f: const_impl(fn_get_input),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_get_input(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Call(call) = &*ctx else {
        return Val::default();
    };
    call.input.clone()
}

fn set_input() -> MutStaticPrimFuncVal {
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
