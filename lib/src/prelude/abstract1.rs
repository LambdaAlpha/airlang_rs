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
    pub(crate) get_function: Named<FuncVal>,
    pub(crate) set_function: Named<FuncVal>,
}

impl Default for AbstractPrelude {
    fn default() -> Self {
        AbstractPrelude { new: new(), get_function: get_function(), set_function: set_function() }
    }
}

impl Prelude for AbstractPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.new.put(m);
        self.get_function.put(m);
        self.set_function.put(m);
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

fn get_function() -> Named<FuncVal> {
    let id = "abstract.function";
    let f = fn_get_function;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_get_function(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_dyn(ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match val.as_const() {
            Val::Abstract(abstract1) => abstract1.func.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Abstract(abstract1) => Abstract::from(abstract1).func,
            _ => Val::default(),
        },
    })
}

fn set_function() -> Named<FuncVal> {
    let id = "abstract.set_function";
    let f = fn_set_function;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_mut_fn(id, f, mode)
}

fn fn_set_function(ctx: MutFnCtx, input: Val) -> Val {
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
            swap(&mut abstract1.func, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}
