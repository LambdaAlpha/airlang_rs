use crate::{
    semantics::{
        eval_mode::{
            BasicEvalMode,
            EvalMode,
        },
        func::{
            CtxFreeFn,
            FuncEval,
            Primitive,
        },
        logic::{
            Prop,
            PropCtx,
            Theorem,
        },
        prelude::{
            names,
            utils::map_remove,
            PrimitiveFunc,
        },
        val::{
            CtxVal,
            PropVal,
            TheoremVal,
        },
        Val,
    },
    types::{
        Bool,
        Reader,
    },
};

pub(crate) fn new_prop() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_NEW_PROP, fn_new_prop);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_new_prop(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let Val::Func(func) = map_remove(&mut map, "function") else {
        return Val::default();
    };
    let input = map_remove(&mut map, "input");
    let output = map_remove(&mut map, "output");
    match &func.0.evaluator {
        FuncEval::Free(_) => {
            let prop = Prop::new_free(func, input, output);
            Val::Prop(PropVal(Reader::new(prop)))
        }
        FuncEval::Const(_) => {
            let Val::Ctx(CtxVal(ctx)) = map_remove(&mut map, "context") else {
                return Val::default();
            };
            let prop = Prop::new_const(func, *ctx, input, output);
            Val::Prop(PropVal(Reader::new(prop)))
        }
        FuncEval::Mutable(_) => {
            let Val::Ctx(CtxVal(before)) = map_remove(&mut map, "before") else {
                return Val::default();
            };
            let Val::Ctx(CtxVal(after)) = map_remove(&mut map, "after") else {
                return Val::default();
            };
            let prop = Prop::new_mutable(func, *before, input, *after, output);
            Val::Prop(PropVal(Reader::new(prop)))
        }
    }
}

pub(crate) fn new_theorem() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_NEW_THEOREM, fn_new_theorem);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_new_theorem(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let Val::Func(func) = map_remove(&mut map, "function") else {
        return Val::default();
    };
    let input = map_remove(&mut map, "input");
    match &func.0.evaluator {
        FuncEval::Free(_) => {
            let theorem = Theorem::new_free(func, input);
            Val::Theorem(TheoremVal(Reader::new(theorem)))
        }
        FuncEval::Const(_) => {
            let Val::Ctx(CtxVal(ctx)) = map_remove(&mut map, "context") else {
                return Val::default();
            };
            let theorem = Theorem::new_const(func, *ctx, input);
            Val::Theorem(TheoremVal(Reader::new(theorem)))
        }
        FuncEval::Mutable(_) => {
            let Val::Ctx(CtxVal(before)) = map_remove(&mut map, "context") else {
                return Val::default();
            };
            let theorem = Theorem::new_mutable(func, *before, input);
            Val::Theorem(TheoremVal(Reader::new(theorem)))
        }
    }
}

pub(crate) fn prove() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_PROVE, fn_prove);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_prove(input: Val) -> Val {
    let Val::Prop(PropVal(prop)) = input else {
        return Val::default();
    };
    let theorem = Theorem::prove(Prop::clone(&*prop));
    Val::Theorem(TheoremVal(Reader::new(theorem)))
}

pub(crate) fn is_true() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_IS_TRUE, fn_is_true);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_is_true(input: Val) -> Val {
    let Val::Theorem(TheoremVal(theorem)) = input else {
        return Val::default();
    };
    Val::Bool(Bool::new(theorem.is_true()))
}

pub(crate) fn get_function() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_FUNCTION, fn_get_function);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_function(input: Val) -> Val {
    let prop = match &input {
        Val::Theorem(TheoremVal(theorem)) => theorem.prop(),
        Val::Prop(PropVal(prop)) => prop,
        _ => return Val::default(),
    };
    Val::Func(prop.func().clone())
}

pub(crate) fn get_input() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_INPUT, fn_get_input);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_input(input: Val) -> Val {
    let prop = match &input {
        Val::Theorem(TheoremVal(theorem)) => theorem.prop(),
        Val::Prop(PropVal(prop)) => prop,
        _ => return Val::default(),
    };
    prop.input().clone()
}

pub(crate) fn get_output() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_OUTPUT, fn_get_output);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_output(input: Val) -> Val {
    let prop = match &input {
        Val::Theorem(TheoremVal(theorem)) => theorem.prop(),
        Val::Prop(PropVal(prop)) => prop,
        _ => return Val::default(),
    };
    prop.output().clone()
}

pub(crate) fn get_before() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_CTX_BEFORE, fn_get_before);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_before(input: Val) -> Val {
    let prop = match &input {
        Val::Theorem(TheoremVal(theorem)) => theorem.prop(),
        Val::Prop(PropVal(prop)) => prop,
        _ => return Val::default(),
    };
    let ctx = prop.ctx();
    match ctx {
        PropCtx::Free => Val::default(),
        PropCtx::Const(ctx) => Val::Ctx(CtxVal(Box::new(ctx.clone()))),
        PropCtx::Mutable(before, _) => Val::Ctx(CtxVal(Box::new(before.clone()))),
    }
}

pub(crate) fn get_after() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_CTX_AFTER, fn_get_after);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_after(input: Val) -> Val {
    let prop = match &input {
        Val::Theorem(TheoremVal(theorem)) => theorem.prop(),
        Val::Prop(PropVal(prop)) => prop,
        _ => return Val::default(),
    };
    let ctx = prop.ctx();
    match ctx {
        PropCtx::Free => Val::default(),
        PropCtx::Const(ctx) => Val::Ctx(CtxVal(Box::new(ctx.clone()))),
        PropCtx::Mutable(_, after) => Val::Ctx(CtxVal(Box::new(after.clone()))),
    }
}
