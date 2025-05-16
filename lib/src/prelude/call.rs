use std::mem::swap;

use crate::Bit;
use crate::Call;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeStaticFn;
use crate::FuncMode;
use crate::MutStaticFn;
use crate::MutStaticImpl;
use crate::Pair;
use crate::Val;
use crate::core::CallApply;
use crate::ctx::main::MainCtx;
use crate::either::Either;
use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::free_impl;
use crate::prelude::mut_impl;
use crate::prelude::named_const_fn;
use crate::prelude::named_free_fn;
use crate::prelude::named_mut_fn;
use crate::prelude::ref_pair_mode;
use crate::syntax::CALL_FORWARD;
use crate::syntax::CALL_REVERSE;
use crate::val::func::FuncVal;

#[derive(Clone)]
pub(crate) struct CallPrelude {
    pub(crate) new_forward: Named<FuncVal>,
    pub(crate) new_reverse: Named<FuncVal>,
    pub(crate) apply: Named<FuncVal>,
    pub(crate) is_reverse: Named<FuncVal>,
    pub(crate) get_func: Named<FuncVal>,
    pub(crate) set_func: Named<FuncVal>,
    pub(crate) get_input: Named<FuncVal>,
    pub(crate) set_input: Named<FuncVal>,
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

fn new_forward() -> Named<FuncVal> {
    let id = CALL_FORWARD;
    let f = free_impl(fn_new_forward);
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_new_forward(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    Val::Call(Call::new(false, pair.first, pair.second).into())
}

fn new_reverse() -> Named<FuncVal> {
    let id = CALL_REVERSE;
    let f = free_impl(fn_new_reverse);
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_new_reverse(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    Val::Call(Call::new(true, pair.first, pair.second).into())
}

fn apply() -> Named<FuncVal> {
    let id = "call.apply";
    let f = MutStaticImpl::new(fn_apply_free, fn_apply_const, fn_apply_mut);
    let mode = FuncMode::default();
    named_mut_fn(id, f, mode)
}

fn fn_apply_free(input: Val) -> Val {
    let Val::Call(call) = input else {
        return Val::default();
    };
    CallApply.free_static_call(call)
}

fn fn_apply_const(ctx: ConstRef<Ctx>, input: Val) -> Val {
    let Val::Call(call) = input else {
        return Val::default();
    };
    CallApply.const_static_call(ctx, call)
}

fn fn_apply_mut(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Call(call) = input else {
        return Val::default();
    };
    CallApply.mut_static_call(ctx, call)
}

fn is_reverse() -> Named<FuncVal> {
    let id = "call.is_reverse";
    let f = const_impl(fn_is_reverse);
    let forward = ref_pair_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_const_fn(id, f, mode)
}

fn fn_is_reverse(ctx: ConstRef<Ctx>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_lossless(&ctx, pair.first, |val| match val {
        Val::Call(call) => Val::Bit(Bit::new(call.reverse)),
        _ => Val::default(),
    })
}

fn get_func() -> Named<FuncVal> {
    let id = "call.function";
    let f = const_impl(fn_get_func);
    let forward = ref_pair_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_const_fn(id, f, mode)
}

fn fn_get_func(ctx: ConstRef<Ctx>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_or_val(&ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match &val {
            Val::Call(call) => call.func.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Call(call) => Call::from(call).func,
            _ => Val::default(),
        },
    })
}

fn set_func() -> Named<FuncVal> {
    let id = "call.set_function";
    let f = mut_impl(fn_set_func);
    let forward = ref_pair_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_mut_fn(id, f, mode)
}

fn fn_set_func(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    MainCtx::with_ref_mut_or_val(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(call) => {
            let Val::Call(call) = call else {
                return Val::default();
            };
            swap(&mut call.func, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}

fn get_input() -> Named<FuncVal> {
    let id = "call.input";
    let f = const_impl(fn_get_input);
    let forward = ref_pair_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_const_fn(id, f, mode)
}

fn fn_get_input(ctx: ConstRef<Ctx>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_or_val(&ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match &val {
            Val::Call(call) => call.input.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Call(call) => Call::from(call).input,
            _ => Val::default(),
        },
    })
}

fn set_input() -> Named<FuncVal> {
    let id = "call.set_input";
    let f = mut_impl(fn_set_input);
    let forward = ref_pair_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_mut_fn(id, f, mode)
}

fn fn_set_input(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    MainCtx::with_ref_mut_or_val(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(call) => {
            let Val::Call(call) = call else {
                return Val::default();
            };
            swap(&mut call.input, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}
