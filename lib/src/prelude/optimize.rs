use std::mem::swap;

use crate::{
    ConstFnCtx,
    FuncMode,
    FuncVal,
    Map,
    MutFnCtx,
    Optimize,
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
    syntax::OPTIMIZE,
    types::either::Either,
};

#[derive(Clone)]
pub(crate) struct OptimizePrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) get_func: Named<FuncVal>,
    pub(crate) set_func: Named<FuncVal>,
}

impl Default for OptimizePrelude {
    fn default() -> Self {
        OptimizePrelude { new: new(), get_func: get_func(), set_func: set_func() }
    }
}

impl Prelude for OptimizePrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.new.put(m);
        self.get_func.put(m);
        self.set_func.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let id = OPTIMIZE;
    let f = fn_new;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_new(input: Val) -> Val {
    Val::Optimize(Optimize::new(input).into())
}

fn get_func() -> Named<FuncVal> {
    let id = "optimize.function";
    let f = fn_get_func;
    let call = ref_pair_mode();
    let optimize = call.clone();
    let solve = FuncMode::default_mode();
    let mode = FuncMode { call, optimize, solve };
    named_const_fn(id, f, mode)
}

fn fn_get_func(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_dyn(ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match val.as_const() {
            Val::Optimize(optimize) => optimize.func.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Optimize(optimize) => Optimize::from(optimize).func,
            _ => Val::default(),
        },
    })
}

fn set_func() -> Named<FuncVal> {
    let id = "optimize.set_function";
    let f = fn_set_func;
    let call = ref_pair_mode();
    let optimize = call.clone();
    let solve = FuncMode::default_mode();
    let mode = FuncMode { call, optimize, solve };
    named_mut_fn(id, f, mode)
}

fn fn_set_func(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx::with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(mut optimize) => {
            let Some(Val::Optimize(optimize)) = optimize.as_mut() else {
                return Val::default();
            };
            swap(&mut optimize.func, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}
