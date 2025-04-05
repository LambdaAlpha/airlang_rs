use std::mem::swap;

use crate::{
    ConstFnCtx,
    FuncMode,
    Inverse,
    Map,
    Pair,
    Symbol,
    Val,
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
    syntax::INVERSE,
    types::either::Either,
    val::func::FuncVal,
};

#[derive(Clone)]
pub(crate) struct InversePrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) get_func: Named<FuncVal>,
    pub(crate) set_func: Named<FuncVal>,
}

impl Default for InversePrelude {
    fn default() -> Self {
        InversePrelude { new: new(), get_func: get_func(), set_func: set_func() }
    }
}

impl Prelude for InversePrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.new.put(m);
        self.get_func.put(m);
        self.set_func.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let id = INVERSE;
    let f = fn_new;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_new(input: Val) -> Val {
    Val::Inverse(Inverse::new(input).into())
}

fn get_func() -> Named<FuncVal> {
    let id = "inverse.function";
    let f = fn_get_func;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_const_fn(id, f, mode)
}

fn fn_get_func(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_dyn(ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match val.as_const() {
            Val::Inverse(inverse) => inverse.func.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Inverse(inverse) => Inverse::from(inverse).func,
            _ => Val::default(),
        },
    })
}

fn set_func() -> Named<FuncVal> {
    let id = "inverse.set_function";
    let f = fn_set_func;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
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
        Either::This(mut inverse) => {
            let Some(Val::Inverse(inverse)) = inverse.as_mut() else {
                return Val::default();
            };
            swap(&mut inverse.func, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}
