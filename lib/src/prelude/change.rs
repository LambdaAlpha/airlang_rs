use std::mem::swap;

use crate::{
    Change,
    ConstFnCtx,
    FuncMode,
    Map,
    Pair,
    Symbol,
    ctx::{
        main::MainCtx,
        map::CtxValue,
        mut1::MutFnCtx,
    },
    either::Either,
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
        ref_pair_mode,
    },
    syntax::CHANGE,
    val::{
        Val,
        func::FuncVal,
    },
};

#[derive(Clone)]
pub(crate) struct ChangePrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) get_from: Named<FuncVal>,
    pub(crate) set_from: Named<FuncVal>,
    pub(crate) get_to: Named<FuncVal>,
    pub(crate) set_to: Named<FuncVal>,
}

impl Default for ChangePrelude {
    fn default() -> Self {
        ChangePrelude {
            new: new(),
            get_from: get_from(),
            set_from: set_from(),
            get_to: get_to(),
            set_to: set_to(),
        }
    }
}

impl Prelude for ChangePrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.new.put(m);
        self.get_from.put(m);
        self.set_from.put(m);
        self.get_to.put(m);
        self.set_to.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let id = CHANGE;
    let f = fn_new;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let from = pair.first;
    let to = pair.second;
    Val::Change(Change::new(from, to).into())
}

fn get_from() -> Named<FuncVal> {
    let id = "change.from";
    let f = fn_get_from;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_get_from(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_dyn(ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match val.as_const() {
            Val::Change(change) => change.from.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Change(change) => Change::from(change).from,
            _ => Val::default(),
        },
    })
}

fn set_from() -> Named<FuncVal> {
    let id = "change.set_from";
    let f = fn_set_from;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_mut_fn(id, f, mode)
}

fn fn_set_from(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    MainCtx::with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(mut change) => {
            let Some(Val::Change(change)) = change.as_mut() else {
                return Val::default();
            };
            swap(&mut change.from, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}

fn get_to() -> Named<FuncVal> {
    let id = "change.to";
    let f = fn_get_to;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_get_to(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_dyn(ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match val.as_const() {
            Val::Change(change) => change.to.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Change(change) => Change::from(change).to,
            _ => Val::default(),
        },
    })
}

fn set_to() -> Named<FuncVal> {
    let id = "change.set_to";
    let f = fn_set_to;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_mut_fn(id, f, mode)
}

fn fn_set_to(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    MainCtx::with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(mut change) => {
            let Some(Val::Change(change)) = change.as_mut() else {
                return Val::default();
            };
            swap(&mut change.to, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}
