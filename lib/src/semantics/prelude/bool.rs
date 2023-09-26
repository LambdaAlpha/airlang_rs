use crate::{
    semantics::{
        ctx::DefaultCtx,
        ctx_access::constant::CtxForConstFn,
        eval_mode::{
            BasicEvalMode,
            EvalMode,
        },
        func::{
            CtxConstFn,
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
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
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
    let eval_mode = EvalMode {
        default: BasicEvalMode::Value,
        pair: Some((BasicEvalMode::Eval, BasicEvalMode::Eval)),
    };
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
    let eval_mode = EvalMode {
        default: BasicEvalMode::Value,
        pair: Some((BasicEvalMode::Eval, BasicEvalMode::Eval)),
    };
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

pub(crate) fn equal() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode {
        default: BasicEvalMode::Value,
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Inline)),
    };
    let primitive = Primitive::<CtxConstFn>::new(names::EQUAL, fn_equal);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_equal(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    DefaultCtx.get_many_const_ref(&ctx, [pair.first, pair.second], |[v1, v2]| {
        Val::Bool(Bool::new(v1 == v2))
    })
}

pub(crate) fn not_equal() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode {
        default: BasicEvalMode::Value,
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Inline)),
    };
    let primitive = Primitive::<CtxConstFn>::new(names::NOT_EQUAL, fn_not_equal);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_not_equal(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    DefaultCtx.get_many_const_ref(&ctx, [pair.first, pair.second], |[v1, v2]| {
        Val::Bool(Bool::new(v1 != v2))
    })
}
