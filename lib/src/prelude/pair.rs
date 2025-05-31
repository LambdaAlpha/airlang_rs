use std::mem::swap;

use crate::ConstRef;
use crate::FuncMode;
use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;
use crate::prelude::free_impl;
use crate::prelude::mut_impl;
use crate::prelude::named_const_fn;
use crate::prelude::named_free_fn;
use crate::prelude::named_mut_fn;
use crate::syntax::PAIR;
use crate::val::Val;
use crate::val::func::FuncVal;

#[derive(Clone)]
pub(crate) struct PairPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) get_first: Named<FuncVal>,
    pub(crate) set_first: Named<FuncVal>,
    pub(crate) get_second: Named<FuncVal>,
    pub(crate) set_second: Named<FuncVal>,
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

fn new() -> Named<FuncVal> {
    let id = PAIR;
    let f = free_impl(fn_new);
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(_) = input else {
        return Val::default();
    };
    input
}

fn get_first() -> Named<FuncVal> {
    let id = "pair.first";
    let f = const_impl(fn_get_first);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_get_first(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        return Val::default();
    };
    pair.first.clone()
}

fn set_first() -> Named<FuncVal> {
    let id = "pair.set_first";
    let f = mut_impl(fn_set_first);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_set_first(ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        return Val::default();
    };
    swap(&mut pair.first, &mut input);
    input
}

fn get_second() -> Named<FuncVal> {
    let id = "pair.second";
    let f = const_impl(fn_get_second);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_get_second(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        return Val::default();
    };
    pair.second.clone()
}

fn set_second() -> Named<FuncVal> {
    let id = "pair.set_second";
    let f = mut_impl(fn_set_second);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_set_second(ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        return Val::default();
    };
    swap(&mut pair.second, &mut input);
    input
}
