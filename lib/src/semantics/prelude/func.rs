use crate::{
    semantics::{
        ctx::{
            Ctx,
            DefaultCtx,
        },
        ctx_access::constant::CtxForConstFn,
        eval_mode::{
            BasicEvalMode,
            EvalMode,
        },
        func::{
            Composed,
            CtxConstEval,
            CtxConstFn,
            CtxConstInfo,
            CtxFreeEval,
            CtxFreeFn,
            CtxFreeInfo,
            CtxMutableEval,
            CtxMutableInfo,
            Func,
            FuncEval,
            FuncImpl,
            Primitive,
        },
        prelude::{
            names,
            utils::{
                generate_eval_mode,
                map_remove,
                parse_eval_mode,
                symbol,
            },
            PrimitiveFunc,
        },
        val::{
            CtxVal,
            FuncVal,
            MapVal,
            Val,
        },
    },
    types::{
        Bool,
        Map,
        Reader,
        Symbol,
    },
};

const BODY: &str = "body";
const CTX: &str = "context";
const INPUT_NAME: &str = "input_name";
const CALLER_NAME: &str = "caller_name";
const CALLER_ACCESS: &str = "caller_access";
const ID: &str = "id";
const EVAL_MODE: &str = "eval_mode";

const DEFAULT_INPUT_NAME: &str = "input";
const DEFAULT_CALLER_NAME: &str = "caller";
const FREE: &str = "free";
const CONST: &str = "constant";
const MUTABLE: &str = "mutable";

