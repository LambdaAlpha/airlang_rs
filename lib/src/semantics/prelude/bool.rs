use crate::{
    semantics::{
        eval::{
            EvalMode,
            Func,
            Primitive,
        },
        prelude::names,
        val::Val,
    },
    types::Bool,
};

pub(crate) fn not() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::NOT,
        EvalMode::Eval,
        fn_not,
    )))
    .into()
}

fn fn_not(input: Val) -> Val {
    let Val::Bool(b) = input else {
        return Val::default();
    };
    Val::Bool(b.not())
}

pub(crate) fn and() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::AND,
        EvalMode::Eval,
        fn_and,
    )))
    .into()
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

pub(crate) fn or() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::OR,
        EvalMode::Eval,
        fn_or,
    )))
    .into()
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

pub(crate) fn equal() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::EQUAL,
        EvalMode::Eval,
        fn_equal,
    )))
    .into()
}

fn fn_equal(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Val::Bool(Bool::new(pair.first == pair.second))
}

pub(crate) fn not_equal() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::NOT_EQUAL,
        EvalMode::Eval,
        fn_not_equal,
    )))
    .into()
}

fn fn_not_equal(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Val::Bool(Bool::new(pair.first != pair.second))
}
