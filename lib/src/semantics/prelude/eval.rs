use crate::semantics::{
    ctx::NameMap,
    ctx_access::{
        free::FreeCtx,
        mutable::CtxForMutableFn,
    },
    eval::Evaluator,
    eval_mode::{
        less::Less,
        more::More,
        value::Value,
        EvalMode,
    },
    io_mode::IoMode,
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
    pub(crate) less: Named<FuncVal>,
    pub(crate) more: Named<FuncVal>,
}

impl Default for EvalPrelude {
    fn default() -> Self {
        EvalPrelude {
            value: value(),
            less: less(),
            more: more(),
        }
    }
}

impl Prelude for EvalPrelude {
    fn put(&self, m: &mut NameMap) {
        self.value.put(m);
        self.less.put(m);
        self.more.put(m);
    }
}

pub(crate) const VALUE: &str = "'";
pub(crate) const LESS: &str = "'f";
pub(crate) const MORE: &str = "'t";

fn value() -> Named<FuncVal> {
    let input_mode = IoMode::Any(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_free_fn(VALUE, input_mode, output_mode, fn_value)
}

fn fn_value(input: Val) -> Val {
    Value.eval(&mut FreeCtx, input)
}

fn less() -> Named<FuncVal> {
    let input_mode = IoMode::Any(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_mutable_fn(LESS, input_mode, output_mode, fn_less)
}

fn fn_less(mut ctx: CtxForMutableFn, input: Val) -> Val {
    Less.eval(&mut ctx, input)
}

fn more() -> Named<FuncVal> {
    let input_mode = IoMode::Any(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_mutable_fn(MORE, input_mode, output_mode, fn_more)
}

fn fn_more(mut ctx: CtxForMutableFn, input: Val) -> Val {
    More.eval(&mut ctx, input)
}
