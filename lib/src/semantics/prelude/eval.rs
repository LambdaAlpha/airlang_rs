use crate::semantics::{
    ctx::NameMap,
    ctx_access::mutable::CtxForMutableFn,
    eval::Evaluator,
    eval_mode::{
        eval::Eval,
        EvalMode,
        BY_VAL,
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
    pub(crate) value: Named<FuncVal>,
    pub(crate) eval: Named<FuncVal>,
    pub(crate) mix: Named<FuncVal>,
}

impl Default for EvalPrelude {
    fn default() -> Self {
        EvalPrelude {
            value: value(),
            eval: eval(),
            mix: mix(),
        }
    }
}

impl Prelude for EvalPrelude {
    fn put(&self, m: &mut NameMap) {
        self.value.put(m);
        self.eval.put(m);
        self.mix.put(m);
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
