use crate::{
    semantics::{
        ctx::{
            Ctx,
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
        eval::{
            input::ByVal,
            Evaluator,
        },
        eval_mode::{
            eval::{
                Eval,
                EvalByRef,
            },
            BasicEvalMode,
            EvalMode,
        },
        func::{
            Composed,
            CtxConstFn,
            CtxConstInfo,
            CtxFreeFn,
            CtxFreeInfo,
            CtxMutableFn,
            CtxMutableInfo,
            Func,
            FuncEval,
            FuncImpl,
            Primitive,
        },
        prelude::{
            names,
            utils,
            PrimitiveFunc,
        },
        val::{
            CtxVal,
            Val,
        },
    },
    types::{
        Bool,
        Either,
        Keeper,
        Reader,
        Str,
        Symbol,
    },
};

pub(crate) fn value() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Value);
    let primitive = Primitive::<CtxFreeFn>::new(names::VALUE, fn_value);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_value(input: Val) -> Val {
    input
}

pub(crate) fn eval() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::EVAL, fn_eval);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_eval(input: Val) -> Val {
    input
}

pub(crate) fn eval_quote() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Quote);
    let primitive = Primitive::<CtxFreeFn>::new(names::EVAL_QUOTE, fn_eval_quote);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_eval_quote(input: Val) -> Val {
    input
}

pub(crate) fn eval_inline() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxFreeFn>::new(names::EVAL_INLINE, fn_eval_inline);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_eval_inline(input: Val) -> Val {
    input
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
    match input {
        Val::Ref(k) => {
            let Ok(input) = Keeper::reader(&k.0) else {
                return Val::default();
            };
            EvalByRef.eval(&mut ctx, &input.val)
        }
        i => {
            let val = Eval.eval(&mut ctx, i);
            Eval.eval(&mut ctx, val)
        }
    }
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
    let val = Eval.eval(&mut ctx, input);
    fn_eval_twice::<Ctx>(ctx, val)
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
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Quote)),
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
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Quote)),
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
    DefaultCtx.get_ref_val_or_default(&mut ctx, name_or_val, |target_ctx| match target_ctx {
        Either::Left(target) => {
            let TaggedRef {
                val_ref: Val::Ctx(CtxVal(target_ctx)),
                is_const: target_ctx_const,
            } = target
            else {
                return Val::default();
            };
            if target_ctx_const {
                Eval.eval(&mut ConstCtx(target_ctx), val)
            } else {
                Eval.eval(&mut MutableCtx(target_ctx), val)
            }
        }
        Either::Right(target) => {
            let Val::Ctx(CtxVal(mut target_ctx)) = target else {
                return Val::default();
            };
            Eval.eval(&mut MutableCtx(&mut target_ctx), val)
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
    let Some(eval_mode) = utils::parse_eval_mode(&mut map) else {
        return Val::default();
    };
    let value = utils::map_remove(&mut map, "value");
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
    let Some(eval_mode) = utils::parse_eval_mode(&mut map) else {
        return Val::default();
    };
    let value = utils::map_remove(&mut map, "value");
    let target_ctx = utils::map_remove(&mut map, "context");
    if target_ctx.is_unit() {
        let is_ctx_const = eval_mode.is_const(&mut ctx, value);
        return Val::Bool(Bool::new(is_ctx_const));
    }
    DefaultCtx.get_ref_val_or_default(&mut ctx, target_ctx, |target_ctx| match target_ctx {
        Either::Left(target) => {
            let TaggedRef {
                val_ref: Val::Ctx(CtxVal(target_ctx)),
                ..
            } = target
            else {
                return Val::default();
            };
            let is_ctx_const = eval_mode.is_const(&mut ConstCtx(target_ctx), value);
            Val::Bool(Bool::new(is_ctx_const))
        }
        Either::Right(target) => {
            let Val::Ctx(CtxVal(mut target_ctx)) = target else {
                return Val::default();
            };
            let is_ctx_const = eval_mode.is_const(&mut ConstCtx(&mut target_ctx), value);
            Val::Bool(Bool::new(is_ctx_const))
        }
    })
}

pub(crate) fn parse() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::PARSE, fn_parse);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_parse(input: Val) -> Val {
    let Val::String(input) = input else {
        return Val::default();
    };
    crate::semantics::parse(&input).unwrap_or_default()
}

pub(crate) fn stringify() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::STRINGIFY, fn_stringify);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_stringify(input: Val) -> Val {
    let Ok(str) = crate::semantics::generate(&input) else {
        return Val::default();
    };
    Val::String(Str::from(str))
}

pub(crate) fn func() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Quote);
    let primitive = Primitive::<CtxFreeFn>::new(names::FUNC, fn_func);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_func(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let body = utils::map_remove(&mut map, "body");
    let func_ctx = match utils::map_remove(&mut map, "context") {
        Val::Ctx(func_ctx) => *func_ctx.0,
        Val::Unit(_) => Ctx::default(),
        _ => return Val::default(),
    };
    let input_name = match utils::map_remove(&mut map, "input") {
        Val::Symbol(name) => name,
        Val::Unit(_) => Symbol::from_str("input"),
        _ => return Val::default(),
    };
    let Some(eval_mode) = utils::parse_eval_mode(&mut map) else {
        return Val::default();
    };
    let caller_name = match utils::map_remove(&mut map, "caller_name") {
        Val::Symbol(name) => name,
        Val::Unit(_) => Symbol::from_str("caller"),
        _ => return Val::default(),
    };
    let caller_access = utils::map_remove(&mut map, "caller_access");
    let caller_access = match &caller_access {
        Val::Symbol(s) => &**s,
        Val::Unit(_) => "free",
        _ => return Val::default(),
    };
    let evaluator = match caller_access {
        "free" => FuncEval::Free(FuncImpl::Composed(Composed {
            body,
            ctx: func_ctx,
            input_name,
            caller: CtxFreeInfo {},
        })),
        "const" => FuncEval::Const(FuncImpl::Composed(Composed {
            body,
            ctx: func_ctx,
            input_name,
            caller: CtxConstInfo { name: caller_name },
        })),
        "mutable" => FuncEval::Mutable(FuncImpl::Composed(Composed {
            body,
            ctx: func_ctx,
            input_name,
            caller: CtxMutableInfo { name: caller_name },
        })),
        _ => return Val::default(),
    };
    let func = Func::new(eval_mode, evaluator);
    Val::Func(Reader::new(func).into())
}

pub(crate) fn chain() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new(names::CHAIN, fn_chain);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_chain(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Eval.eval_call(&mut ctx, pair.second, pair.first)
}
