use crate::{
    semantics::{
        ctx::{
            DefaultCtx,
            TaggedRef,
        },
        ctx_access::{
            constant::{
                ConstCtx,
                CtxForConstFn,
            },
            free::FreeCtx,
            mutable::CtxForMutableFn,
            CtxAccessor,
        },
        eval::Evaluator,
        eval_mode::{
            eval::Eval,
            EvalMode,
            QUOTE,
        },
        func::{
            CtxConstFn,
            CtxFreeFn,
            CtxMutableFn,
            Primitive,
        },
        input_mode::InputMode,
        prelude::{
            names,
            utils::{
                map_remove,
                parse_input_mode,
                symbol,
            },
            PrimitiveFunc,
        },
        val::{
            CtxVal,
            Val,
        },
    },
    types::{
        Bool,
        Map,
    },
};

pub(crate) fn value() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let primitive = Primitive::<CtxFreeFn>::new(names::VALUE, fn_value);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_value(input: Val) -> Val {
    input
}

pub(crate) fn eval() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new(names::EVAL, fn_eval);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_eval(mut ctx: CtxForMutableFn, input: Val) -> Val {
    Eval.eval(&mut ctx, input)
}

pub(crate) fn quote() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new(names::QUOTE, fn_quote);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_quote(mut ctx: CtxForMutableFn, input: Val) -> Val {
    QUOTE.eval(&mut ctx, input)
}

pub(crate) fn eval_twice() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::EVAL_TWICE,
        fn_eval_twice::<FreeCtx>,
        |ctx, val| fn_eval_twice(ctx, val),
        |ctx, val| fn_eval_twice(ctx, val),
    );
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_eval_twice<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let val = Eval.eval(&mut ctx, input);
    Eval.eval(&mut ctx, val)
}

pub(crate) fn eval_thrice() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::EVAL_THRICE,
        fn_eval_thrice::<FreeCtx>,
        |ctx, val| fn_eval_thrice(ctx, val),
        |ctx, val| fn_eval_thrice(ctx, val),
    );
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_eval_thrice<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let val1 = Eval.eval(&mut ctx, input);
    let val2 = Eval.eval(&mut ctx, val1);
    Eval.eval(&mut ctx, val2)
}

const VALUE: &str = "value";
const CONTEXT: &str = "context";
const INPUT_MODE: &str = "input_mode";

pub(crate) fn is_ctx_free() -> PrimitiveFunc<CtxFreeFn> {
    let mut map = Map::default();
    map.insert(symbol(VALUE), InputMode::Any(EvalMode::Quote));
    map.insert(symbol(INPUT_MODE), InputMode::Any(EvalMode::Quote));
    let input_mode = InputMode::MapForSome(map);
    let primitive = Primitive::<CtxFreeFn>::new(names::IS_CTX_FREE, fn_is_ctx_free);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_is_ctx_free(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let Some(input_mode) = parse_input_mode(map_remove(&mut map, INPUT_MODE)) else {
        return Val::default();
    };
    let value = map_remove(&mut map, VALUE);
    let is_ctx_free = input_mode.is_free(&mut FreeCtx, value);
    Val::Bool(Bool::new(is_ctx_free))
}

pub(crate) fn is_ctx_const() -> PrimitiveFunc<CtxConstFn> {
    let mut map = Map::default();
    map.insert(symbol(VALUE), InputMode::Any(EvalMode::Quote));
    map.insert(symbol(CONTEXT), InputMode::Symbol(EvalMode::Value));
    map.insert(symbol(INPUT_MODE), InputMode::Any(EvalMode::Quote));
    let input_mode = InputMode::MapForSome(map);
    let primitive = Primitive::<CtxConstFn>::new(names::IS_CTX_CONST, fn_is_ctx_const);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_is_ctx_const(mut ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let Some(input_mode) = parse_input_mode(map_remove(&mut map, INPUT_MODE)) else {
        return Val::default();
    };
    let value = map_remove(&mut map, VALUE);
    let target_ctx = map_remove(&mut map, CONTEXT);
    if target_ctx.is_unit() {
        let is_ctx_const = input_mode.is_const(&mut ctx, value);
        return Val::Bool(Bool::new(is_ctx_const));
    }
    DefaultCtx.get_tagged_ref(&mut ctx, target_ctx, |target_ctx| {
        let TaggedRef {
            val_ref: Val::Ctx(CtxVal(target_ctx)),
            ..
        } = target_ctx
        else {
            return Val::default();
        };
        let is_ctx_const = input_mode.is_const(&mut ConstCtx(target_ctx), value);
        Val::Bool(Bool::new(is_ctx_const))
    })
}
