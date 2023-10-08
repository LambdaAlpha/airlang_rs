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
            Primitive,
        },
        prelude::{
            names,
            PrimitiveFunc,
        },
        Val,
    },
    types::Symbol,
};

pub(crate) fn type_of() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Quote);
    let primitive = Primitive::<CtxConstFn>::new(names::TYPE_OF, fn_type_of);
    PrimitiveFunc::new(eval_mode, primitive)
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
            Val::Theorem(_) => "theorem",
        };
        Val::Symbol(Symbol::from_str(s))
    })
}
