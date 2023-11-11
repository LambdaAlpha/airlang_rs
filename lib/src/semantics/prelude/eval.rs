use crate::semantics::{
    ctx::NameMap,
    ctx_access::{
        free::FreeCtx,
        mutable::CtxForMutableFn,
        CtxAccessor,
    },
    eval::Evaluator,
    eval_mode::{
        eval::Eval,
        EvalMode,
        BY_VAL,
    },
    func::{
        CtxMutableFn,
        Primitive,
    },
    input_mode::InputMode,
    prelude::{
        named_free_fn,
        named_mutable_fn,
        Named,
        Prelude,
    },
    val::{
        FuncVal,
        Val,
    },
};

#[derive(Clone)]
pub(crate) struct EvalPrelude {
    value: Named<FuncVal>,
    eval: Named<FuncVal>,
    mix: Named<FuncVal>,
    eval_twice: Named<FuncVal>,
    eval_thrice: Named<FuncVal>,
}

impl Default for EvalPrelude {
    fn default() -> Self {
        EvalPrelude {
            value: value(),
            eval: eval(),
            mix: mix(),
            eval_twice: eval_twice(),
            eval_thrice: eval_thrice(),
        }
    }
}

impl Prelude for EvalPrelude {
    fn put(&self, m: &mut NameMap) {
        self.value.put(m);
        self.eval.put(m);
        self.mix.put(m);
        self.eval_twice.put(m);
        self.eval_thrice.put(m);
    }
}

pub(crate) const VALUE: &str = "`0";
pub(crate) const EVAL: &str = "`1";
pub(crate) const MIX: &str = "``";

fn value() -> Named<FuncVal> {
    let input_mode = InputMode::Any(EvalMode::Value);
    named_free_fn(VALUE, input_mode, fn_value)
}

fn fn_value(input: Val) -> Val {
    input
}

fn eval() -> Named<FuncVal> {
    let input_mode = InputMode::Any(EvalMode::Value);
    named_mutable_fn(EVAL, input_mode, fn_eval)
}

fn fn_eval(mut ctx: CtxForMutableFn, input: Val) -> Val {
    Eval.eval(&mut ctx, input)
}

fn mix() -> Named<FuncVal> {
    let input_mode = InputMode::Any(EvalMode::Value);
    named_mutable_fn(MIX, input_mode, fn_mix)
}

fn fn_mix(mut ctx: CtxForMutableFn, input: Val) -> Val {
    BY_VAL.mix.eval(&mut ctx, input)
}

fn eval_twice() -> Named<FuncVal> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let func = Primitive::<CtxMutableFn>::dispatch(
        fn_eval_twice::<FreeCtx>,
        |ctx, val| fn_eval_twice(ctx, val),
        |ctx, val| fn_eval_twice(ctx, val),
    );
    named_mutable_fn("`2", input_mode, func)
}

fn fn_eval_twice<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let val = Eval.eval(&mut ctx, input);
    Eval.eval(&mut ctx, val)
}

fn eval_thrice() -> Named<FuncVal> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let func = Primitive::<CtxMutableFn>::dispatch(
        fn_eval_thrice::<FreeCtx>,
        |ctx, val| fn_eval_thrice(ctx, val),
        |ctx, val| fn_eval_thrice(ctx, val),
    );
    named_mutable_fn("`3", input_mode, func)
}

fn fn_eval_thrice<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let val1 = Eval.eval(&mut ctx, input);
    let val2 = Eval.eval(&mut ctx, val1);
    Eval.eval(&mut ctx, val2)
}
