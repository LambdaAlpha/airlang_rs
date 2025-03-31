use std::mem::swap;

use crate::{
    Abstract,
    ConstFnCtx,
    FuncMode,
    FuncVal,
    Map,
    MutFnCtx,
    Pair,
    Symbol,
    Val,
    ctx::{
        default::DefaultCtx,
        map::CtxValue,
    },
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
        ref_pair_mode,
    },
    syntax::ABSTRACT,
    types::either::Either,
};

#[derive(Clone)]
pub(crate) struct AbstractPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) get_value: Named<FuncVal>,
    pub(crate) set_value: Named<FuncVal>,
}

impl Default for AbstractPrelude {
    fn default() -> Self {
        AbstractPrelude { new: new(), get_value: get_value(), set_value: set_value() }
    }
}

impl Prelude for AbstractPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.new.put(m);
        self.get_value.put(m);
        self.set_value.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let id = ABSTRACT;
    let f = fn_new;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_new(input: Val) -> Val {
    Val::Abstract(Abstract::new(input).into())
}

fn get_value() -> Named<FuncVal> {
    let id = "abstract.value";
    let f = fn_get_value;
    let call = ref_pair_mode();
    let optimize = call.clone();
    let solve = FuncMode::default_mode();
    let mode = FuncMode { call, optimize, solve };
    named_const_fn(id, f, mode)
}

fn fn_get_value(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_dyn(ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match val.as_const() {
            Val::Abstract(abstract1) => abstract1.value.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Abstract(abstract1) => Abstract::from(abstract1).value,
            _ => Val::default(),
        },
    })
}

fn set_value() -> Named<FuncVal> {
    let id = "abstract.set_value";
    let f = fn_set_value;
    let call = ref_pair_mode();
    let optimize = call.clone();
    let solve = FuncMode::default_mode();
    let mode = FuncMode { call, optimize, solve };
    named_mut_fn(id, f, mode)
}

fn fn_set_value(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx::with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(mut pair) => {
            let Some(Val::Abstract(abstract1)) = pair.as_mut() else {
                return Val::default();
            };
            swap(&mut abstract1.value, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}
