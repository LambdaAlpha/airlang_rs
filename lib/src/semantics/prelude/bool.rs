use crate::{
    semantics::{
        eval::{
            BasicEvalMode,
            EvalMode,
            Func,
            Primitive,
        },
        prelude::{
            names,
            prelude_func,
        },
        val::Val,
    },
    types::Bool,
};

pub(crate) fn not() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::NOT,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_not,
    )))
}

fn fn_not(input: Val) -> Val {
    let Val::Bool(b) = input else {
        return Val::default();
    };
    Val::Bool(b.not())
}

pub(crate) fn and() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::AND,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_and,
    )))
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::OR,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_or,
    )))
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::EQUAL,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_equal,
    )))
}

fn fn_equal(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Val::Bool(Bool::new(pair.first == pair.second))
}

pub(crate) fn not_equal() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::NOT_EQUAL,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_not_equal,
    )))
}

fn fn_not_equal(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Val::Bool(Bool::new(pair.first != pair.second))
}
