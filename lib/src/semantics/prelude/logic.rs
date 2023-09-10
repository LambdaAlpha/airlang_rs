use crate::{
    semantics::{
        eval_mode::{
            BasicEvalMode,
            EvalMode,
        },
        func::{
            CtxFreeFn,
            Primitive,
        },
        logic::{
            Prop,
            PropCtx,
            Theorem,
        },
        prelude::{
            names,
            utils::{
                basic_eval_mode_to_symbol,
                const_ref_val,
                map_remove,
                parse_eval_mode,
            },
            PrimitiveFunc,
        },
        val::{
            PropVal,
            TheoremVal,
        },
        Val,
    },
    types::{
        Bool,
        Pair,
        Reader,
        Symbol,
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
    let Some(eval_mode) = parse_eval_mode(&mut map) else {
        return Val::default();
    };
    let input = const_ref_val(map_remove(&mut map, "input"));
    let output = const_ref_val(map_remove(&mut map, "output"));
    let access = map_remove(&mut map, "access");
    let access = match &access {
        Val::Symbol(s) => &**s,
        Val::Unit(_) => "mutable",
        _ => return Val::default(),
    };
    match access {
        "free" => {
            let Some(prop) = Prop::new_free(eval_mode, input, output) else {
                return Val::default();
            };
            Val::Prop(PropVal(Reader::new(prop)))
        }
        "const" => {
            let ctx = const_ref_val(map_remove(&mut map, "context"));
            let Some(prop) = Prop::new_const(eval_mode, ctx, input, output) else {
                return Val::default();
            };
            Val::Prop(PropVal(Reader::new(prop)))
        }
        "mutable" => {
            let before = const_ref_val(map_remove(&mut map, "before"));
            let after = const_ref_val(map_remove(&mut map, "after"));
            let Some(prop) = Prop::new_mutable(eval_mode, before, input, after, output) else {
                return Val::default();
            };
            Val::Prop(PropVal(Reader::new(prop)))
        }
        _ => Val::default(),
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
    let Some(eval_mode) = parse_eval_mode(&mut map) else {
        return Val::default();
    };
    let input = const_ref_val(map_remove(&mut map, "input"));
    let access = map_remove(&mut map, "access");
    let access = match &access {
        Val::Symbol(s) => &**s,
        Val::Unit(_) => "mutable",
        _ => return Val::default(),
    };
    match access {
        "free" => {
            let Some(theorem) = Theorem::new_free(eval_mode, input) else {
                return Val::default();
            };
            Val::Theorem(TheoremVal(Reader::new(theorem)))
        }
        "const" => {
            let ctx = const_ref_val(map_remove(&mut map, "context"));
            let Some(theorem) = Theorem::new_const(eval_mode, ctx, input) else {
                return Val::default();
            };
            Val::Theorem(TheoremVal(Reader::new(theorem)))
        }
        "mutable" => {
            let ctx = const_ref_val(map_remove(&mut map, "context"));
            let Some(theorem) = Theorem::new_mutable(eval_mode, ctx, input) else {
                return Val::default();
            };
            Val::Theorem(TheoremVal(Reader::new(theorem)))
        }
        _ => Val::default(),
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
    let Some(theorem) = Theorem::prove(Prop::clone(&*prop)) else {
        return Val::default();
    };
    Val::Theorem(TheoremVal(Reader::new(theorem)))
}

pub(crate) fn relax() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Eval, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_RELAX, fn_relax);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_relax(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Map(mut map) = pair.second else {
        return Val::default();
    };
    let Val::Symbol(access) = map_remove(&mut map, "access") else {
        return Val::default();
    };
    let is_const = match &*access {
        "const" => true,
        "mutable" => false,
        _ => return Val::default(),
    };
    let ctx = map_remove(&mut map, "context");
    match pair.first {
        Val::Prop(PropVal(prop)) => {
            if is_const {
                let ctx = const_ref_val(ctx);
                let Some(new_prop) = prop.relax_to_const(ctx) else {
                    return Val::default();
                };
                Val::Prop(PropVal(Reader::new(new_prop)))
            } else {
                let ctx = if ctx.is_unit() {
                    None
                } else {
                    Some(const_ref_val(ctx))
                };
                let Some(new_prop) = prop.relax_to_mutable(ctx) else {
                    return Val::default();
                };
                Val::Prop(PropVal(Reader::new(new_prop)))
            }
        }
        Val::Theorem(TheoremVal(theorem)) => {
            if is_const {
                let ctx = const_ref_val(ctx);
                let Some(new_theorem) = theorem.relax_to_const(ctx) else {
                    return Val::default();
                };
                Val::Theorem(TheoremVal(Reader::new(new_theorem)))
            } else {
                let ctx = if ctx.is_unit() {
                    None
                } else {
                    Some(const_ref_val(ctx))
                };
                let Some(new_theorem) = theorem.relax_to_mutable(ctx) else {
                    return Val::default();
                };
                Val::Theorem(TheoremVal(Reader::new(new_theorem)))
            }
        }
        _ => Val::default(),
    }
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

pub(crate) fn get_access() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_ACCESS, fn_get_access);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_access(input: Val) -> Val {
    let prop = match &input {
        Val::Theorem(TheoremVal(theorem)) => theorem.prop(),
        Val::Prop(PropVal(prop)) => prop,
        _ => return Val::default(),
    };
    let access = match prop.ctx() {
        PropCtx::Free => "free",
        PropCtx::Const(_) => "const",
        PropCtx::Mutable(_, _) => "mutable",
    };
    Val::Symbol(Symbol::from_str(access))
}

pub(crate) fn get_eval_mode() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_EVAL_MODE, fn_get_eval_mode);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_eval_mode(input: Val) -> Val {
    let prop = match &input {
        Val::Theorem(TheoremVal(theorem)) => theorem.prop(),
        Val::Prop(PropVal(prop)) => prop,
        _ => return Val::default(),
    };
    let eval_mode = prop.eval_mode();
    let s = basic_eval_mode_to_symbol(eval_mode.default);
    Val::Symbol(s)
}

pub(crate) fn get_pair_eval_mode() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_PAIR_EVAL_MODE, fn_get_pair_eval_mode);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_pair_eval_mode(input: Val) -> Val {
    let prop = match &input {
        Val::Theorem(TheoremVal(theorem)) => theorem.prop(),
        Val::Prop(PropVal(prop)) => prop,
        _ => return Val::default(),
    };
    let eval_mode = prop.eval_mode();
    let (first, second) = match eval_mode.pair {
        None => {
            let s = basic_eval_mode_to_symbol(eval_mode.default);
            (s.clone(), s)
        }
        Some((first, second)) => {
            let first = basic_eval_mode_to_symbol(first);
            let second = basic_eval_mode_to_symbol(second);
            (first, second)
        }
    };
    Val::Pair(Box::new(Pair::new(Val::Symbol(first), Val::Symbol(second))))
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
    Val::Ref(prop.input().clone())
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
    Val::Ref(prop.output().clone())
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
        PropCtx::Const(ctx) => Val::Ref(ctx.clone()),
        PropCtx::Mutable(input, _) => Val::Ref(input.clone()),
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
        PropCtx::Const(ctx) => Val::Ref(ctx.clone()),
        PropCtx::Mutable(_, output) => Val::Ref(output.clone()),
    }
}
