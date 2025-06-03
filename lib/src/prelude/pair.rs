use std::mem::swap;

use crate::ConstRef;
use crate::ConstStaticPrimFuncVal;
use crate::FreeStaticPrimFuncVal;
use crate::FuncMode;
use crate::MutStaticPrimFuncVal;
use crate::prelude::DynFn;
use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;
use crate::prelude::free_impl;
use crate::prelude::mut_impl;
use crate::syntax::PAIR;
use crate::val::Val;

#[derive(Clone)]
pub(crate) struct PairPrelude {
    pub(crate) new: FreeStaticPrimFuncVal,
    pub(crate) get_first: ConstStaticPrimFuncVal,
    pub(crate) set_first: MutStaticPrimFuncVal,
    pub(crate) get_second: ConstStaticPrimFuncVal,
    pub(crate) set_second: MutStaticPrimFuncVal,
}

impl Default for PairPrelude {
    fn default() -> Self {
        PairPrelude {
            new: new(),
            get_first: get_first(),
            set_first: set_first(),
            get_second: get_second(),
            set_second: set_second(),
        }
    }
}

impl Prelude for PairPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.new.put(ctx);
        self.get_first.put(ctx);
        self.set_first.put(ctx);
        self.get_second.put(ctx);
        self.set_second.put(ctx);
    }
}

fn new() -> FreeStaticPrimFuncVal {
    FreeFn { id: PAIR, f: free_impl(fn_new), mode: FuncMode::default() }.free_static()
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(_) = input else {
        return Val::default();
    };
    input
}

fn get_first() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "pair.first",
        f: const_impl(fn_get_first),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_get_first(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        return Val::default();
    };
    pair.first.clone()
}

fn set_first() -> MutStaticPrimFuncVal {
    DynFn {
        id: "pair.set_first",
        f: mut_impl(fn_set_first),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
}

fn fn_set_first(ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        return Val::default();
    };
    swap(&mut pair.first, &mut input);
    input
}

fn get_second() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "pair.second",
        f: const_impl(fn_get_second),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_get_second(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        return Val::default();
    };
    pair.second.clone()
}

fn set_second() -> MutStaticPrimFuncVal {
    DynFn {
        id: "pair.set_second",
        f: mut_impl(fn_set_second),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
}

fn fn_set_second(ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        return Val::default();
    };
    swap(&mut pair.second, &mut input);
    input
}
