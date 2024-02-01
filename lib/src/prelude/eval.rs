use crate::{
    ctx::NameMap,
    ctx_access::{
        free::FreeCtx,
        mutable::CtxForMutableFn,
    },
    eval::Evaluator,
    eval_mode::{
        eager::Eager,
        lazy::Lazy,
        value::Value,
        EvalMode,
    },
    io_mode::IoMode,
    prelude::{
        default_mode,
        named_free_fn,
        named_mutable_fn,
        Named,
        Prelude,
    },
    val::{
        func::FuncVal,
        Val,
    },
};

#[derive(Clone)]
pub(crate) struct EvalPrelude {
    pub(crate) value: Named<FuncVal>,
    pub(crate) lazy: Named<FuncVal>,
    pub(crate) eager: Named<FuncVal>,
}

impl Default for EvalPrelude {
    fn default() -> Self {
        EvalPrelude {
            value: value(),
            lazy: lazy(),
            eager: eager(),
        }
    }
}

impl Prelude for EvalPrelude {
    fn put(&self, m: &mut NameMap) {
        self.value.put(m);
        self.lazy.put(m);
        self.eager.put(m);
    }
}

pub(crate) const VALUE: &str = "`u";
pub(crate) const LAZY: &str = "`f";
pub(crate) const EAGER: &str = "`t";

fn value() -> Named<FuncVal> {
    let input_mode = IoMode::Eval(EvalMode::Value);
    let output_mode = default_mode();
    named_free_fn(VALUE, input_mode, output_mode, fn_value)
}

fn fn_value(input: Val) -> Val {
    Value.eval(&mut FreeCtx, input)
}

fn lazy() -> Named<FuncVal> {
    let input_mode = IoMode::Eval(EvalMode::Value);
    let output_mode = default_mode();
    named_mutable_fn(LAZY, input_mode, output_mode, fn_lazy)
}

fn fn_lazy(mut ctx: CtxForMutableFn, input: Val) -> Val {
    Lazy.eval(&mut ctx, input)
}

fn eager() -> Named<FuncVal> {
    let input_mode = IoMode::Eval(EvalMode::Value);
    let output_mode = default_mode();
    named_mutable_fn(EAGER, input_mode, output_mode, fn_eager)
}

fn fn_eager(mut ctx: CtxForMutableFn, input: Val) -> Val {
    Eager.eval(&mut ctx, input)
}
