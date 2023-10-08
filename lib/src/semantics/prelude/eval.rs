use crate::{
    semantics::{
        ctx::{
            CtxTrait,
            DefaultCtx,
            TaggedRef,
        },
        ctx_access::{
            constant::{
                ConstCtx,
                CtxForConstFn,
            },
            free::FreeCtx,
            mutable::{
                CtxForMutableFn,
                MutableCtx,
            },
            CtxAccessor,
        },
        eval::Evaluator,
        eval_mode::{
            eval::Eval,
            BasicEvalMode,
            EvalMode,
            QUOTE,
        },
        func::{
            CtxConstFn,
            CtxFreeFn,
            CtxMutableFn,
            Primitive,
        },
        prelude::{
            names,
            utils::{
                map_remove,
                parse_eval_mode,
            },
            PrimitiveFunc,
        },
        val::{
            CtxVal,
            Val,
        },
    },
    types::Bool,
};

pub(crate) fn value() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Value);
    let primitive = Primitive::<CtxFreeFn>::new(names::VALUE, fn_value);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_value(input: Val) -> Val {
    input
}

pub(crate) fn eval() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new(names::EVAL, fn_eval);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_eval(mut ctx: CtxForMutableFn, input: Val) -> Val {
    Eval.eval(&mut ctx, input)
}

pub(crate) fn eval_quote() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new(names::QUOTE, fn_eval_quote);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_eval_quote(mut ctx: CtxForMutableFn, input: Val) -> Val {
    QUOTE.eval(&mut ctx, input)
}

pub(crate) fn eval_twice() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::EVAL_TWICE,
        fn_eval_twice::<FreeCtx>,
        |ctx, val| fn_eval_twice(ctx, val),
        |ctx, val| fn_eval_twice(ctx, val),
    );
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_eval_twice<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let val = Eval.eval(&mut ctx, input);
    Eval.eval(&mut ctx, val)
}

pub(crate) fn eval_thrice() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::EVAL_THRICE,
        fn_eval_thrice::<FreeCtx>,
        |ctx, val| fn_eval_thrice(ctx, val),
        |ctx, val| fn_eval_thrice(ctx, val),
    );
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_eval_thrice<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let val1 = Eval.eval(&mut ctx, input);
    let val2 = Eval.eval(&mut ctx, val1);
    Eval.eval(&mut ctx, val2)
}

pub(crate) fn eval_free() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Quote);
    let primitive = Primitive::<CtxFreeFn>::new(names::EVAL_FREE, fn_eval_free);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_eval_free(input: Val) -> Val {
    Eval.eval(&mut FreeCtx, input)
}

pub(crate) fn eval_const() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Quote, BasicEvalMode::Quote)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxConstFn>::new(names::EVAL_CONST, fn_eval_const);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_eval_const(ctx: CtxForConstFn, input: Val) -> Val {
    fn_eval_in_ctx(ctx, input)
}

pub(crate) fn eval_mutable() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Quote, BasicEvalMode::Quote)),
        default: BasicEvalMode::Value,
    };
    let primitive =
        Primitive::<CtxMutableFn>::new(names::EVAL_MUTABLE, |ctx, val| fn_eval_in_ctx(ctx, val));
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_eval_in_ctx<Ctx: CtxTrait>(mut ctx: Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let name_or_val = pair.first;
    let val = pair.second;
    DefaultCtx.get_tagged_ref(&mut ctx, name_or_val, |target_ctx| {
        let TaggedRef {
            val_ref: Val::Ctx(CtxVal(target_ctx)),
            is_const: target_ctx_const,
        } = target_ctx
        else {
            return Val::default();
        };
        if target_ctx_const {
            Eval.eval(&mut ConstCtx(target_ctx), val)
        } else {
            Eval.eval(&mut MutableCtx(target_ctx), val)
        }
    })
}

pub(crate) fn is_ctx_free() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Quote);
    let primitive = Primitive::<CtxFreeFn>::new(names::IS_CTX_FREE, fn_is_ctx_free);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_is_ctx_free(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let Some(eval_mode) = parse_eval_mode(&mut map) else {
        return Val::default();
    };
    let value = map_remove(&mut map, "value");
    let is_ctx_free = eval_mode.is_free(&mut FreeCtx, value);
    Val::Bool(Bool::new(is_ctx_free))
}

pub(crate) fn is_ctx_const() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Quote);
    let primitive = Primitive::<CtxConstFn>::new(names::IS_CTX_CONST, fn_is_ctx_const);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_is_ctx_const(mut ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let Some(eval_mode) = parse_eval_mode(&mut map) else {
        return Val::default();
    };
    let value = map_remove(&mut map, "value");
    let target_ctx = map_remove(&mut map, "context");
    if target_ctx.is_unit() {
        let is_ctx_const = eval_mode.is_const(&mut ctx, value);
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
        let is_ctx_const = eval_mode.is_const(&mut ConstCtx(target_ctx), value);
        Val::Bool(Bool::new(is_ctx_const))
    })
}
