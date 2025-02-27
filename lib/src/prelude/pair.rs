use std::mem::swap;

use crate::{
    ConstFnCtx,
    FuncMode,
    Map,
    Pair,
    Symbol,
    ctx::{
        default::DefaultCtx,
        map::CtxValue,
        mut1::MutFnCtx,
    },
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
        ref_pair_mode,
    },
    syntax::PAIR,
    types::either::Either,
    val::{
        Val,
        func::FuncVal,
    },
};

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
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.new.put(m);
        self.get_first.put(m);
        self.set_first.put(m);
        self.get_second.put(m);
        self.set_second.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let id = PAIR;
    let f = fn_new;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(_) = input else {
        return Val::default();
    };
    input
}

fn get_first() -> Named<FuncVal> {
    let id = "pair.first";
    let f = fn_get_first;
    let call = ref_pair_mode();
    let abstract1 = call.clone();
    let ask = FuncMode::default_mode();
    let mode = FuncMode { call, abstract1, ask };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_get_first(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_dyn(ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match val.as_const() {
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
    let f = fn_set_first;
    let call = ref_pair_mode();
    let abstract1 = call.clone();
    let ask = FuncMode::default_mode();
    let mode = FuncMode { call, abstract1, ask };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_set_first(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx::with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(mut pair) => {
            let Some(Val::Pair(pair)) = pair.as_mut() else {
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
    let f = fn_get_second;
    let call = ref_pair_mode();
    let abstract1 = call.clone();
    let ask = FuncMode::default_mode();
    let mode = FuncMode { call, abstract1, ask };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_get_second(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_dyn(ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match val.as_const() {
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
    let f = fn_set_second;
    let call = ref_pair_mode();
    let abstract1 = call.clone();
    let ask = FuncMode::default_mode();
    let mode = FuncMode { call, abstract1, ask };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_set_second(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx::with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(mut pair) => {
            let Some(Val::Pair(pair)) = pair.as_mut() else {
                return Val::default();
            };
            swap(&mut pair.second, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}
