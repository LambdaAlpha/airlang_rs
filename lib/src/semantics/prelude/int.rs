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
    types::Pair,
};

pub(crate) fn add() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_ADD,
        EvalMode::Eval,
        fn_add,
    )))
    .into()
}

fn fn_add(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Int(i1.add(i2))
}

pub(crate) fn subtract() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_SUBTRACT,
        EvalMode::Eval,
        fn_subtract,
    )))
    .into()
}

fn fn_subtract(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Int(i1.subtract(i2))
}

pub(crate) fn multiply() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_MULTIPLY,
        EvalMode::Eval,
        fn_multiply,
    )))
    .into()
}

fn fn_multiply(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Int(i1.multiply(i2))
}

pub(crate) fn divide() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_DIVIDE,
        EvalMode::Eval,
        fn_divide,
    )))
    .into()
}

fn fn_divide(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let Some(i) = i1.divide(i2) else {
        return Val::default();
    };
    Val::Int(i)
}

pub(crate) fn remainder() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_REMAINDER,
        EvalMode::Eval,
        fn_remainder,
    )))
    .into()
}

fn fn_remainder(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let Some(i) = i1.remainder(i2) else {
        return Val::default();
    };
    Val::Int(i)
}

pub(crate) fn divide_remainder() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_DIVIDE_REMAINDER,
        EvalMode::Eval,
        fn_divide_remainder,
    )))
    .into()
}

fn fn_divide_remainder(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let Some((quotient, rem)) = i1.divide_remainder(i2) else {
        return Val::default();
    };
    Val::Pair(Box::new(Pair::new(Val::Int(quotient), Val::Int(rem))))
}

pub(crate) fn less_than() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_LESS_THAN,
        EvalMode::Eval,
        fn_less_than,
    )))
    .into()
}

fn fn_less_than(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bool(i1.less_than(&i2))
}

pub(crate) fn less_equal() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_LESS_EQUAL,
        EvalMode::Eval,
        fn_less_equal,
    )))
    .into()
}

fn fn_less_equal(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bool(i1.less_equal(&i2))
}

pub(crate) fn greater_than() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_GREATER_THAN,
        EvalMode::Eval,
        fn_greater_than,
    )))
    .into()
}

fn fn_greater_than(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bool(i1.greater_than(&i2))
}

pub(crate) fn greater_equal() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_GREATER_EQUAL,
        EvalMode::Eval,
        fn_greater_equal,
    )))
    .into()
}

fn fn_greater_equal(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bool(i1.greater_equal(&i2))
}

pub(crate) fn less_greater() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_LESS_GREATER,
        EvalMode::Eval,
        fn_less_greater,
    )))
    .into()
}

fn fn_less_greater(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bool(i1.less_greater(&i2))
}
