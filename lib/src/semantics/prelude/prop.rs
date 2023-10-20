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
            CtxFreeFn,
            FuncEval,
            Primitive,
        },
        logic::{
            Prop,
            PropCtx,
            Truth,
        },
        prelude::{
            names,
            utils::{
                map_remove,
                symbol,
            },
            PrimitiveFunc,
        },
        val::{
            CtxVal,
            MapVal,
            PropVal,
        },
        Val,
    },
    types::{
        Bool,
        Map,
        Reader,
        Unit,
    },
};

const FUNCTION: &str = "function";
const INPUT: &str = "input";
const OUTPUT: &str = "output";
const CTX: &str = "context";
const BEFORE: &str = "before";
const AFTER: &str = "after";
const TRUTH: &str = "truth";

pub(crate) fn prop_new() -> PrimitiveFunc<CtxFreeFn> {
    let mut map = Map::default();
    map.insert(symbol(FUNCTION), EvalMode::Any(BasicEvalMode::Eval));
    map.insert(symbol(INPUT), EvalMode::Any(BasicEvalMode::Eval));
    map.insert(symbol(OUTPUT), EvalMode::Any(BasicEvalMode::Eval));
    map.insert(symbol(CTX), EvalMode::Any(BasicEvalMode::Eval));
    map.insert(symbol(BEFORE), EvalMode::Any(BasicEvalMode::Eval));
    map.insert(symbol(AFTER), EvalMode::Any(BasicEvalMode::Eval));
    let eval_mode = EvalMode::MapForSome(map);
    let primitive = Primitive::<CtxFreeFn>::new(names::PROP_NEW, fn_prop_new);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_prop_new(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let Val::Func(func) = map_remove(&mut map, FUNCTION) else {
        return Val::default();
    };
    let input = map_remove(&mut map, INPUT);
    let output = map_remove(&mut map, OUTPUT);
    match &func.0.evaluator {
        FuncEval::Free(_) => {
            let prop = Prop::new_free(func, input, output);
            Val::Prop(PropVal(Reader::new(prop)))
        }
        FuncEval::Const(_) => {
            let Val::Ctx(CtxVal(ctx)) = map_remove(&mut map, CTX) else {
                return Val::default();
            };
            let prop = Prop::new_const(func, *ctx, input, output);
            Val::Prop(PropVal(Reader::new(prop)))
        }
        FuncEval::Mutable(_) => {
            let Val::Ctx(CtxVal(before)) = map_remove(&mut map, BEFORE) else {
                return Val::default();
            };
            let Val::Ctx(CtxVal(after)) = map_remove(&mut map, AFTER) else {
                return Val::default();
            };
            let prop = Prop::new_mutable(func, *before, input, *after, output);
            Val::Prop(PropVal(Reader::new(prop)))
        }
    }
}

pub(crate) fn prop_repr() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::Any(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::PROP_REPR, fn_prop_repr);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_prop_repr(input: Val) -> Val {
    let Val::Prop(PropVal(prop)) = input else {
        return Val::default();
    };
    let mut repr = MapVal::default();
    generate_prop(&mut repr, &prop);
    Val::Map(repr)
}

fn generate_prop(repr: &mut MapVal, prop: &Prop) {
    repr.insert(symbol(FUNCTION), Val::Func(prop.func().clone()));
    repr.insert(symbol(INPUT), prop.input().clone());
    repr.insert(symbol(OUTPUT), prop.output().clone());
    match prop.ctx() {
        PropCtx::Free => {}
        PropCtx::Const(ctx) => {
            repr.insert(symbol(CTX), Val::Ctx(CtxVal(Box::new(ctx.clone()))));
        }
        PropCtx::Mutable(before, after) => {
            repr.insert(symbol(BEFORE), Val::Ctx(CtxVal(Box::new(before.clone()))));
            repr.insert(symbol(AFTER), Val::Ctx(CtxVal(Box::new(after.clone()))));
        }
    }
    match prop.truth() {
        Truth::True => {
            repr.insert(symbol(TRUTH), Val::Bool(Bool::t()));
        }
        Truth::False => {
            repr.insert(symbol(TRUTH), Val::Bool(Bool::f()));
        }
        _ => {}
    };
}

pub(crate) fn get_truth() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Symbol(BasicEvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::PROP_TRUTH, fn_get_truth);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_truth(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Prop(PropVal(prop)) = val else {
            return Val::default();
        };
        match prop.truth() {
            Truth::None => Val::Unit(Unit),
            Truth::True => Val::Bool(Bool::t()),
            Truth::False => Val::Bool(Bool::f()),
        }
    })
}

pub(crate) fn get_function() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Symbol(BasicEvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::PROP_FUNCTION, fn_get_function);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_function(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Prop(PropVal(prop)) = val else {
            return Val::default();
        };
        Val::Func(prop.func().clone())
    })
}

pub(crate) fn get_input() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Symbol(BasicEvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::PROP_INPUT, fn_get_input);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_input(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Prop(PropVal(prop)) = val else {
            return Val::default();
        };
        prop.input().clone()
    })
}

pub(crate) fn get_output() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Symbol(BasicEvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::PROP_OUTPUT, fn_get_output);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_output(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Prop(PropVal(prop)) = val else {
            return Val::default();
        };
        prop.output().clone()
    })
}

pub(crate) fn get_before() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Symbol(BasicEvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::PROP_CTX_BEFORE, fn_get_before);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_before(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Prop(PropVal(prop)) = val else {
            return Val::default();
        };
        let ctx = prop.ctx();
        match ctx {
            PropCtx::Free => Val::default(),
            PropCtx::Const(ctx) => Val::Ctx(CtxVal(Box::new(ctx.clone()))),
            PropCtx::Mutable(before, _) => Val::Ctx(CtxVal(Box::new(before.clone()))),
        }
    })
}

pub(crate) fn get_after() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Symbol(BasicEvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::PROP_CTX_AFTER, fn_get_after);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_after(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Prop(PropVal(prop)) = val else {
            return Val::default();
        };
        let ctx = prop.ctx();
        match ctx {
            PropCtx::Free => Val::default(),
            PropCtx::Const(ctx) => Val::Ctx(CtxVal(Box::new(ctx.clone()))),
            PropCtx::Mutable(_, after) => Val::Ctx(CtxVal(Box::new(after.clone()))),
        }
    })
}
