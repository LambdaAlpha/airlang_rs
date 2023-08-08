use crate::{
    semantics::{
        eval_mode::{
            BasicEvalMode,
            EvalMode,
        },
        func::{
            CtxFreeFn,
            Primitive,
        },
        prelude::{
            names,
            PrimitiveFunc,
        },
        val::Val,
    },
    types::Bool,
};

pub(crate) fn not() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::NOT, fn_not);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_not(input: Val) -> Val {
    let Val::Bool(b) = input else {
        return Val::default();
    };
    Val::Bool(b.not())
}

pub(crate) fn and() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::AND, fn_and);
    PrimitiveFunc::new(eval_mode, primitive)
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
    let eval_mode = EvalMode::Basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::OR, fn_or);
    PrimitiveFunc::new(eval_mode, primitive)
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

pub(crate) fn equal() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::EQUAL, fn_equal);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_equal(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Val::Bool(Bool::new(pair.first == pair.second))
}

pub(crate) fn not_equal() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::NOT_EQUAL, fn_not_equal);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_not_equal(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Val::Bool(Bool::new(pair.first != pair.second))
}
