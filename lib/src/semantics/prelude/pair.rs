use {
    crate::{
        semantics::{
            ctx::DefaultCtx,
            ctx_access::{
                constant::CtxForConstFn,
                mutable::CtxForMutableFn,
            },
            eval_mode::EvalMode,
            func::{
                CtxConstFn,
                CtxMutableFn,
                Primitive,
            },
            input_mode::InputMode,
            prelude::{
                names,
                PrimitiveFunc,
            },
            val::Val,
        },
        types::{
            Either,
            Pair,
        },
    },
    std::mem::swap,
};

pub(crate) fn first() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::PAIR_FIRST, fn_first);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_first(mut ctx: CtxForConstFn, input: Val) -> Val {
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

pub(crate) fn first_assign() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxMutableFn>::new(names::PAIR_FIRST_ASSIGN, fn_first_assign);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_first_assign(mut ctx: CtxForMutableFn, input: Val) -> Val {
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

pub(crate) fn second() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::PAIR_SECOND, fn_second);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_second(mut ctx: CtxForConstFn, input: Val) -> Val {
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

pub(crate) fn second_assign() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxMutableFn>::new(names::PAIR_SECOND_ASSIGN, fn_second_assign);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_second_assign(mut ctx: CtxForMutableFn, input: Val) -> Val {
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
