use crate::{
    semantics::{
        eval_mode::EvalMode,
        func::{
            CtxFreeFn,
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
        Bool,
        Pair,
    },
};

pub(crate) fn not() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Any(EvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::NOT, fn_not);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_not(input: Val) -> Val {
    let Val::Bool(b) = input else {
        return Val::default();
    };
    Val::Bool(b.not())
}

pub(crate) fn and() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxFreeFn>::new(names::AND, fn_and);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_and(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bool(left) = pair.first else {
        return Val::default();
    };
    if left.bool() {
        let Val::Bool(right) = pair.second else {
            return Val::default();
        };
        Val::Bool(right)
    } else {
        Val::Bool(Bool::f())
    }
}

pub(crate) fn or() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxFreeFn>::new(names::OR, fn_or);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_or(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bool(left) = pair.first else {
        return Val::default();
    };
    if left.bool() {
        Val::Bool(Bool::t())
    } else {
        let Val::Bool(right) = pair.second else {
            return Val::default();
        };
        Val::Bool(right)
    }
}
