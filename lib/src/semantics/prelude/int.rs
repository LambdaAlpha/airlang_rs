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
    types::Pair,
};

pub(crate) fn add() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxFreeFn>::new(names::INT_ADD, fn_add);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn subtract() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxFreeFn>::new(names::INT_SUBTRACT, fn_subtract);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn multiply() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxFreeFn>::new(names::INT_MULTIPLY, fn_multiply);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn divide() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxFreeFn>::new(names::INT_DIVIDE, fn_divide);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn remainder() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxFreeFn>::new(names::INT_REMAINDER, fn_remainder);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn divide_remainder() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxFreeFn>::new(names::INT_DIVIDE_REMAINDER, fn_divide_remainder);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn less_than() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxFreeFn>::new(names::INT_LESS_THAN, fn_less_than);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn less_equal() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxFreeFn>::new(names::INT_LESS_EQUAL, fn_less_equal);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn greater_than() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxFreeFn>::new(names::INT_GREATER_THAN, fn_greater_than);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn greater_equal() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxFreeFn>::new(names::INT_GREATER_EQUAL, fn_greater_equal);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn less_greater() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxFreeFn>::new(names::INT_LESS_GREATER, fn_less_greater);
    PrimitiveFunc::new(input_mode, primitive)
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