pub(crate) fn func_new() -> PrimitiveFunc<CtxFreeFn> {
    let mut map = Map::default();
    map.insert(symbol(BODY), EvalMode::Any(BasicEvalMode::Quote));
    map.insert(symbol(CTX), EvalMode::Any(BasicEvalMode::Eval));
    map.insert(symbol(INPUT_NAME), EvalMode::Symbol(BasicEvalMode::Value));
    map.insert(symbol(CALLER_NAME), EvalMode::Symbol(BasicEvalMode::Value));
    map.insert(
        symbol(CALLER_ACCESS),
        EvalMode::Symbol(BasicEvalMode::Value),
    );
    map.insert(symbol(EVAL_MODE), EvalMode::Any(BasicEvalMode::Quote));
    let eval_mode = EvalMode::MapForSome(map);
    let primitive = Primitive::<CtxFreeFn>::new(names::FUNC_NEW, fn_func_new);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_func_new(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let body = map_remove(&mut map, BODY);
    let func_ctx = match map_remove(&mut map, CTX) {
        Val::Ctx(func_ctx) => *func_ctx.0,
        Val::Unit(_) => Ctx::default(),
        _ => return Val::default(),
    };
    let input_name = match map_remove(&mut map, INPUT_NAME) {
        Val::Symbol(name) => name,
        Val::Unit(_) => Symbol::from_str(DEFAULT_INPUT_NAME),
        _ => return Val::default(),
    };
    let Some(eval_mode) = parse_eval_mode(map_remove(&mut map, EVAL_MODE)) else {
        return Val::default();
    };
    let caller_name = match map_remove(&mut map, CALLER_NAME) {
        Val::Symbol(name) => name,
        Val::Unit(_) => Symbol::from_str(DEFAULT_CALLER_NAME),
        _ => return Val::default(),
    };
    let caller_access = map_remove(&mut map, CALLER_ACCESS);
    let caller_access = match &caller_access {
        Val::Symbol(s) => &**s,
        Val::Unit(_) => FREE,
        _ => return Val::default(),
    };
    let evaluator = match caller_access {
        FREE => FuncEval::Free(FuncImpl::Composed(Composed {
            body,
            ctx: func_ctx,
            input_name,
            caller: CtxFreeInfo {},
        })),
        CONST => FuncEval::Const(FuncImpl::Composed(Composed {
            body,
            ctx: func_ctx,
            input_name,
            caller: CtxConstInfo { name: caller_name },
        })),
        MUTABLE => FuncEval::Mutable(FuncImpl::Composed(Composed {
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

pub(crate) fn func_repr() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::Any(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::FUNC_REPR, fn_func_repr);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_func_repr(input: Val) -> Val {
    let Val::Func(FuncVal(func)) = input else {
        return Val::default();
    };
    let mut repr = MapVal::default();

    if func.input_eval_mode != EvalMode::Any(BasicEvalMode::Eval) {
        let eval_mode = generate_eval_mode(&func.input_eval_mode);
        repr.insert(symbol(EVAL_MODE), eval_mode);
    }

    match &func.evaluator {
        FuncEval::Free(eval) => match eval {
            CtxFreeEval::Primitive(p) => {
                repr.insert(symbol(ID), Val::Symbol(p.get_id().clone()));
            }
            CtxFreeEval::Composed(c) => {
                repr.insert(symbol(BODY), c.body.clone());
                if c.ctx != Ctx::default() {
                    repr.insert(symbol(CTX), Val::Ctx(CtxVal(Box::new(c.ctx.clone()))));
                }
                if &*c.input_name != DEFAULT_INPUT_NAME {
                    repr.insert(symbol(INPUT_NAME), Val::Symbol(c.input_name.clone()));
                }
            }
        },
        FuncEval::Const(eval) => {
            repr.insert(symbol(CALLER_ACCESS), symbol(CONST));
            match eval {
                CtxConstEval::Primitive(p) => {
                    repr.insert(symbol(ID), Val::Symbol(p.get_id().clone()));
                }
                CtxConstEval::Composed(c) => {
                    repr.insert(symbol(BODY), c.body.clone());
                    if c.ctx != Ctx::default() {
                        repr.insert(symbol(CTX), Val::Ctx(CtxVal(Box::new(c.ctx.clone()))));
                    }
                    if &*c.input_name != DEFAULT_INPUT_NAME {
                        repr.insert(symbol(INPUT_NAME), Val::Symbol(c.input_name.clone()));
                    }
                    if &*c.caller.name != DEFAULT_CALLER_NAME {
                        repr.insert(symbol(CALLER_NAME), Val::Symbol(c.caller.name.clone()));
                    }
                }
            }
        }
        FuncEval::Mutable(eval) => {
            repr.insert(symbol(CALLER_ACCESS), symbol(MUTABLE));
            match eval {
                CtxMutableEval::Primitive(p) => {
                    repr.insert(symbol(ID), Val::Symbol(p.get_id().clone()));
                }
                CtxMutableEval::Composed(c) => {
                    repr.insert(symbol(BODY), c.body.clone());
                    if c.ctx != Ctx::default() {
                        repr.insert(symbol(CTX), Val::Ctx(CtxVal(Box::new(c.ctx.clone()))));
                    }
                    if &*c.input_name != DEFAULT_INPUT_NAME {
                        repr.insert(symbol(INPUT_NAME), Val::Symbol(c.input_name.clone()));
                    }
                    if &*c.caller.name != DEFAULT_CALLER_NAME {
                        repr.insert(symbol(CALLER_NAME), Val::Symbol(c.caller.name.clone()));
                    }
                }
            }
        }
    }
    Val::Map(repr)
}

pub(crate) fn func_access() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Symbol(BasicEvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::FUNC_ACCESS, fn_func_access);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_func_access(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let access = match &func.evaluator {
            FuncEval::Free(_) => FREE,
            FuncEval::Const(_) => CONST,
            FuncEval::Mutable(_) => MUTABLE,
        };
        Val::Symbol(Symbol::from_str(access))
    })
}

pub(crate) fn func_eval_mode() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Symbol(BasicEvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::FUNC_EVAL_MODE, fn_func_eval_mode);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_func_eval_mode(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        generate_eval_mode(&func.input_eval_mode)
    })
}

pub(crate) fn func_is_primitive() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Symbol(BasicEvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::FUNC_IS_PRIMITIVE, fn_func_is_primitive);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_func_is_primitive(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let is_primitive = func.is_primitive();
        Val::Bool(Bool::new(is_primitive))
    })
}

pub(crate) fn func_id() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Symbol(BasicEvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::FUNC_ID, fn_func_id);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_func_id(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let Some(id) = func.primitive_id() else {
            return Val::default();
        };
        Val::Symbol(id)
    })
}

pub(crate) fn func_body() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Symbol(BasicEvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::FUNC_BODY, fn_func_body);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_func_body(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let Some(body) = func.composed_body() else {
            return Val::default();
        };
        body
    })
}

pub(crate) fn func_context() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Symbol(BasicEvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::FUNC_CTX, fn_func_context);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_func_context(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let Some(ctx) = func.composed_context() else {
            return Val::default();
        };
        Val::Ctx(CtxVal(Box::new(ctx)))
    })
}

pub(crate) fn func_input_name() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Symbol(BasicEvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::FUNC_INPUT_NAME, fn_func_input_name);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_func_input_name(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let Some(name) = func.composed_input_name() else {
            return Val::default();
        };
        Val::Symbol(name)
    })
}

pub(crate) fn func_caller_name() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Symbol(BasicEvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::FUNC_CALLER_NAME, fn_func_caller_name);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_func_caller_name(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Func(FuncVal(func)) = val else {
            return Val::default();
        };
        let Some(name) = func.composed_caller_name() else {
            return Val::default();
        };
        Val::Symbol(name)
    })
}
