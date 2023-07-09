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
    types::Pair,
};

pub(crate) fn add() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_ADD,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_add,
    )))
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_SUBTRACT,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_subtract,
    )))
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_MULTIPLY,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_multiply,
    )))
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_DIVIDE,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_divide,
    )))
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_REMAINDER,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_remainder,
    )))
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_DIVIDE_REMAINDER,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_divide_remainder,
    )))
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_LESS_THAN,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_less_than,
    )))
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_LESS_EQUAL,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_less_equal,
    )))
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_GREATER_THAN,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_greater_than,
    )))
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_GREATER_EQUAL,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_greater_equal,
    )))
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::INT_LESS_GREATER,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_less_greater,
    )))
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
