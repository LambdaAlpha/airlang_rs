use {
    crate::{
        ctx::{
            DefaultCtx,
            NameMap,
        },
        ctx_access::{
            constant::CtxForConstFn,
            mutable::CtxForMutableFn,
        },
        eval_mode::EvalMode,
        io_mode::IoMode,
        pair::Pair,
        prelude::{
            named_const_fn,
            named_mutable_fn,
            Named,
            Prelude,
        },
        types::either::Either,
        val::{
            FuncVal,
            Val,
        },
    },
    std::mem::swap,
};

#[derive(Clone)]
pub(crate) struct PairPrelude {
    pub(crate) get_first: Named<FuncVal>,
    pub(crate) set_first: Named<FuncVal>,
    pub(crate) get_second: Named<FuncVal>,
    pub(crate) set_second: Named<FuncVal>,
}

impl Default for PairPrelude {
    fn default() -> Self {
        PairPrelude {
            get_first: get_first(),
            set_first: set_first(),
            get_second: get_second(),
            set_second: set_second(),
        }
    }
}

impl Prelude for PairPrelude {
    fn put(&self, m: &mut NameMap) {
        self.get_first.put(m);
        self.set_first.put(m);
        self.get_second.put(m);
        self.set_second.put(m);
    }
}

fn get_first() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_const_fn("get_1", input_mode, output_mode, fn_get_first)
}

fn fn_get_first(mut ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_ref_or_val(&mut ctx, input, |ref_or_val| match ref_or_val {
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
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Symbol(EvalMode::Value),
        IoMode::Any(EvalMode::More),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_mutable_fn("set_1", input_mode, output_mode, fn_set_first)
}

fn fn_set_first(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.get_ref_or_val(&mut ctx, name, |ref_or_val| match ref_or_val {
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
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_const_fn("get_2", input_mode, output_mode, fn_get_second)
}

fn fn_get_second(mut ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_ref_or_val(&mut ctx, input, |ref_or_val| match ref_or_val {
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
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Symbol(EvalMode::Value),
        IoMode::Any(EvalMode::More),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_mutable_fn("set_2", input_mode, output_mode, fn_set_second)
}

fn fn_set_second(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.get_ref_or_val(&mut ctx, name, |ref_or_val| match ref_or_val {
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
