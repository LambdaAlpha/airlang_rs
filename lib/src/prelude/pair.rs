use std::mem::swap;

use crate::ConstRef;
use crate::Ctx;
use crate::FuncMode;
use crate::Pair;
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
    let forward = ref_pair_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_const_fn(id, f, mode)
}

fn fn_get_first(ctx: ConstRef<Ctx>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_or_val(&ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match &val {
            Val::Pair(pair) => pair.first.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Pair(pair) => Pair::from(pair).first,
            _ => Val::default(),
        },
    })
}

fn set_first() -> Named<FuncVal> {
    let id = "pair.set_first";
    let f = mut_impl(fn_set_first);
    let forward = ref_pair_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_mut_fn(id, f, mode)
}

fn fn_set_first(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    MainCtx::with_ref_mut_or_val(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(pair) => {
            let Val::Pair(pair) = pair else {
                return Val::default();
            };
            swap(&mut pair.first, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}

fn get_second() -> Named<FuncVal> {
    let id = "pair.second";
    let f = const_impl(fn_get_second);
    let forward = ref_pair_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_const_fn(id, f, mode)
}

fn fn_get_second(ctx: ConstRef<Ctx>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_or_val(&ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match &val {
            Val::Pair(pair) => pair.second.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Pair(pair) => Pair::from(pair).second,
            _ => Val::default(),
        },
    })
}

fn set_second() -> Named<FuncVal> {
    let id = "pair.set_second";
    let f = mut_impl(fn_set_second);
    let forward = ref_pair_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_mut_fn(id, f, mode)
}

fn fn_set_second(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    MainCtx::with_ref_mut_or_val(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(pair) => {
            let Val::Pair(pair) = pair else {
                return Val::default();
            };
            swap(&mut pair.second, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}
