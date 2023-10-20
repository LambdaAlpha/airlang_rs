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
        logic::Prop,
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
            PropVal,
        },
        Val,
    },
    types::{
        Map,
        Reader,
    },
};

const FUNCTION: &str = "function";
const INPUT: &str = "input";
const CTX: &str = "context";
const BEFORE: &str = "before";

pub(crate) fn theorem_new() -> PrimitiveFunc<CtxFreeFn> {
    let mut map = Map::default();
    map.insert(symbol(FUNCTION), EvalMode::Any(BasicEvalMode::Eval));
    map.insert(symbol(INPUT), EvalMode::Any(BasicEvalMode::Eval));
    map.insert(symbol(CTX), EvalMode::Any(BasicEvalMode::Eval));
    map.insert(symbol(BEFORE), EvalMode::Any(BasicEvalMode::Eval));
    let eval_mode = EvalMode::MapForSome(map);
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_THEOREM_NEW, fn_theorem_new);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_theorem_new(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let Val::Func(func) = map_remove(&mut map, FUNCTION) else {
        return Val::default();
    };
    let input = map_remove(&mut map, INPUT);
    match &func.0.evaluator {
        FuncEval::Free(_) => {
            let theorem = Prop::new_free_theorem(func, input);
            Val::Prop(PropVal(Reader::new(theorem)))
        }
        FuncEval::Const(_) => {
            let Val::Ctx(CtxVal(ctx)) = map_remove(&mut map, CTX) else {
                return Val::default();
            };
            let theorem = Prop::new_const_theorem(func, *ctx, input);
            Val::Prop(PropVal(Reader::new(theorem)))
        }
        FuncEval::Mutable(_) => {
            let Val::Ctx(CtxVal(before)) = map_remove(&mut map, BEFORE) else {
                return Val::default();
            };
            let theorem = Prop::new_mutable_theorem(func, *before, input);
            Val::Prop(PropVal(Reader::new(theorem)))
        }
    }
}

pub(crate) fn prove() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::Any(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::LOGIC_PROVE, fn_prove);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_prove(input: Val) -> Val {
    let Val::Prop(PropVal(prop)) = input else {
        return Val::default();
    };
    let theorem = Prop::prove(Prop::clone(&*prop));
    Val::Prop(PropVal(Reader::new(theorem)))
}
