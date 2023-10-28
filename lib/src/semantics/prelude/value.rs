use crate::{
    semantics::{
        ctx::DefaultCtx,
        ctx_access::constant::CtxForConstFn,
        eval_mode::EvalMode,
        func::{
            CtxConstFn,
            Primitive,
        },
        input_mode::InputMode,
        prelude::{
            names,
            PrimitiveFunc,
        },
        Val,
    },
    types::{
        Bool,
        Pair,
        Symbol,
    },
};

pub(crate) fn type_of() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::TYPE_OF, fn_type_of);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_type_of(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let s = match val {
            Val::Unit(_) => "unit",
            Val::Bool(_) => "bool",
            Val::Int(_) => "int",
            Val::Float(_) => "float",
            Val::Bytes(_) => "bytes",
            Val::Symbol(_) => "symbol",
            Val::String(_) => "string",
            Val::Pair(_) => "pair",
            Val::Call(_) => "call",
            Val::Reverse(_) => "reverse",
            Val::List(_) => "list",
            Val::Map(_) => "map",
            Val::Func(_) => "function",
            Val::Ctx(_) => "context",
            Val::Prop(_) => "proposition",
        };
        Val::Symbol(Symbol::from_str(s))
    })
}

pub(crate) fn equal() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Symbol(EvalMode::Value),
    )));
    let primitive = Primitive::<CtxConstFn>::new(names::EQUAL, fn_equal);
    PrimitiveFunc::new(input_mode, primitive)
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
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Symbol(EvalMode::Value),
    )));
    let primitive = Primitive::<CtxConstFn>::new(names::NOT_EQUAL, fn_not_equal);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_not_equal(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    DefaultCtx.get_many_const_ref(&ctx, [pair.first, pair.second], |[v1, v2]| {
        Val::Bool(Bool::new(v1 != v2))
    })
}
