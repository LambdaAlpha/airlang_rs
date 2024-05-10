use std::mem::swap;

use crate::{
    ctx::{
        CtxMap,
        DefaultCtx,
    },
    ctx_access::{
        constant::CtxForConstFn,
        mutable::CtxForMutableFn,
    },
    prelude::{
        default_mode,
        named_const_fn,
        named_free_fn,
        named_mutable_fn,
        pair_mode,
        symbol_id_mode,
        Named,
        Prelude,
    },
    syntax::PAIR_INFIX,
    types::either::Either,
    val::{
        func::FuncVal,
        Val,
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
    fn put(&self, m: &mut CtxMap) {
        self.new.put(m);
        self.get_first.put(m);
        self.set_first.put(m);
        self.get_second.put(m);
        self.set_second.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    named_free_fn(PAIR_INFIX, input_mode, output_mode, fn_new)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(_) = input else {
        return Val::default();
    };
    input
}

fn get_first() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn("get_1", input_mode, output_mode, fn_get_first)
}

fn fn_get_first(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_dyn(ctx, input, |ref_or_val| match ref_or_val {
        Either::Left(val) => match val.as_const() {
            Val::Pair(pair) => pair.first.clone(),
            _ => Val::default(),
        },
        Either::Right(val) => match val {
            Val::Pair(pair) => pair.first,
            _ => Val::default(),
        },
    })
}

fn set_first() -> Named<FuncVal> {
    let input_mode = pair_mode(symbol_id_mode(), default_mode());
    let output_mode = default_mode();
    named_mutable_fn("set_1", input_mode, output_mode, fn_set_first)
}

fn fn_set_first(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut pair) => {
            let Some(Val::Pair(pair)) = pair.as_mut() else {
                return Val::default();
            };
            swap(&mut pair.first, &mut val);
            val
        }
        Either::Right(_) => Val::default(),
    })
}

fn get_second() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn("get_2", input_mode, output_mode, fn_get_second)
}

fn fn_get_second(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_dyn(ctx, input, |ref_or_val| match ref_or_val {
        Either::Left(val) => match val.as_const() {
            Val::Pair(pair) => pair.second.clone(),
            _ => Val::default(),
        },
        Either::Right(val) => match val {
            Val::Pair(pair) => pair.second,
            _ => Val::default(),
        },
    })
}

fn set_second() -> Named<FuncVal> {
    let input_mode = pair_mode(symbol_id_mode(), default_mode());
    let output_mode = default_mode();
    named_mutable_fn("set_2", input_mode, output_mode, fn_set_second)
}

fn fn_set_second(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut pair) => {
            let Some(Val::Pair(pair)) = pair.as_mut() else {
                return Val::default();
            };
            swap(&mut pair.second, &mut val);
            val
        }
        Either::Right(_) => Val::default(),
    })
}
